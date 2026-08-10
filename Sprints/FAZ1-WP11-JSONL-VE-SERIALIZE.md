# Sprint FAZ1-WP11 — JSONL Oturum Kaydı ve xterm Serialize

> **Kart:** FAZ1-PLANI.md §5 WP-11 · ADR-8
> **Takvim:** Hafta 2 · Gün 10 (plan: 2026-08-21) · **Süre:** 4 sa · **Öncelik:** P1
> **Durum:** ⏳ Planlandı

## 1. Hedef

docs 12.2'deki konuşma geçmişi zorunluluğunu kapatmak: her ajan oturumunun çıktısı
`~/.agentcompany/logs/<agent>/<task>-<timestamp>.jsonl` dosyasına JSONL olarak yazılır;
terminal buffer'ı oturum sonunda `xterm-addon-serialize` ile "session_buffer" kaydı olarak
eklenir ve sekme yeniden açıldığında geri yüklenir (FAZ0'ın serialize ertelemesi kapanır).

## 2. Definition of Done (DoD)

- [ ] `pty/runtime/transcript.rs`: `TranscriptWriter` (ajan başına dizin, oturum başına dosya)
- [ ] Pompa: output (ham byte → text olarak decode edilir) + Progress + exit özeti JSONL'a yazılıyor
- [ ] `agent_write` girdileri `type:"input"` satırı olarak kaydediliyor
- [ ] `transcript_append_session_buffer(agent_id, execution_id, text)` komutu
- [ ] `PtyTerminal`: exit olayında `terminal.serialize()` → komut; sekme yeniden açılışta in-memory geri yükleme
- [ ] `chrono` bağımlılığı eklendi (RFC3339 zaman damgası) — sürüm pinli
- [ ] `.gitignore`'a `.agentcompany/` eklendi
- [ ] Testler: dosya oluşturma, JSON satır geçerliliği, path sanitize ✅

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-04 (Progress sinyalleri), WP-08 (keepLogs seçeneği — dizin silme).
- Çıkış bağımlıları: WP-13 (cost JSONL'da), M2 (transcript okuma/geri sarma).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | `chrono` ekle | `cargo add chrono` (default-features kısıtlı: `clock`) | Lockfile güncel |
| T-2 | TranscriptWriter | `open(agent_id, task_id?) -> path`; `append(entry)`; JSONL satırı tek satır | Sanitize testi |
| T-3 | Pompa bağlama | output → `{ts, type:"output", content:<utf8-lossy>}`; Progress → `{ts, type:"progress", ...}`; exit → `{ts, type:"exit", code, outputBytes}` | Çok baytlı UTF-8 satırı bozulmaz (JSON string escape) |
| T-4 | Input kaydı | `agent_write` içinde `{ts, type:"input", content}` | Girdi kaybolmaz |
| T-5 | serialize komutu | `transcript_append_session_buffer(agent_id, execution_id, text)` → `{ts, type:"session_buffer"}` | Exit akışı testli |
| T-6 | Frontend geri yükleme | `terminalStore`: `bufferByAgentId` map; exit'te serialize+invoke; mount'ta varsa `terminal.write(buffer)` | Sekme geri açılınca içerik duruyor |
| T-7 | gitignore + doküman | `.agentcompany/`; DEVELOPERS.md notu | Git temiz |

## 5. Teknik Talimatlar

### 5.1 JSONL kayıt şeması (docs 12.2)

```jsonl
{"ts":"2026-08-21T09:12:03Z","type":"output","content":"\u001b[32m> Task :compileJava\u001b[0m"}
{"ts":"2026-08-21T09:12:04Z","type":"input","content":"echo selam\n"}
{"ts":"2026-08-21T09:12:05Z","type":"progress","turn":1,"cost":0.012,"tokensIn":1200,"tokensOut":340}
{"ts":"2026-08-21T09:12:09Z","type":"exit","code":0,"outputBytes":48213}
{"ts":"2026-08-21T09:12:10Z","type":"session_buffer","content":"<serialized xterm buffer, ANSI dahil>"}
```

- content ham bayt `String::from_utf8_lossy` (JSON string escape sayesinde çok baytlı bozulmaz;
  terminal ekranı zaten Channel'da ham bayt alıyor — kayıt amacıyla text'e çevrilir).
- Dosya adı: `<task-id>-<epoch>.jsonl` veya `manual-<epoch>.jsonl` (task yoksa).
- Ajan slug: `sanitize_agent_name` (mevcut worktree yardımcısı) — path traversal önlemi.

### 5.2 Dizin yapısı

```
~/.agentcompany/logs/<agent-slug>/
└── 12-1784700000.jsonl
```

- `dirs::home_dir()` (bu sprintte `dirs` eklenir) veya `app.path().app_data_dir()` yerine
  docs 12.2'ye sadık kalınır: `~/.agentcompany/logs/`. (Tauri sandbox'ında app_data_dir
  tercih edilecekse ADR notu düşülür — karar: docs 12.2, `~/.agentcompany`.)
- `keepLogs=false` (WP-08) bu dizini siler.

### 5.3 Frontend serialize döngüsü

```ts
// exit olayında
const buffer = terminal.serialize()
await transcriptAppendSessionBuffer({ agentId, executionId, text: buffer })
useTerminalStore.getState().setBuffer(agentId, buffer)   // in-memory geri yükleme için

// PtyTerminal mount'ta
const saved = useTerminalStore.getState().buffers[agentId]
if (saved && !hasSessionOutput) terminal.write(saved)
```

- `xterm-addon-serialize` zaten bağımlılıkta (0.10.0) — `import { SerializeAddon } from 'xterm-addon-serialize'` ile bağlanır (FAZ0'da kurulmuş, bağlanmamıştı).

## 6. Test Planı

- `transcript_creates_dir_and_file`: home dizini mock edilemez → `TranscriptWriter::open_in(dir)` (test için dizin parametresi); `~` versiyonu wrapper.
- `transcript_jsonl_valid`: her satır `serde_json::from_str` geçer; UTF-8 içerik tek satırda.
- `transcript_sanitizes_agent_name`: `../../x` → `x` (veya benzeri slug).
- `input_and_output_roundtrip`: append + okuma → tipler doğru.
- Frontend: manuel — oturum bitir → JSONL'de son satır `session_buffer`; sekme kapat/aç → içerik geri geldi.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked transcript_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # manuel serialize senaryosu
ls ~/.agentcompany/logs/*/   # dosyalar oluştu
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| `~/.agentcompany` diskte büyür | Dokümantasyon + `keepLogs` (WP-08); M2'de yaşlandırma |
| `from_utf8_lossy` kayıtta kayıp | Yalnız kayıt amaçlı; ekran akışı ham bayt (korunuyor) |
| serialize buffer çok büyük | JSONL tek satır olabilir (xterm buffer ~MB); kabul edilebilir, M2'de chunk |
| Home yolu platform farkı (Win `%USERPROFILE%`) | `dirs::home_dir()` platformlar arası |

## 9. Sprint Gate

- DoD ✓; gerçek oturum sonunda `.jsonl` dosyası okunabilir; buffer persist çalışıyor.

## 10. Çıktılar

- `src-tauri/src/pty/runtime/transcript.rs` · `pty/runtime/mod.rs` (pump bağlama) · `pty/mod.rs` (`transcript_append_session_buffer`) · `src/components/PtyTerminal.tsx` · `src/store/terminal.ts` · `Cargo.toml` (+chrono, +dirs) · `.gitignore`.

## 11. Devir Notları (sonraki sprinte)

- WP-13: Progress cost JSONL'da zaten var → TopBar sayaç buradan beslenebilir.
- M2: "Time Machine" (docs 15.4) transcript okuma/geri sarma — veri hazır.
