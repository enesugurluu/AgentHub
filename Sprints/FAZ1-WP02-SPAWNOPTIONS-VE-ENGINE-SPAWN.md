# Sprint FAZ1-WP02 — SpawnOptions Tam Hali ve agent_spawn_engine Genişletmesi

> **Kart:** FAZ1-PLANI.md §5 WP-2 · ADR-2
> **Takvim:** Hafta 1 · Gün 2 (2026-08-11) · **Süre:** 4 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — kod + golden argv testleri yazıldı; `cargo test` doğrulaması CI/kullanıcı makinesinde (WP-14 kapanışında nihai)

## 1. Hedef

`CliSpawnOptions { workdir, env, args }` yerine docs 7.1'deki **tam `SpawnOptions`**'ı
kurmak: `model, effort, max_budget_usd, max_turns, non_interactive, task_file`.
FAZ0'ın "bilinçli erteleme" listesindeki en kritik kalem budur; WP-03 (adaptörler),
WP-10 (görev protokolü) ve WP-13 (bütçe aktarımı) bu arayüze dayanır.

## 2. Definition of Done (DoD)

- [ ] `pty/adapters/mod.rs`'te `CliSpawnOptions` kaldırıldı; `SpawnOptions` (serde Deserialize, Default) geldi
- [ ] `Effort` enum'ı (`low|medium|high|xhigh|max`) + `as_str()`/`FromStr` hazır
- [ ] `spawn_cli(&self, opts: SpawnOptions, cols, rows)` — trait default'ları korunuyor
- [ ] `agent_spawn_engine` komutuna `options: SpawnOptions` parametresi eklendi; claude.rs yeni imzayla derleniyor
- [ ] `claude.rs::spawn_cli` model/effort/budget/turns/non_interactive/task_file → doğru flag'ler
- [ ] `ipc.ts`'de `SpawnOptionsIpc` tipi + `agentSpawnEngine` güncellendi; tüm çağrılar derleniyor
- [ ] Serde roundtrip + flag haritalama unit testleri ✅

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-00 (zemin), WP-01 (agent_get — isteğe bağlı doğrulama için).
- Çıkış bağımlıları: WP-03 (yeni adaptörler bu imzayı kullanır), WP-10, WP-13.

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | `SpawnOptions` + `Effort` | `pty/adapters/mod.rs`; `Effort` enum serde `rename_all="lowercase"` | Derleniyor; Default dolu |
| T-2 | Trait imza değişimi | `spawn_cli` imzası; eski adı kullanan tüm çağrılar güncellendi (`pty/mod.rs::agent_spawn_engine`, `claude.rs`) | `cargo check` temiz |
| T-3 | claude flag haritalama | `--model`, `--effort <x>`, `--max-budget-usd <f>`, `--max-turns <n>`, `-p` (non_interactive), `--output-format stream-json` (non_interactive + parser WP-04) | Golden argv testi |
| T-4 | `task_file` köprüsü | claude.rs: `task_file` varsa içeriğini okuyup prompt argümanı olarak ver (dosya yoksa hata) | Test: içerik argv'de |
| T-5 | IPC genişletme | `agent_spawn_engine(agent_id, engine_type, options, cols, rows, channel)` | Tauri arg sırası + camelCase |
| T-6 | TS tip + çağrıcılar | `ipc.ts` `SpawnOptionsIpc`; `TerminalTabs`/`PtyTerminal` boş options ile güncellenir | `pnpm typecheck` |

## 5. Teknik Talimatlar

### 5.1 Yeni tipler (pty/adapters/mod.rs)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effort { Low, Medium, High, XHigh, Max }

impl Effort {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Low => "low", Self::Medium => "medium",
                    Self::High => "high", Self::XHigh => "xhigh", Self::Max => "max" }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SpawnOptions {
    pub workdir: PathBuf,
    pub env: Vec<(String, String)>,
    pub args: Vec<String>,              // adaptörün ek flag'leri
    pub model: Option<String>,
    pub effort: Option<Effort>,
    pub max_budget_usd: Option<f64>,
    pub max_turns: Option<u32>,
    pub non_interactive: bool,
    pub task_file: Option<PathBuf>,
}
```

> `#[serde(default)]` → frontend eksik alan gönderirse sessiz default; eski çağrılar bozulmaz
> (FAZ0 roadmap §5 "geriye uyumlu default" kuralı).

