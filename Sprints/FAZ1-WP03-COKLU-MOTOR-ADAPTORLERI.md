# Sprint FAZ1-WP03 — Çoklu Motor Adaptörleri (Codex, Gemini, OpenCode, Aider) + Kurulum

> **Kart:** FAZ1-PLANI.md §5 WP-3 · ADR-3
> **Takvim:** Hafta 1 · Gün 3–5 (plan: 2026-08-12 → 2026-08-14) · **Süre:** 12–16 sa · **Öncelik:** P0
> **Durum:** ⏳ Planlandı

## 1. Hedef

Registry'yi gerçek "çoklu motor" hâline getirmek: `codex`, `gemini`, `opencode`, `aider`
adaptörleri (`claude.rs` kalıbında) + `agent_install_engine` kurulum komutu + mock-CLI
test matrisi. Bu sprint, FAZ1 Gate'inin "2+ farklı CLI motoru" koşulunun belkemiğidir.

## 2. Definition of Done (DoD)

- [ ] `agents/codex.rs`, `agents/gemini.rs`, `agents/opencode.rs`, `agents/aider.rs` eklendi (detect/health/spawn_cli/stop)
- [ ] Ortak `detect_binary_version(binary)` yardımcısı `agents/mod.rs`'e taşındı (claude.rs de kullanıyor)
- [ ] `EngineAdapterRegistry::with_builtins()` 6 adaptör kaydediyor; registry testleri güncellendi
- [ ] Her adaptörün capability listesi + `install_hint` (`DetectResult`'a eklendi) doğru
- [ ] `agent_install_engine(engine_type, cols, rows, channel)` komutu: kurulum komutunu **backend** çözüyor, PTY'de çalıştırıyor (S5: frontend program/args göndermez)
- [ ] Mock-CLI test matrisi (PATH'e sahte binary): detect/health/spawn_cli flag'leri
- [ ] `pnpm typecheck/build` + clippy yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-00, WP-02 (SpawnOptions imzası).
- Çıkış bağımlıları: WP-04 (parser seçimi), WP-07 (motor dropdown), WP-12 (kurulum UI), WP-13 (flag eşlemesi).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Ortak yardımcı | `agents/mod.rs`'te `pub(crate) fn detect_binary_version(binary: &str, args: &[&str]) -> Option<String>`; claude.rs'teki `detect_version` onu çağırır | Refactor temiz |
| T-2 | `DetectResult` + `install_hint` | `pty/adapters/mod.rs`: `DetectResult { detected, version, capabilities, install_hint }` (default None) | UI (WP-12) hint'i görür |
| T-3 | Codex adaptörü | `codex --version`; `codex exec` (non_interactive), `--model`; bütçe/turn CLI-native değil → capability ilan etme | Golden argv testi |
| T-4 | Gemini adaptörü | `gemini --version`; `gemini run` / `-p`, `--model` | Golden argv testi |
| T-5 | OpenCode adaptörü | `opencode --version`; `opencode run`, `--model`; çıktı JSONL (WP-04) | Golden argv testi |
| T-6 | Aider adaptörü | `aider --version`; `aider --message "<task>"`, `--model`, `--no-auto-commits`, `--architect` opsiyonel | Golden argv testi |
| T-7 | Registry | `with_builtins()` + 4 adaptör; `builtins_register_*` testleri güncel | 6 id |
| T-8 | `agent_install_engine` | `install_command()` trait'e default `None`; komut backend'de çözülür, `pty` adaptörüyle spawn, oturum id `install-<engine_type>`, `events` kaydı | Kurulum PTY'de akar |
| T-9 | Mock CLI matrisi | `#[cfg(unix)]` sahte binary (`sh` script) + PATH override; Windows'ta `skip` | Her adaptörün detect/health testi |

## 5. Teknik Talimatlar

### 5.1 Flag matrisi (uygulama sırasında `--help` ile doğrulanacak)

| Adaptör | `id` / `engine_type` | detect | non-interactive | model | budget/turn | install_hint |
|:---|:---|:---|:---|:---|:---|:---|
| Claude Code | `claude-code` / `claude` | `claude --version` | `-p --output-format stream-json` | `--model` | `--max-budget-usd`, `--max-turns`, `--effort` | native installer |
| Codex CLI | `codex-cli` / `codex` | `codex --version` | `codex exec` | `--model` | desteklemiyor → capability yok | `npm i -g @openai/codex` |
| Gemini CLI | `gemini-cli` / `gemini` | `gemini --version` | `gemini run -p` (veya `--print`) | `--model` | desteklemiyor → capability yok | `npm i -g @google/gemini-cli` |
| OpenCode | `opencode-adapter` / `opencode` | `opencode --version` | `opencode run` | `--model` | cost JSONL'den (WP-04) | `curl -fsSL https://opencode.ai/install \| bash` |
| Aider | `aider-adapter` / `aider` | `aider --version` | `aider --message "<task>"` | `--model` | desteklemiyor | `pip install aider-install` |

- **Bütçe/turn desteklemeyen motorlar:** capability listesinde `budget`/`turns` yok;
  `SpawnOptions.max_budget_usd` doluysa adaptör yok sayar (sessiz) ve `tracing::warn!` loglar.
- **`task_file` (claude dışı):** opencode/aider için içerik prompt argümanı olarak
  `--message "$(cat AGENT_TASK.md)"` biçiminde; codex/gemini'de argüman veya stdin köprüsü
  (uygulama sırasında `--help` çıktısına göre; gerekirse `args` ile geçici çözüm).

### 5.2 Adaptör iskeleti (her motor için aynı kalıp)

```rust
pub struct CodexAdapter;   // + Default, Clone, Copy

impl EngineAdapter for CodexAdapter {
    fn id(&self) -> &str { "codex-cli" }
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata { engine_type: "codex".into(),
            version: detect_binary_version("codex", &["--version"]),
            capabilities: vec!["print".into(), "worktree".into()] }
    }
    fn detect(&self) -> bool { detect_binary_version("codex", &["--version"]).is_some() }
    fn health(&self) -> Result<(), String> {
        detect_binary_version("codex", &["--version"])
            .map(|v| tracing::debug!("codex version: {v}"))
            .ok_or_else(|| "codex CLI bulunamadı (npm i -g @openai/codex)".to_string())
    }
    fn spawn_cli(&self, opts: SpawnOptions, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
        let (program, args) = build_codex_command(&opts);
        spawn_pty_isolated(command_from(program, args, &opts), cols, rows)
    }
    fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
        stop_child_tree(child)
    }
}
```

- Her adaptörde `build_<engine>_command(&SpawnOptions) -> (String, Vec<String>)` **test-only
  olmayan** yardımcı (WP-02'deki claude deseni); hem spawn hem golden test onu çağırır.
- Capability ilanı **detect ile çelişemez**: `--effort` desteklemeyen motor `effort` ilan etmez.

### 5.3 agent_install_engine (pty/mod.rs — S5 uyumlu)

```rust
#[tauri::command]
pub fn agent_install_engine(
    app: AppHandle, manager: State<PtyManager>, adapters: State<EngineAdapterRegistry>,
    engine_type: String, cols: u16, rows: u16, channel: Channel<PtyEvent>,
) -> Result<AgentSpawnResult, String>
```

1. `adapters.find_by_engine_type(engine_type)` → ilk adaptör; `install_command() -> Option<Vec<String>>`.
2. `None` ise `Err("bu motor için kurulum komutu tanımlı değil")`.
3. Komut backend'de kurulur (`CommandBuilder`), `engine_type="pty"` adaptörüyle spawn;
   `agent_id = "install-<engine_type>"`, `execution_id` yeni; `events`'e `install` kaydı.
4. Frontend hiçbir zaman `program/args` göndermez — FAZ0 S5 korunur.

### 5.4 Mock-CLI test matrisi (cfg(unix))

```rust
fn with_fake_binary(name: &str, script: &str, test: impl FnOnce()) {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join(name);
    std::fs::write(&bin, format!("#!/bin/sh\n{script}\n")).unwrap();
    // chmod +x
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).unwrap();
    let prev = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("{}:{}", dir.path().display(), prev));
    test();
    std::env::set_var("PATH", prev);
}
```

- Windows: gerçek CLI yoksa `#[cfg(not(unix))]` testlerde `eprintln!("skip")` + erken dönüş.
- `cargo test` çalıştırılırken env değişimi testler arası yarış yaratmasın → `serial_test`
  eklemek yerine her test kendi temp PATH'ini kurar; gerekiyorsa tek test fonksiyonunda topla.

## 6. Test Planı

- Her adaptör: `detect_with_fake_binary` (script `echo 1.2.3` → version `1.2.3`).
- `build_*_command` golden argv: model/effort/budget/turns/non_interactive/task_file kombinasyonları.
- `registry_builtins_include_all`: 6 id içeriyor.
- `install_command_hints`: her adaptörün hint'i `Some`.
- `agent_install_engine_unknown`: bilinmeyen engine → hata.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
# Manuel (kullanıcı makinesi): Settings → Motorlar → codex "Kur" → terminal sekmesinde kurulum akar
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| CLI flag sürüm kayması | `--help` doğrulaması uygulama anında; golden testler sürüm bağımsız (mock); min. sürüm kapısı detect'te |
| Gerçek CLI'lar CI'da yok | Mock matrisi unit; entegrasyon kullanıcı makinesinde elle (WP-14 senaryo listesi) |
| `install_command` güvenliği (curl|bash) | Yalnızca adaptörün tanımlı komutu; kullanıcı onayı UI'da (WP-12); frontend program gönderemez |
| Windows'ta mock binary yok | cfg(unix) matrisi + Windows'ta gerçek CLI varlığına göre `skip` |
| OpenCode JSONL formatı değişirse | WP-04 parser fixture'ları sabitlenir; format değişince fixture güncellenir |

## 9. Sprint Gate

- DoD ✓; `with_builtins()` 6 adaptör; her adaptörün detect/health/golden-argv testi yeşil;
  `agent_install_engine` bilinen engine'de PTY oturumu açıyor (manuel doğrulama).

## 10. Çıktılar

- `src-tauri/src/agents/{codex,gemini,opencode,aider}.rs` · `agents/mod.rs` (ortak yardımcı) · `pty/adapters/mod.rs` (DetectResult.install_hint) · `pty/registry/engine_adapter_registry.rs` · `pty/mod.rs` (`agent_install_engine`).

## 11. Devir Notları (sonraki sprinte)

- WP-04: `engine_type ∈ {claude, opencode}` ve `non_interactive` → parser seçimi.
- WP-07: motor dropdown `pty_list_engine_adapters('detected')` + kurulmamışlar hint'li.
- WP-12: `install_hint` + `agent_install_engine` UI'a bağlanır.
- WP-13: budget/turn desteklemeyen motorların capability filtresi burada netleşti.
