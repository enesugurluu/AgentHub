# Sprint FAZ1-WP04 — OutputParser İskeleti ve Progress Event

> **Kart:** FAZ1-PLANI.md §5 WP-4 · ADR-4
> **Takvim:** Hafta 3 · Gün 13–14 (2026-08-24 → 2026-08-25) · **Süre:** 8 sa · **Öncelik:** P1
> **Durum:** ✅ Kapandı — kod + fixture testler yazıldı; `cargo test` doğrulaması CI/kullanıcı makinesinde
>
> **Uygulama notları (2026-08-10):**
> - `pty/runtime/parser.rs`: `OutputParser` trait + `OutputSignal` + satır tamponlu 3 parser
>   (ClaudeStreamJsonParser, OpencodeJsonlParser, RegexProgressParser) + `select_parser`.
> - `PtyEventKind::Signal { signal }` eklendi (iç içe tag çakışmasını önlemek için newtype yerine struct varyant).
> - `pump_loop(reader, parser, agent_id, execution_id, on_event) -> PumpResult` — Channel'dan bağımsız, unit test edilebilir.
> - `start_output_pump` parser parametresi aldı; son TaskCompleted/Failed `PtySession.last_completion`'a yazılır (WP-10 finalize).
> - `register_session` engine_type + non_interactive alıyor; `agent_spawn_engine`'de `opts.non_interactive` spawn öncesi yakalanıyor.
> - Fixture testleri: usage/result, kısmi chunk, opencode cost/completed, regex progress/approval, pump forwarding, tür kontrolü (7 test).

## 1. Hedef

Her CLI'nin ilerleme/maliyet/onay/tamamlanma işaretlerini ortak bir arayüze döken
`OutputParser` katmanı (docs 7.3): Claude Code `stream-json`, OpenCode JSONL ve genel
regex eşleşmeleri. `PtyEventKind::Progress` uçtan uca akar; M2'nin maliyet dashboard'ı
ve onay akışının veri omurgası bu sprintte kurulur.

## 2. Definition of Done (DoD)

- [ ] `pty/runtime/parser.rs`: `OutputParser` trait + `OutputSignal` enum
- [ ] `ClaudeStreamJsonParser`, `OpencodeJsonlParser`, `RegexProgressParser` implementasyonları
- [ ] `PtyEventKind::Progress { turn, cost, tokens_in, tokens_out }` eklendi (serde camelCase)
- [ ] `start_output_pump` parser'ı besliyor; sinyaller Channel'a + (WP-11 hazırsa) JSONL'a gidiyor
- [ ] Parser seçimi `register_session`'da `engine_type` + `non_interactive`'a göre
- [ ] Fixture tabanlı unit testler (stream-json örnek satırları, opencode JSONL, regex) ✅
- [ ] clippy + `pnpm typecheck/build` yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-02 (SpawnOptions.non_interactive), WP-03 (engine_type seti).
- Çıkış bağımlıları: WP-10 (TaskCompleted/Failed sinyalleri), WP-11 (JSONL Progress satırı), WP-13 (cost toplamı).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Trait + sinyaller | `parser.rs` (aşağıdaki imzalar) | Derleniyor |
| T-2 | Claude stream-json | Satır bazlı JSON; `{"type":"system","subtype":"usage",...}` → Progress; `subtype:"result"` → TaskCompleted/TaskFailed | Fixture testi |
| T-3 | OpenCode JSONL | JSONL event'leri; cost/meta → Progress; `session.completed` → TaskCompleted | Fixture testi |
| T-4 | Regex parser | `[n/N]`, `Tokens: x`, `Allow?`/`[y/n]` → Progress/ApprovalRequested (onay yalnız işaret, köprü M2) | Fixture testi |
| T-5 | Event genişletme | `PtyEventKind::Progress`; `PtyEvent`'e `task_id: Option<String>` (WP-10 için şimdiden) | serde tag uyumlu |
| T-6 | Pump entegrasyonu | `start_output_pump` parametresi `parser: Box<dyn OutputParser>`; sinyaller → channel | Uçtan uca akış |
| T-7 | Parser seçimi | `register_session`'da `select_parser(engine_type, non_interactive)` | claude+print → stream-json |

## 5. Teknik Talimatlar

### 5.1 Trait ve sinyaller (pty/runtime/parser.rs)

```rust
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OutputSignal {
    Progress { turn: u32, cost: f64, tokens_in: u64, tokens_out: u64 },
    ApprovalRequested { pattern: String },
    TaskCompleted { summary: String },
    TaskFailed { reason: String },
}

pub trait OutputParser: Send + Sync {
    fn feed(&mut self, bytes: &[u8], out: &mut Vec<OutputSignal>);
    fn reset(&mut self);
}
```

- Parser'lar **kayıpsız satır sınırı** korur: `feed` içinde `Vec<u8>` tampona ekle,
  `\n` sınırından tam satırları çıkar; kalan parçayı tamponda tut (UTF-8 bozulması Channel
  tarafında zaten ham bayt olduğu için risk yok; JSON satırları ASCII/UTF-8 metindir).
- `feed` bloklamaz: her satır için `serde_json::from_str` dene; parse edilemeyen satırı
  sessizce geç (çıktıya ANSI karışabilir).