### 5.2 claude.rs flag haritalama (golden beklenti)

| SpawnOptions alanı | claude flag | Not |
|:---|:---|:---|
| `model` | `--model <v>` | |
| `effort` | `--effort <as_str>` | capability "effort" zaten ilanlı |
| `max_budget_usd` | `--max-budget-usd <f>` | f64 → `to_string()` |
| `max_turns` | `--max-turns <n>` | |
| `non_interactive` | `-p --output-format stream-json` | stream-json = WP-04 parser girdisi |
| `task_file` | prompt argümanı = dosya içeriği | `std::fs::read_to_string`; yoksa `Err` |
| `args` | sırayla eklenir | WP-10 "hafif" ek flag'ler |
| `env` | `cmd.env(k,v)` | |

### 5.3 agent_spawn_engine (pty/mod.rs)

```rust
#[tauri::command]
pub fn agent_spawn_engine(
    app: AppHandle, manager: State<PtyManager>, adapters: State<EngineAdapterRegistry>,
    agent_id: String, engine_type: String, options: SpawnOptions, cols: u16, rows: u16,
    channel: Channel<PtyEvent>,
) -> Result<AgentSpawnResult, String>
```

- `options.workdir` boşsa `resolve_agent_workdir`'den doldur (WP-05'te ensure'a bağlanır).
- `register_session`'a `engine_type` bilgisi geçilir → WP-04 pump parser seçimi için.

## 6. Test Planı

- `spawn_options_serde_roundtrip`: JSON → SpawnOptions → JSON; camelCase alanlar eşleşir.
- `effort_as_str`: tüm varyantlar `low..max` döndürür.
- `claude_spawn_options_flag_mapping`: sahte `CommandBuilder` yakalayıcı yoksa, `spawn_cli`
  için golden test — adapter'ın ürettiği komut satırını doğrulayan yardımcı (test-only
  `fn build_claude_command(opts) -> (String, Vec<String>)` çıkarılır ve hem `spawn_cli`
  hem test onu çağırır).
- `task_file_reads_content`: geçici dosya yaz → argv'de içerik var; eksik dosya → Err.
- `agent_spawn_engine_default_options`: options boş gelse bile workdir doldurulur.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked spawn_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| TS↔Rust alan uyumsuzluğu (camelCase/snake) | `serde(rename_all="camelCase")` + roundtrip testi; `pnpm typecheck` |
| `CliSpawnOptions`'ı hâlâ kullanan kod derlenmez | T-2'de tüm çağrı yerleri taranır (`rg CliSpawnOptions`) |
| `task_file` çok büyükse argv taşar | FAZ1'de `AGENT_TASK.md` küçük tutulur; M2'de stdin köprüsü |
| claude `--effort` sürüm farkı | capability "effort" detect'te doğrulanır; sürüm kapısı WP-03 |

## 9. Sprint Gate

- DoD ✓; `rg "CliSpawnOptions"` sıfır sonuç; golden flag testleri yeşil; TS tipi eşleşiyor.

## 10. Çıktılar

- `src-tauri/src/pty/adapters/mod.rs` (SpawnOptions/Effort) · `src-tauri/src/agents/claude.rs` (flag haritalama) · `src-tauri/src/pty/mod.rs` (komut imzası) · `src/lib/ipc.ts`.

## 11. Devir Notları (sonraki sprinte)

- WP-03: yeni adaptörler `SpawnOptions`'ı kullanacak; `build_<engine>_command` yardımcı deseni claude'dan kopyalanır.
- WP-04: `non_interactive && engine_type == "claude"` → stream-json parser tetiklenir.
- WP-10: `task_file` + `non_interactive` görev protokolünün iki anahtarı hazır.
- WP-13: model/effort/budget/turns artık hire config'inden doldurulabilir.
