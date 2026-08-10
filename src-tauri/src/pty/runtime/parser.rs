//! CLI çıktı çözümleyicileri (AjanOfis docs Bölüm 7.3 — stream parser).
//!
//! Her CLI'nin ilerleme/maliyet/onay/tamamlanma işaretlerini ortak `OutputSignal`
//! akışına döker. FAZ1 kapsamı (ADR-4/WP-04):
//! - `ClaudeStreamJsonParser`: `claude -p --output-format stream-json` (usage → Progress,
//!   result → TaskCompleted/TaskFailed)
//! - `OpencodeJsonlParser`: opencode JSONL (`message.updated` cost → Progress,
//!   `session.completed` → TaskCompleted)
//! - `RegexProgressParser`: genel regex eşleşmeleri (`[n/N]`, `Tokens:`, onay kalıpları)
//!
//! Parser'lar **satır tamponlu**dır: çok baytlı UTF-8 / chunk sınırında bozulma olmaz.

use serde::Serialize;

/// Ortak çıktı sinyali — `PtyEventKind::Signal` olarak frontend'e akar (WP-04/10/13).
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OutputSignal {
  Progress {
    turn: u32,
    cost: f64,
    tokens_in: u64,
    tokens_out: u64,
  },
  /// Yalnızca İŞARET: tam onay köprüsü (allow/deny/edit/always) M2'de (docs 7.4).
  ApprovalRequested { pattern: String },
  TaskCompleted { summary: String },
  TaskFailed { reason: String },
}

/// Çıktı çözümleyici arayüzü — `feed` bloklamaz, tampon tutar.
pub trait OutputParser: Send + Sync {
  fn feed(&mut self, bytes: &[u8], out: &mut Vec<OutputSignal>);
  fn reset(&mut self);
}

/// Satır tamponu: parçalanmış chunk'ları birleştirir, tam satırları verir.
/// Satır sonu (`\n`) dahil edilmez; kalan kısım tamponda bekler.
#[derive(Default)]
struct LineBuffer {
  buf: Vec<u8>,
}

impl LineBuffer {
  fn push(&mut self, bytes: &[u8]) -> Vec<Vec<u8>> {
    self.buf.extend_from_slice(bytes);
    let mut lines = Vec::new();
    let mut start = 0;
    for (i, b) in self.buf.iter().enumerate() {
      if *b == b'\n' {
        lines.push(self.buf[start..i].to_vec());
        start = i + 1;
      }
    }
    self.buf.drain(..start);
    lines
  }
}

/// Claude Code `--output-format stream-json` satırlarını parse eder.
pub struct ClaudeStreamJsonParser {
  buffer: LineBuffer,
  turn: u32,
}

impl Default for ClaudeStreamJsonParser {
  fn default() -> Self {
    Self {
      buffer: LineBuffer::default(),
      turn: 0,
    }
  }
}

