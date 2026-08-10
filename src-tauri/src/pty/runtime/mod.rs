//! PTY çıktı pompası ve yaşam döngüsü monitörü.
//!
//! - Çıktı, per-session `Channel<PtyEvent>` üzerinden **ham bayt** olarak akar
//!   (UTF-8 çok baytlı karakterlerin chunk sınırında bozulmasını önler;
//!   xterm `Uint8Array`'i doğrudan işler).
//! - Çıktı aynı anda `OutputParser`'a beslenir (WP-04): `Signal` olayları
//!   (Progress / ApprovalRequested / TaskCompleted / TaskFailed) kanala akar.
//! - Çıkış, `Exit { code }` olayı ile frontend'e bildirilir ve `events`
//!   tablosuna yazılır.

use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::ipc::Channel;
use tauri::{AppHandle, Manager, State};

use crate::db::AppDb;
use crate::pty::registry::PtyManager;
use crate::pty::runtime::parser::{OutputParser, OutputSignal};
use crate::pty::runtime::transcript::{append_transcript_entry, epoch_seconds, output_entry};

pub mod parser;
pub mod transcript;

/// Frontend'e giden PTY olayı (serde tag'i: `kind.type = "output" | "exit" | "signal"`).
///
/// NOT: Channel payload'ları serde ile birebir serialize edilir; Tauri alan
/// adlarını dönüştürmez. Frontend `agentId`/`executionId` beklediği için
/// camelCase rename zorunlu — aksi halde tüm olaylar sessizce düşer.
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PtyEvent {
  pub agent_id: String,
  pub execution_id: String,
  /// Bağlı görev (WP-10) — frontend terminal sekmelerinde kullanılmaz; events için.
  #[serde(default)]
  pub task_id: Option<i64>,
  pub kind: PtyEventKind,
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PtyEventKind {
  Output { data: Vec<u8> },
  Exit { code: u32 },
  /// Parser sinyali (WP-04): `{ type: "signal", signal: { type: "progress", ... } }`
  Signal { signal: OutputSignal },
}

/// `pump_loop` sonucu — telemetri + WP-10 tamamlanma algılaması için son sinyal.
pub struct PumpResult {
  pub output_bytes: u64,
  pub last_completion: Option<OutputSignal>,
}

/// Okuyucudan gelen ham baytları parser'a besleyip `on_event`'e olay olarak
/// iletir. Tauri Channel'ına bağımlı DEĞİLDİR — unit test `on_event`'i yakalar.
pub fn pump_loop(
  mut reader: Box<dyn Read + Send>,
  mut parser: Box<dyn OutputParser>,
  agent_id: &str,
  execution_id: &str,
  task_id: Option<i64>,
  mut on_event: impl FnMut(PtyEvent),
) -> PumpResult {
  let mut buf = [0u8; 8192];
  let mut output_bytes: u64 = 0;
  let mut last_completion: Option<OutputSignal> = None;

  loop {
    match reader.read(&mut buf) {
      Ok(0) => break,
      Ok(n) => {
        output_bytes += n as u64;

        let mut signals = Vec::new();
        parser.feed(&buf[..n], &mut signals);
        for sig in signals {
          if matches!(
            sig,
            OutputSignal::TaskCompleted { .. } | OutputSignal::TaskFailed { .. }
          ) {
            last_completion = Some(sig.clone());
          }
          on_event(PtyEvent {
            agent_id: agent_id.to_string(),
            execution_id: execution_id.to_string(),
            task_id,
            kind: PtyEventKind::Signal { signal: sig },
          });
        }

        on_event(PtyEvent {
          agent_id: agent_id.to_string(),
          execution_id: execution_id.to_string(),
          task_id,
          kind: PtyEventKind::Output {
            data: buf[..n].to_vec(),
          },
        });
      }
      Err(_) => break,
    }
  }

  PumpResult {
    output_bytes,
    last_completion,
  }
}

/// Oturum başına çıktı pompası + yaşam döngüsü monitörü.
///
/// `parser` motor/moda göre `select_parser` ile seçilir (WP-04); son
/// TaskCompleted/Failed sinyali oturumda saklanır (WP-10 finalize).
/// `transcript_path` doluysa output/progress/exit satırları JSONL'a yazılır (WP-11).
pub fn start_output_pump(
  app: AppHandle,
  agent_id: String,
  execution_id: String,
  reader: Box<dyn Read + Send>,
  channel: Channel<PtyEvent>,
  parser: Box<dyn OutputParser>,
  transcript_path: Option<PathBuf>,
  task_id: Option<i64>,
  worktree_path: Option<PathBuf>,
) {
  // Oturum telemetrisi: pompa bayt sayar, exit olayında events tablosuna yazılır
  // (FAZ0 kabul kriteri 4 — chunk başına DB kaydı yerine kümülatif sayaç).
  let output_bytes = Arc::new(AtomicU64::new(0));
  // Toplam maliyet (WP-13): Progress.cost birikimi — exit payload + JSONL'a yazılır.
  let total_cost = Arc::new(std::sync::Mutex::new(0.0f64));

  // -- Çıktı pompası: reader → parser + Channel + JSONL --------------------------
  {
    let app = app.clone();
    let agent_id = agent_id.clone();
    let execution_id = execution_id.clone();
    let channel = channel.clone();
    let output_bytes = output_bytes.clone();
    let total_cost = total_cost.clone();
    let transcript_path = transcript_path.clone();

    thread::spawn(move || {
      let result = pump_loop(reader, parser, &agent_id, &execution_id, task_id, |event| {
        if matches!(event.kind, PtyEventKind::Output { .. }) {
          // Bayt sayacı exit telemetrisinde kullanılır.
          if let PtyEventKind::Output { ref data } = event.kind {
            output_bytes.fetch_add(data.len() as u64, Ordering::Relaxed);
            // JSONL output satırı (docs 12.2; WP-11).
            if let Some(path) = &transcript_path {
              let _ = append_transcript_entry(path, output_entry(data));
            }
          }
        }
        if let PtyEventKind::Signal { ref signal } = event.kind {
          // Maliyet birikimi (WP-13).
          if let OutputSignal::Progress { cost, .. } = signal {
            *total_cost.lock().unwrap() += cost;
          }
          // JSONL progress satırı (WP-11/13).
          if let Some(path) = &transcript_path {
            if let OutputSignal::Progress {
              turn,
              cost,
              tokens_in,
              tokens_out,
            } = signal
            {
              let _ = append_transcript_entry(
                path,
                serde_json::json!({
                  "ts": epoch_seconds(),
                  "type": "progress",
                  "turn": turn,
                  "cost": cost,
                  "tokensIn": tokens_in,
                  "tokensOut": tokens_out,
                }),
              );
            }
          }
        }
        if channel.send(event).is_err() {
          // Frontend kanalı kapandıysa (pencere/sekme kapatıldı) dur.
          return;
        }
      });

      // WP-10 tamamlanma algılaması: son TaskCompleted/Failed sinyalini oturuma yaz.
      if let Some(sig) = result.last_completion {
        let state: State<PtyManager> = app.state();
        if let Ok(mut sessions) = state.sessions.lock() {
          if let Some(session) = sessions.get_mut(&agent_id) {
            if session.execution_id == execution_id {
              *session.last_completion.lock().unwrap() = Some(sig);
            }
          }
        }
      }
    });
  }

  // -- Yaşam döngüsü monitörü: try_wait → Exit olayı + DB kaydı ------------------
  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_millis(250));

      let state: State<PtyManager> = app.state();

      let mut remove = false;
      let mut exit_code: u32 = 0;
      let mut last_completion: Option<OutputSignal> = None;

      if let Ok(mut sessions) = state.sessions.lock() {
        if let Some(session) = sessions.get_mut(&agent_id) {
          if session.execution_id != execution_id {
            // Bu agent ID'sini farklı bir execution devralmış; monitor artık geçersiz.
            break;
          }
          // Mutable erişimle child durumunu kontrol et.
          if let Ok(Some(status)) = session.child.try_wait() {
            exit_code = status.exit_code();
            remove = true;
            // Parser'ın son sinyali (WP-04) — removal öncesi yakala.
            last_completion = session.last_completion.lock().unwrap().clone();
          }
        } else {
          // Oturum zaten kaldırılmış (agent_stop tarafından).
          break;
        }

        if remove {
          sessions.remove(&agent_id);
        }
      }

      if remove {
        // WP-10: görev tamamlanma algılama (docs 13.2) — dosya > parser > exit kodu.
        if let Some(tid) = task_id {
          let (column, reason) = crate::tasks::decide_completion(
            worktree_path.as_deref(),
            last_completion.as_ref(),
            exit_code,
          );
          if let Some(db) = app.try_state::<AppDb>() {
            let _ = db.finalize_task(tid, &column, 0.0, 0, 0);
            let _ = db.record_event(
              Some(&agent_id),
              Some(tid),
              if column == "review" {
                "task_completed"
              } else {
                "task_failed"
              },
              Some(
                &serde_json::json!({ "reason": reason, "code": exit_code }).to_string(),
              ),
            );
          }
        }

        let event = PtyEvent {
          agent_id: agent_id.clone(),
          execution_id: execution_id.clone(),
          kind: PtyEventKind::Exit { code: exit_code },
        };
        let _ = channel.send(event);

        // JSONL exit satırı (docs 12.2; WP-11/13 — totalCostUsd dahil).
        let session_cost = *total_cost.lock().unwrap();
        if let Some(path) = &transcript_path {
          let _ = append_transcript_entry(
            path,
            serde_json::json!({
              "ts": epoch_seconds(),
              "type": "exit",
              "code": exit_code,
              "outputBytes": output_bytes.load(Ordering::Relaxed),
              "totalCostUsd": session_cost,
            }),
          );
        }

        if let Some(db) = app.try_state::<AppDb>() {
          let total_output = output_bytes.load(Ordering::Relaxed);
          let payload = serde_json::json!({
            "executionId": execution_id,
            "code": exit_code,
            "outputBytes": total_output,
            "totalCostUsd": session_cost,
          });
          let _ = db.record_event(
            Some(&agent_id),
            None,
            "exit",
            Some(&payload.to_string()),
          );
        }
        break;
      }
    }
  });
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pty::runtime::parser::{ClaudeStreamJsonParser, RegexProgressParser};

  struct FakeReader {
    chunks: Vec<Vec<u8>>,
  }

  impl Read for FakeReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
      if self.chunks.is_empty() {
        return Ok(0);
      }
      let chunk = self.chunks.remove(0);
      let n = chunk.len().min(buf.len());
      buf[..n].copy_from_slice(&chunk[..n]);
      if n < chunk.len() {
        self.chunks.insert(0, chunk[n..].to_vec());
      }
      Ok(n)
    }
  }

  #[test]
  fn pump_loop_forwards_output_and_progress() {
    let reader = Box::new(FakeReader {
      chunks: vec![
        b"selam\n".to_vec(),
        b"{\"type\":\"system\",\"subtype\":\"usage\",\"usage\":{\"input_tokens\":3},\"cost_usd\":0.01}\n".to_vec(),
      ],
    });
    let mut events: Vec<PtyEvent> = Vec::new();
    let result = pump_loop(
      reader,
      Box::<ClaudeStreamJsonParser>::default(),
      "1",
      "exec-1",
      |e| events.push(e),
    );

    assert_eq!(result.output_bytes, 6 + 90); // iki chunk
    assert!(result.last_completion.is_none());
    // output + progress sinyali
    assert!(events
      .iter()
      .any(|e| matches!(e.kind, PtyEventKind::Output { .. })));
    assert!(events.iter().any(|e| matches!(
      &e.kind,
      PtyEventKind::Signal { signal: OutputSignal::Progress { .. } }
    )));
  }

  #[test]
  fn pump_loop_captures_last_completion() {
    let reader = Box::new(FakeReader {
      chunks: vec![
        b"{\"type\":\"result\",\"subtype\":\"success\",\"result\":\"bitti\"}\n".to_vec(),
      ],
    });
    let mut last = None;
    let result = pump_loop(
      reader,
      Box::<ClaudeStreamJsonParser>::default(),
      "2",
      "exec-2",
      |e| last = Some(e),
    );
    assert!(result.last_completion.is_some());
    assert!(last.is_some());
  }

  #[test]
  fn pump_loop_works_with_regex_parser() {
    let reader = Box::new(FakeReader {
      chunks: vec![b"[1/3] calisiyor\n".to_vec()],
    });
    let mut signals = Vec::new();
    let _ = pump_loop(reader, Box::<RegexProgressParser>::default(), "3", "exec-3", |e| {
      if let PtyEventKind::Signal { signal } = e.kind {
        signals.push(signal);
      }
    });
    assert!(signals
      .iter()
      .any(|s| matches!(s, OutputSignal::Progress { turn: 1, .. })));
  }
}