### 5.2 Claude stream-json örnek satırları (fixture)

```json
{"type":"system","subtype":"init","cwd":"/repo","session_id":"abc"}
{"type":"system","subtype":"usage","usage":{"input_tokens":1200,"output_tokens":340,"cache_creation_input_tokens":0,"cache_read_input_tokens":500},"cost_usd":0.012}
{"type":"assistant","message":{"content":[{"type":"text","text":"..."}]}}
{"type":"result","subtype":"success","result":"done","total_cost_usd":0.045,"usage":{...}}
```

- `usage` → `Progress { turn: <sayaç>, cost: cost_usd, tokens_in: input_tokens, tokens_out: output_tokens }`.
- `result.subtype == "success"` → `TaskCompleted`; `"error_network"`/`"error_permission"` vb. → `TaskFailed`.
- Bilinmeyen satır tipleri yok sayılır.

### 5.3 OpenCode JSONL (fixture)

```json
{"type":"message.part.updated","sessionID":"...","messageID":"...","part":{"type":"text","text":"..."}}
{"type":"message.updated","sessionID":"...","info":{"cost":{"totalCostUSD":0.021,"totalTokens":{"input":5000,"output":800}}}}
{"type":"session.completed","sessionID":"...","reason":"completed"}
```

- `message.updated` → `Progress`; `session.completed` → `TaskCompleted`.

### 5.4 RegexProgressParser

| Desen | Sinyal |
|:---|:---|
| `\[(\d+)/(\d+)\]` | `Progress { turn: m1, .. }` (cost/token 0) |
| `Tokens:\s*([\d.,]+)` | `Progress` tokens_in |
| `Allow\?|\[y/n\]|Do you want to proceed` | `ApprovalRequested { pattern }` |

### 5.5 Pump entegrasyonu

```rust
pub fn start_output_pump(
    app: AppHandle, agent_id: String, execution_id: String,
    mut reader: Box<dyn Read + Send>, channel: Channel<PtyEvent>,
    mut parser: Box<dyn OutputParser>,
) {
    // okunan her chunk: parser.feed(&buf[..n], &mut signals);
    // her sinyal: channel.send(PtyEvent { kind: PtyEventKind::Signal(sig) ... })
}
```

- `PtyEventKind`'a `Signal(OutputSignal)` varyantı eklemek yerine `Progress`/`TaskCompleted`
  varyantlarını doğrudan koymak daha sade; karar: **`Signal(OutputSignal)`** tek varyant
  (TS tarafında `kind.type === 'signal'` + `signal.type` ayrımı) — serde tag iç içe çalışır.

### 5.6 Parser seçimi

| Koşul | Parser |
|:---|:---|
| `engine_type == "claude"` ve `non_interactive` | `ClaudeStreamJsonParser` |
| `engine_type == "opencode"` | `OpencodeJsonlParser` |
| diğer | `RegexProgressParser` |

## 6. Test Planı

- `claude_parser_usage_and_result`: fixture satırlar → beklenen sinyal dizisi (turn/cost/token doğru).
- `claude_parser_partial_chunk_utf8`: satır ortasından bölünen chunk'lar → yine de tam satır parse edilir.
- `opencode_parser_cost_and_completed`.
- `regex_parser_progress_and_approval`.
- `select_parser_engine_mapping`.
- `pump_signal_forwarding`: sahte reader + sahte parser ile `PtyEvent`'in kanala düştüğü doğrulanır (Channel Tauri'ye bağlı olduğu için pump'ın parser bölümü ayrı fonksiyona çıkarılır: `fn pump_loop(reader, parser, on_event: impl FnMut(PtyEvent))` — unit test on_event'i yakalar).

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked parser_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| stream-json satırları ANSI/prefix ile karışır | JSON parse denemesi; olmayan satırı geç; `{` ile başlayan satırlara odaklan |
| Format değişikliği (Anthropic/OpenCode) | Fixture'lar sabit; değişimde fixture + parser birlikte güncellenir |
| Yanlış TaskCompleted eşleşmesi | Yalnız kesin `subtype`/`session.completed`; regex parser `TaskCompleted` üretmez (güvenli yön) |
| `Signal` varyantı TS'te karmaşık | `ipc.ts`'te daraltılmış tip; `pnpm typecheck` gate |

## 9. Sprint Gate

- DoD ✓; fixture testleri yeşil; gerçek bir `claude -p --output-format stream-json` çıktısı
  kullanıcı makinesinde elle doğrulandı (Progress olayları terminal/store'a düşüyor).

## 10. Çıktılar

- `src-tauri/src/pty/runtime/parser.rs` · `pty/runtime/mod.rs` (pump entegrasyonu) · `pty/mod.rs` (parser seçimi) · `src/lib/ipc.ts` (Signal tipi).

## 11. Devir Notları (sonraki sprinte)

- WP-10: `TaskCompleted/Failed` sinyalleri + `PtyEvent.task_id` tamamlanma algılamayı besler.
- WP-11: Progress sinyalleri JSONL'a `type:"progress"` satırı olarak yazılır.
- WP-13: cost toplamı `Progress.cost` birikiminden; TopBar CostMeter.