impl OutputParser for ClaudeStreamJsonParser {
  fn feed(&mut self, bytes: &[u8], out: &mut Vec<OutputSignal>) {
    for line in self.buffer.push(bytes) {
      let Ok(value) = serde_json::from_slice::<serde_json::Value>(&line) else {
        continue; // ANSI/prefix karışan satırlar sessizce geçilir
      };
      let Some(ty) = value.get("type").and_then(|v| v.as_str()) else {
        continue;
      };
      match ty {
        "system" => {
          let subtype = value.get("subtype").and_then(|v| v.as_str()).unwrap_or("");
          if subtype == "usage" {
            self.turn += 1;
            let usage = value.get("usage").unwrap_or(&serde_json::Value::Null);
            out.push(OutputSignal::Progress {
              turn: self.turn,
              cost: value
                .get("cost_usd")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
              tokens_in: usage
                .get("input_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
              tokens_out: usage
                .get("output_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            });
          }
        }
        "result" => {
          let subtype = value.get("subtype").and_then(|v| v.as_str()).unwrap_or("");
          let summary = value
            .get("result")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
          if subtype == "success" {
            out.push(OutputSignal::TaskCompleted { summary });
          } else {
            out.push(OutputSignal::TaskFailed {
              reason: if summary.is_empty() {
                format!("claude result subtype: {subtype}")
              } else {
                summary
              },
            });
          }
        }
        _ => {}
      }
    }
  }

  fn reset(&mut self) {
    self.buffer = LineBuffer::default();
    self.turn = 0;
  }
}

/// OpenCode JSONL event'lerini parse eder.
pub struct OpencodeJsonlParser {
  buffer: LineBuffer,
}

impl Default for OpencodeJsonlParser {
  fn default() -> Self {
    Self {
      buffer: LineBuffer::default(),
    }
  }
}

impl OutputParser for OpencodeJsonlParser {
  fn feed(&mut self, bytes: &[u8], out: &mut Vec<OutputSignal>) {
    for line in self.buffer.push(bytes) {
      let Ok(value) = serde_json::from_slice::<serde_json::Value>(&line) else {
        continue;
      };
      let Some(ty) = value.get("type").and_then(|v| v.as_str()) else {
        continue;
      };
      match ty {
        "message.updated" => {
          if let Some(info) = value.get("info") {
            if let Some(cost) = info.get("cost") {
              let total_cost = cost.get("totalCostUSD").and_then(|v| v.as_f64()).unwrap_or(0.0);
              let tokens = cost.get("totalTokens").unwrap_or(&serde_json::Value::Null);
              out.push(OutputSignal::Progress {
                turn: 0,
                cost: total_cost,
                tokens_in: tokens.get("input").and_then(|v| v.as_u64()).unwrap_or(0),
                tokens_out: tokens.get("output").and_then(|v| v.as_u64()).unwrap_or(0),
              });
            }
          }
        }
        "session.completed" => {
          let reason = value
            .get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("completed")
            .to_string();
          out.push(OutputSignal::TaskCompleted { summary: reason });
        }
        _ => {}
      }
    }
  }

  fn reset(&mut self) {
    self.buffer = LineBuffer::default();
  }
}

/// Genel regex eşleşmeleri (docs 7.3): ilerleme `[n/N]`, `Tokens:`, onay kalıpları.
pub struct RegexProgressParser {
  buffer: LineBuffer,
  last_turn: u32,
}

impl Default for RegexProgressParser {
  fn default() -> Self {
    Self {
      buffer: LineBuffer::default(),
      last_turn: 0,
    }
  }
}

impl OutputParser for RegexProgressParser {
  fn feed(&mut self, bytes: &[u8], out: &mut Vec<OutputSignal>) {
    for line in self.buffer.push(bytes) {
      let text = String::from_utf8_lossy(&line);
      let lower = text.to_ascii_lowercase();

      if let Some(cap) = text.match_indices("[").next() {
        // "[n/N]" kalıbı (örn. "[2/5]")
        if let Some(rest) = text.get(cap.0 + 1..) {
          if let Some(close) = rest.find(']') {
            let inner = &rest[..close];
            if let Some((turn, total)) = inner.split_once('/') {
              if let (Ok(t), Ok(_total)) = (turn.trim().parse::<u32>(), total.trim().parse::<u32>()) {
                if t > self.last_turn {
                  self.last_turn = t;
                  out.push(OutputSignal::Progress {
                    turn: t,
                    cost: 0.0,
                    tokens_in: 0,
                    tokens_out: 0,
                  });
                }
              }
            }
          }
        }
      }

      if let Some(prefix) = text.to_lowercase().find("tokens:") {
        let rest = &text[prefix + 7..];
        let value: String = rest
          .trim_start()
          .chars()
          .take_while(|c| c.is_ascii_digit() || *c == '.')
          .collect();
        if let Ok(tokens) = value.parse::<u64>() {
          out.push(OutputSignal::Progress {
            turn: self.last_turn,
            cost: 0.0,
            tokens_in: tokens,
            tokens_out: 0,
          });
        }
      }

      let is_approval = ["allow?", "[y/n]", "do you want to proceed", "permission required"]
        .iter()
        .any(|p| lower.contains(p));
      if is_approval {
        out.push(OutputSignal::ApprovalRequested {
          pattern: text.trim().chars().take(80).collect(),
        });
      }
    }
  }

  fn reset(&mut self) {
    self.buffer = LineBuffer::default();
    self.last_turn = 0;
  }
}

/// Motor + moda göre parser seçimi (ADR-4; WP-04).
pub fn select_parser(engine_type: &str, non_interactive: bool) -> Box<dyn OutputParser> {
  match engine_type {
    "claude" if non_interactive => Box::<ClaudeStreamJsonParser>::default(),
    "opencode" => Box::<OpencodeJsonlParser>::default(),
    _ => Box::<RegexProgressParser>::default(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn claude_parser_usage_and_result() {
    let mut parser = ClaudeStreamJsonParser::default();
    let mut signals = Vec::new();
    parser.feed(
      br#"{"type":"system","subtype":"init","session_id":"abc"}
{"type":"system","subtype":"usage","usage":{"input_tokens":1200,"output_tokens":340},"cost_usd":0.012}
{"type":"result","subtype":"success","result":"done","total_cost_usd":0.045}
"#,
      &mut signals,
    );
    assert_eq!(signals.len(), 2);
    match &signals[0] {
      OutputSignal::Progress {
        turn,
        cost,
        tokens_in,
        tokens_out,
      } => {
        assert_eq!(*turn, 1);
        assert!((*cost - 0.012).abs() < 1e-9);
        assert_eq!(*tokens_in, 1200);
        assert_eq!(*tokens_out, 340);
      }
      other => panic!("beklenen Progress, gelen: {other:?}"),
    }
    match &signals[1] {
      OutputSignal::TaskCompleted { summary } => assert_eq!(summary, "done"),
      other => panic!("beklenen TaskCompleted, gelen: {other:?}"),
    }
  }

  #[test]
  fn claude_parser_result_failure() {
    let mut parser = ClaudeStreamJsonParser::default();
    let mut signals = Vec::new();
    parser.feed(
      br#"{"type":"result","subtype":"error_network","result":"network down"}"#,
      &mut signals,
    );
    assert!(matches!(&signals[0], OutputSignal::TaskFailed { .. }));
  }

  #[test]
  fn claude_parser_partial_chunks() {
    let mut parser = ClaudeStreamJsonParser::default();
    let mut signals = Vec::new();
    // Satır ortasından ikiye bölünen chunk'lar → yine de tam satır parse edilir.
    parser.feed(b"{\"type\":\"system\",\"subtype\":\"usa", &mut signals);
    parser.feed(b"ge\",\"usage\":{\"input_tokens\":5},\"cost_usd\":0.1}\n", &mut signals);
    assert_eq!(signals.len(), 1);
    assert!(matches!(&signals[0], OutputSignal::Progress { tokens_in: 5, .. }));
  }

  #[test]
  fn opencode_parser_cost_and_completed() {
    let mut parser = OpencodeJsonlParser::default();
    let mut signals = Vec::new();
    parser.feed(
      br#"{"type":"message.updated","info":{"cost":{"totalCostUSD":0.021,"totalTokens":{"input":5000,"output":800}}}}
{"type":"session.completed","reason":"completed"}
"#,
      &mut signals,
    );
    assert_eq!(signals.len(), 2);
    match &signals[0] {
      OutputSignal::Progress { cost, tokens_in, .. } => {
        assert!((*cost - 0.021).abs() < 1e-9);
        assert_eq!(*tokens_in, 5000);
      }
      other => panic!("beklenen Progress, gelen: {other:?}"),
    }
    assert!(matches!(&signals[1], OutputSignal::TaskCompleted { .. }));
  }

  #[test]
  fn regex_parser_progress_and_approval() {
    let mut parser = RegexProgressParser::default();
    let mut signals = Vec::new();
    parser.feed(b"tur [2/5] calisiyor...\nTokens: 1234\nAllow? y/n\n", &mut signals);
    assert!(signals
      .iter()
      .any(|s| matches!(s, OutputSignal::Progress { turn: 2, .. })));
    assert!(signals
      .iter()
      .any(|s| matches!(s, OutputSignal::Progress { tokens_in: 1234, .. })));
    assert!(signals
      .iter()
      .any(|s| matches!(s, OutputSignal::ApprovalRequested { .. })));
  }

  #[test]
  fn select_parser_mapping() {
    // claude+print → stream-json; claude interaktif → regex; opencode → jsonl; diğer → regex.
    assert!(select_parser("claude", true).is::<ClaudeStreamJsonParser>());
    assert!(select_parser("claude", false).is::<RegexProgressParser>());
    assert!(select_parser("opencode", false).is::<OpencodeJsonlParser>());
    assert!(select_parser("codex", true).is::<RegexProgressParser>());
  }
}
