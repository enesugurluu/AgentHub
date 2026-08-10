# AgentHub — FAZ1 Uygulama Planı (M1: Çoklu Motor, Worktree ve İşe Alım)

**Tarih:** 2026-08-10
**Kapsam:** `FAZ0-DURUM-ANALIZI-VE-UYGULAMA-PLANI.md` §9–12 + `Docs/ajanofis-buyuk-parcalar/` (04, 05, 06, 07, 10, 12, 13, 16, 17) + `Docs/claude-code-rapor-bolumleri/` (10, 11, 19)
**Yöntem:** FAZ0 raporu baştan sona okundu; AjanOfis M1 iş kalemleri ve FAZ0'un "bilinçli ertelenen" listesi eşleştirildi; mevcut kod tabanı dosya dosya doğrulandı (backend + frontend + CI); plan, repo konvansiyonlarına (Türkçe yorum, İngilçe commit) ve FAZ0 raporunun ADR/karar stiline uygun yazıldı.

> **Adlandırma notu (önemli):** AjanOfis yol haritasında (Docs 17) "Faz 1: İskelet (M0)" = AgentHub'ın **FAZ0'ı**dır (tamamlandı). Bu belgedeki **FAZ1**, AjanOfis'in **"Faz 2: Çoklu Motor ve Worktree (3 Hafta) — M1"** milestone'ıdır; README ve FAZ0 raporu §9'daki "Faz 2 (M1)" ile birebir aynı kapsamdadır. Adlandırma kullanıcı tercihiyle "Faz 1" olarak sadeleştirilmiştir; commit mesajları ve dal isimlerinde karışıklık olmaması için milestone etiketi **M1** kullanılacaktır.

---

## 1. Yönetici Özeti

| Başlık | Durum / Hedef |
|:---|:---|
| FAZ0 gate'i (tek Claude ajanı terminalde) | ✅ Geçildi (kod doğrulaması + CI yeşil) |
| **FAZ1 hedefi (Docs 17 M1)** | 🎯 Kullanıcı **birden fazla CLI ajanı işe alabilir**, her biri **kendi worktree'sinde görev alabilir** |
| Çoklu motor | `claude` ✅ var → `codex`, `gemini`, `opencode`, `aider` eklenecek |
| Worktree | ✅ altyapı var → **hire/task zamanı otomatik oluşturma + `.env.local` + port offset** eklenecek |
| İşe alım/çıkarma | ❌ yok → **Hire Wizard (3 adım) + Fire onay akışı** |
| Görev protokolü | ❌ yok → `AGENT_TASK.md` + tamamlanma algılama + `tasks` satırı güncelleme |
| Ofis katı | 🟡 statik masa kartları → **SVG AgentDesk + StatusBadge + zoom/pan** |
| Tahmini süre | **3 hafta** (Docs 17 M1; tek geliştirici) — WP bazlı ~90–110 saat |

**Özet:** FAZ0, AjanOfis M0'ın backend'ini (PTY + adaptör trait + registry + worktree + SQLite) ve frontend iskeletini tamamlamıştı. FAZ1'in işi üç eksende toplanıyor: **(a)** adaptör katmanını gerçek çoklu-motor hâline getirmek (`SpawnOptions` tam hali + 4 yeni CLI adaptörü), **(b)** ajan yaşam döngüsünü veri modeline bağlamak (hire/fire + worktree + görev protokolü), **(c)** ofis katını etkileşimli hâle getirmek (SVG masa + durum rozetleri + repo seçici). FAZ0'ın "bilinçli ertelenen" listesindeki 5 kalemin 4'ü bu fazda kapanıyor.

---

## 2. FAZ0'dan Devralınan Durum (Kod Doğrulamalı)

### 2.1 FAZ0'da tamamlanan ve FAZ1'in üzerine inşa edeceği katmanlar

| Katman | Doğrulanan dosya(lar) | FAZ1'de ne olacak |
|:---|:---|:---|
| Tauri 2 + React 19 + Vite + TS iskeleti | `src-tauri/src/lib.rs`, `main.rs` | — (korunur) |
| PTY motoru (portable-pty) + Windows Job Objects / Unix process-group | `pty/adapters/portable_pty_native.rs`, `pty/adapters/mod.rs` (`spawn_pty_isolated`, `stop_child_tree`) | — (korunur) |
| `EngineAdapter` trait + registry (`detect/health/spawn/spawn_cli/resize/stop`) | `pty/adapters/mod.rs`, `pty/registry/engine_adapter_registry.rs` | `CliSpawnOptions` → **tam `SpawnOptions`**; 4 yeni adaptör `with_builtins()`'e |
| Per-session `Channel<PtyEvent>` (ham bayt + Exit) | `pty/runtime/mod.rs` | + **JSONL oturum kaydı** (docs 12.2), + **Progress/Cost event iskeleti** (docs 7.3) |
| Git worktree yöneticisi (`.git/agenthub-worktrees`, `.agenthub.json`, path-traversal korumalı) | `worktree.rs` | hire/task zamanı otomatik oluşturma, `.env.local` + port offset, "commit'le sakla" silme seçeneği |
| SQLite + WAL (`agents/tasks/events/settings`) | `db.rs` | `PRAGMA user_version` migration runner; hire/fire/settings komutları; starter company seed |
| Claude Code adaptörü (detect/health/spawn_cli) | `agents/claude.rs` | `SpawnOptions` uyumu + `stream-json` cost/usage parse (light) |
| Tailwind 4 + shadcn/ui + Zustand + WebGL xterm + Biome + husky + CI (3 OS) | `src/`, `package.json`, `.github/workflows/ci.yml` | yeni shadcn bileşenleri (select, alert-dialog, switch, textarea, tooltip, dropdown-menu, progress); CI'da FAZ1 test matrisi |

### 2.2 FAZ0 §12 "Bilinçli ertelemeler" → FAZ1 eşlemesi

| FAZ0'da ertelenen | FAZ1 planındaki karşılığı |
|:---|:---|
| `SpawnOptions`'ın tam hali (budget/turns/model/non-interactive) — şu an `CliSpawnOptions { workdir, env, args }` | **WP-2** (ADR-2): docs 7.1'deki şekille tam `SpawnOptions`; geriye uyumlu default'lar |
| `xterm-addon-serialize` bağımlılığı duruyor ama bağlı değil (sekme buffer persist) | **WP-9**: serialize + JSONL transcript'e bağlama (docs 12.2) |
| Gerçek `claude doctor` parse'ı (health hızlı sürüm kapısı) | **WP-3/WP-10**: her adaptörde sürüm kapısı + dokümantasyon; tam `doctor` parse'ı M2'ye bırakılır (kasıtlı: interaktif komut, her health çağrısında pahalı) |
| Dialog tabanlı repo seçici UI (`AGENTHUB_REPO_PATH` env köprüsü mevcut) | **WP-7** (ADR-7): `tauri-plugin-dialog` klasör seçici + `settings` tablosunda kalıcılık |

### 2.3 FAZ1 başlangıcında bilinen teknik kısıtlar

- **Rust toolchain bu sandbox'ta yok** → FAZ1'de her WP'nin Rust tarafı `cargo check/clippy/test` ile kullanıcı makinesinde/CI'da doğrulanır (FAZ0 raporu §10 ile aynı kural).
- `Cargo.toml`'da `tokio`, `chrono`, `dirs`, `thiserror` **yok** (FAZ0'da yalnız `tracing`, `tracing-subscriber`, `sysinfo` eklendi). FAZ1 ekler: `dirs` (log yolu), `thiserror` (opsiyonel, hata tipleri). `tokio` gerekmiyor — pump thread tabanlı.
- DB tek `Mutex<Connection>`; FAZ1'de olay yazma sıklığı düşük (exit'te 1 kayıt) → yeterli; havuz M2'de değerlendirilecek.
- `PtyTerminal` motor seçici şu an sabit `'pty' | 'claude'`; `TerminalTabs` engine'i `session.engineType === 'claude'` ile kestiriyor → FAZ1'de ajanın DB `motor` alanından türetilecek.

---

## 3. FAZ1 Kapsamı

### 3.1 Kapsamda (In Scope)

1. **Çoklu motor adaptörleri:** Codex CLI, Gemini CLI, OpenCode, Aider (+ mevcut Claude Code uyumu).
2. **SpawnOptions tam hali:** model, effort, budget, max_turns, non_interactive, task_file.
3. **Ajan yaşam döngüsü:** Hire Wizard (3 adım), Fire onay akışı, ajan DB kaydı, durum makinesi.
4. **Worktree otomasyonu:** hire/görev zamanı otomatik oluşturma, `.env.local` + port offset, silme seçenekleri, worktree'de spawn (repo köküne düşme davranışı kalkar).
5. **Görev protokolü (hafif):** "Görev Ver" → `AGENT_TASK.md` → non-interactive spawn → tamamlanma/hatada `tasks` güncelleme. (Kanban UI M2.)
6. **Repo seçici:** dialog ile proje yolu seçimi + `settings` kalıcılığı.
7. **Ofis katı v1:** SVG masa (`AgentDesk`), durum rozeti, zoom/pan, seçim → Inspector bağlantısı.
8. **Sağlık paneli:** her adaptör için detect/health/sürüm/kurulum ipucu; "Kur" akışı (kullanıcı onaylı, PTY'de).
9. **Oturum kaydı:** JSONL transcript + `xterm-addon-serialize` bağlama.
10. **Test, CI, dokümantasyon:** mock-CLI unit testleri, 3-OS CI, `DEVELOPERS.md`/`README.md` güncelleme, kapanış denetimi.

### 3.2 Kapsam dışı (bilinçli M2+ erteleme)

- Kanban panosu (dnd-kit, swimlane, WIP) — **M2**
- CEO orkestratör (görev bölme, dağıtım, handoff) — **M2**
- Onay akışı (allow/deny/edit/always) + policy engine — **M2** (FAZ1 yalnızca "onay bekleniyor" durumunu algılar)
- Maliyet/token dashboard + gerçek sayaç — **M2** (FAZ1 CLI-native `--max-budget-usd/--max-turns` bayraklarını iletir)
- A2A / toplantı / Memory Keeper / bilgi grafı — **M3/M4**
- MCP Hub (`rmcp`) — **M5**
- Tam `claude doctor` interaktif parse — M2'ye; hızlı sürüm kapısı yeterli (FAZ0 kararı)

### 3.3 FAZ1 Gate (Docs 17 M1 tanımı)

> **"Kullanıcı birden fazla CLI ajanı işe alabilir, her biri kendi worktree'sinde görev alabilir."**

Genişletilmiş kabul listesi §7'dedir.

---

## 4. Mimari Kararlar (ADR'ler)

Her karar: sorun → karar → gerekçe → maliyet/risk. Kararlar FAZ0 raporunun "YAPILMALI/DEĞİŞMELİ/DURMALI" çizgisiyle uyumludur.

### ADR-1 — Ajan yaşam döngüsü veri modeli (hire/fire/status)

**Sorun:** `agents` tablosu şema olarak docs 12.1'i karşılıyor ama yalnızca seed + listeleme kullanılıyor; `config_json`, `hired_at`, `fired_at`, `worktree_path` hiç yazılmıyor. Ajan "işe alınabilir" değil.

**Karar:**
- Yeni komutlar: `agent_hire(payload: HirePayload) -> AgentRecord`, `agent_fire(id, FireOptions) -> AgentRecord` (pasifleştirir; kalıcı silme ayrı `agent_delete`), `agent_update(id, patch)`, `agent_get(id)`.
- `HirePayload`: `name, role, motor, model?, effort?, max_budget_usd?, max_turns?, permissions_profile (full|standard|limited|custom), system_prompt?, avatar_color, skills[], mcp_servers[]` → `config_json`'a JSON olarak serileşir.
- Durum makinesi (docs 5.4 durumlarıyla birebir): `hired → idle → starting → running | thinking | blocked(waiting_approval) → done | error → fired`.
- Seed kaldırılır; yerine **migration 2** "starter company" (docs 4.2 alt kümesi: Backend, Frontend, QA) yalnızca boş tabloya ve ilk kurulumda yazar; kullanıcı hepsini işten çıkarabilir. Ofis boşsa "İşe Al" CTA'sı gösterilir.

**Gerekçe:** docs 6.1/6.4'ün tamamı DB'de gerçek ajan kaydına dayanır; FAZ0'ın S9 ("sabit kimliklerle oturum açma") kararı ancak böyle kapanır.

### ADR-2 — `CliSpawnOptions` → tam `SpawnOptions` (docs 7.1)

**Sorun:** Ajanlara model/effort/bütçe/turn limiti iletilemiyor; hire sihirbazında seçilen değerler spawn'a taşınamaz.

**Karar:** `pty/adapters/mod.rs`'de:
```rust
#[derive(Debug, Clone, Default)]
pub struct SpawnOptions {
    pub workdir: PathBuf,
    pub env: Vec<(String, String)>,
    pub args: Vec<String>,                    // adaptörün ek flag'leri (ör. "-p")
    pub model: Option<String>,
    pub effort: Option<Effort>,               // low|medium|high|xhigh|max
    pub max_budget_usd: Option<f64>,
    pub max_turns: Option<u32>,
    pub non_interactive: bool,                // claude -p / codex exec / aider --message
    pub task_file: Option<PathBuf>,           // AGENT_TASK.md
}
```
- `spawn_cli(&self, opts: SpawnOptions, cols, rows)` — mevcut trait default'ları korunur; `CliSpawnOptions` adı kaldırılır, tek çağrı yeri (`pty/mod.rs::agent_spawn_engine`) ve `claude.rs` güncellenir (derleme hatası erken yakalar).
- `agent_spawn_engine` imzasına `options: SpawnOptionsIpc` eklenir (TS tarafında aynı camelCase tip).
- Her adaptör, desteklediği alanları `capabilities` listesinde ilan eder; desteklenmeyen alan sessizce yok sayılır (ör. aider `effort` desteklemiyorsa flag üretmez).

**Gerekçe:** FAZ0 §12 ertelemesini kapatır; roadmap §5 "default implementasyonlarla geriye uyum" kuralı korunur. **Risk:** TS↔Rust alan adı eşleşmesi — `serde(rename_all = "camelCase")` + tip testi.

### ADR-3 — Yeni motor adaptörleri (codex, gemini, opencode, aider)

**Sorun:** Registry'de yalnız `portable-pty-native` + `claude-code` var; "çoklu motor" hedefi karşılanmıyor.

**Karar:** `src-tauri/src/agents/` altında `codex.rs`, `gemini.rs`, `opencode.rs`, `aider.rs` — hepsi `claude.rs` kalıbını izler (detect → `--version`; health → sürüm + capability; `spawn_cli` → kendi komutunu kurar; `stop` → `stop_child_tree`). Registry `with_builtins()`'e eklenir. **Flag matrisi** (uygulama sırasında `--help` ile doğrulanacak; fixture-testlerle sabitlenir):

| Adaptör | `id` / `engine_type` | detect (version) | non-interactive spawn | budget/turn flag'leri | install_hint |
|:---|:---|:---|:---|:---|:---|
| Claude Code | `claude-code` / `claude` | `claude --version` | `-p --output-format stream-json` | `--max-budget-usd`, `--max-turns`, `--effort`, `--model` | native installer |
| Codex CLI | `codex-cli` / `codex` | `codex --version` | `codex exec` (yapılandırma: approval-mode) | `--model`; bütçe CLI-native değil → env/uyarı | `npm i -g @openai/codex` |
| Gemini CLI | `gemini-cli` / `gemini` | `gemini --version` | `gemini run` / `-p` | `--model`; bütçe CLI-native değil | `npm i -g @google/gemini-cli` |
| OpenCode | `opencode-adapter` / `opencode` | `opencode --version` | `opencode run` (JSONL event) | `--model`; cost event'i çıktıdan parse edilir | `curl -fsSL https://opencode.ai/install | bash` |
| Aider | `aider-adapter` / `aider` | `aider --version` | `aider --message "<task>"` | `--model`, `--no-auto-commits`, `--architect` | `pip install aider-install` |

- `Capabilities` (docs 7.1): `supports_worktree / supports_budget / supports_effort / supports_json_output / supports_non_interactive` → mevcut `capabilities: Vec<String>` sözlüğüne işlenir; `install_hint` `DetectResult`'a eklenir.
- **Kurulum akışı (docs 7.5):** yeni `agent_install_engine(engine_type)` komutu; kurulum komutunu kullanıcı onayıyla **ayrı bir PTY oturumunda** çalıştırır (frontend'den keyfi program spawn edilemez — FAZ0 S5 korunur).

**Gerekçe:** AjanOfis'in çekirdek farklılaştırıcısı (11+ motor); docs 7.6 seçim kriterleri uygulanır. **Risk:** CLI flag kayması → §8 risk tablosu.

### ADR-4 — Çıktı çözümleyici iskeleti (docs 7.3)

**Sorun:** Her CLI'nin ilerleme/maliyet/onay/tamamlanma işaretleri farklı; ham bayt akışı tek başına "görev bitti" diyemez.

**Karar:** `pty/runtime/` altında `OutputParser` trait'i:
```rust
pub trait OutputParser: Send + Sync {
    fn feed(&mut self, bytes: &[u8], out: &mut Vec<OutputSignal>);
    fn reset(&mut self);
}
pub enum OutputSignal {
    Progress { turn: u32, cost: f64, tokens_in: u64, tokens_out: u64 },
    ApprovalRequested { pattern: String },
    TaskCompleted { summary: String },
    TaskFailed { reason: String },
}
```
- FAZ1 kapsamı: **Claude Code** için `stream-json` `system.usage`/`result` parse; **OpenCode** için JSONL `cost`/`session.completed`; diğerleri için regex tabanlı ilerleme (`[n/N]`, `Tokens:`); **onay** algılamada yalnız `ApprovalRequested` → ajan durumu `blocked` (yanıt köprüsü M2).
- `PtyEventKind`'a `Progress` varyantı eklenir; pompa parser'ı besler, sinyaller hem Channel'a hem JSONL'a gider.

**Gerekçe:** docs 7.3 tablosu; M2 onay akışının ön koşulu. **Sınır:** parser'lar fixture tabanlı test edilir (gerçek CLI'lar CI'da yok).

### ADR-5 — Worktree: hire/görev zamanı otomasyon + runtime izolasyonu (docs 10)

**Sorun:** `resolve_agent_workdir` worktree yoksa **repo köküne düşüyor** — izolasyon sözü bozuluyor; `.env.local`/port offset hiç yok.

**Karar:**
- `resolve_agent_workdir` repo köküne düşme davranışı **kaldırılır**; yerine `ensure_agent_worktree(repo, agent)` → yoksa `worktree_create` (branch stratejisi: `NewBranchFrom { base_branch: settings'ten seçilen ana dal, name: "agent/<slug>" }`), varsa mevcut.
- `worktree_prepare_env(agent)` → her worktree'ye **`.env.local`** (gitignored; docs 10.3):
  ```dotenv
  PORT=3000+(agent_id*10)  REDIS_DB=<agent_id>  TEST_DB=test_<agent_id>
  AGENTHUB_AGENT_ID=<id>   AGENTHUB_WORKTREE=<path>
  ```
- **Sır politikası (docs 15.1):** ana `.env` worktree'ye KOPYALANMAZ; `.env.local` yalnız offset değişkenlerini içerir; `.env*` zaten `.gitignore`'da.
- `node_modules` paylaşımı: pnpm workspace'lerde zaten symlink; ayrıca her worktree'de `node_modules` yoksa ana repoya **bağıl symlink** (Windows'ta junction) — platform korumalı, başarısızsa sessizce atla (docs 10.1 m.3).
- `worktree_remove` seçenekleri genişler (docs 6.2): `{ delete: true } | { keep: true } | { commit_and_keep: true }` — commit seçeneği `git -C <wt> add -A && commit` + branch saklama.
- Yeni komut: `worktree_for_agent(repo, agent_id)` (spawn öncesi frontend'in görmesi için), `worktree_prepare_env` dahili.

**Gerekçe:** Gate'in "her biri kendi worktree'sinde" koşulu; docs 10.3'ün birebir karşılığı. **Risk:** worktree disk büyümesi → dokümantasyon + M2'de kotası.

### ADR-6 — Görev protokolü (AGENT_TASK.md) — hafif sürüm (docs 13.1–13.2)

**Karar:**
- `task_create(title, description, acceptance_criteria, priority, budget)` + `task_assign(agent_id, task_id, options)` komutları.
- `task_assign` akışı: `ensure_agent_worktree` → worktree köküne `AGENT_TASK.md` yaz (görev, kabul kriterleri, branch, bütçe, ilgili dosyalar) → `SpawnOptions { non_interactive: true, task_file, budget, turns, model }` ile `spawn_cli` → oturum `task_id` ile etiketlenir → `tasks.column='in_progress'`, `started_at`.
- Tamamlanma: (1) parser `TaskCompleted/TaskFailed` sinyali, (2) worktree'de `TASK_COMPLETE.md`/`TASK_BLOCKED.md` varlığı (docs 13.2), (3) exit code 0 + test/typecheck geçti (opsiyonel deterministik gate, docs 09). İlk eşleşen kazanır → `tasks.completed_at/spent_cost` günceller, `column='review'|'done'`.
- Oturum → görev bağlantısı: `PtyEvent`'e `task_id` alanı (opsiyonel) eklenir; `events` tablosunda `task_id` zaten var.

**Gerekçe:** Gate'in "görev alabilir" koşulu; M2 CEO/kanban'ın veri omurgası. **Sınır:** bağımlılık grafiği, paralel görev, handoff M2.

### ADR-7 — Repo seçici + settings kalıcılığı

**Karar:** `AGENTHUB_REPO_PATH` env köprüsü dev modu olarak kalır; asıl akış:
- `settings_get(key) / settings_set(key, value)` komutları (`settings` tablosu).
- `repo_select(path)` → geçerlilik (`.git` varlığı) + `settings`'e `repo_path` yazar.
- `resolve_repo_root()` sırası: `settings.repo_path` → env (dev) → `current_dir` (son çare).
- TopBar "Proje: repo kökü" çipi tıklanabilir olur → `tauri-plugin-dialog` klasör seçici (permission `dialog:default` zaten var; gerekirse `dialog:allow-open` genişletilir).
- Açılışta `repo_path` yoksa onboarding mini-dialog (FAZ1'de basit: seç + "atla").

### ADR-8 — Oturum/terminal: motor ajan kaydından türetilir

**Karar:**
- `PtyTerminal`'deki sabit `'pty' | 'claude'` seçici **kaldırılır**; sekme, ajanın DB `motor` alanından adaptörü çözer (`pty_find_by_engine_type(motor)` → ilk sağlıklı adaptör). Shell oturumu (`pty` engine) yalnızca "Hızlı Terminal" eyleminde açılır (ör. kurulum akışı için).
- `TerminalTabs` engine kestirimi (`engineType === 'claude'`) yerine `session.engineType`'ı doğrudan kullanır.
- `xterm-addon-serialize` bağlanır: oturum sonlandığında (exit/stop) terminal buffer'ı `terminal.serialize()` ile alınır → JSONL transcript'e "session_buffer" kaydı; sekme kapatılıp yeniden açıldığında geri yazılır (bellek içi; DB'ye yazılmaz).
- JSONL yazıcı: `dirs::home_dir()/.agentcompany/logs/<agent-slug>/<task>-<ts>.jsonl` (docs 12.2); `.gitignore`'a `.agentcompany/` eklenir.

**Gerekçe:** FAZ0 S9'u kapatır; docs 12.2 konuşma geçmişi zorunluluğu; serialize ertelemesini kapatır.

### ADR-9 — DB migration altyapısı (`PRAGMA user_version`)

**Karar:** `db.rs`'e sıralı migration runner:
```rust
const MIGRATIONS: &[&str] = &[
  // 1 = FAZ0 taban şeması (mevcut SCHEMA)
  // 2 = FAZ1: events(agent_id, timestamp) indeksi + starter company seed (yalnız boşsa)
];
```
- `open()`: `PRAGMA user_version` oku; her migration'ı bir işlemde uygula; `user_version`'ı güncelle. Mevcut DB'ler (v0) otomatik 1'den başlar — geriye uyumlu.
- `SCHEMA` sabiti migration 1 olur; `seed_demo_agents` migration 2'ye taşınır (boş tablo koşulu korunur).
- **Test:** tempfile DB üzerinde 0→2 geçişi + idempotency unit testi.

### ADR-10 — Frontend bileşen mimarisi

**Karar (docs 5.3/16 ile uyumlu):**
- `src/store/`: `agents.ts` (hire/fire/update + fetch), yeni `projects.ts` (repo path), yeni `tasks.ts` (hafif: create/assign/list), `terminal.ts` (Progress/Cost durumu), `settings.ts` (tema + repo).
- Yeni bileşenler: `Settings/HireWizard.tsx` (3 adım — docs 6.1), `Settings/FireDialog.tsx` (docs 6.2 onay + seçenekler), `OfficeFloor/AgentDesk.tsx`, `OfficeFloor/StatusBadge.tsx`, `OfficeFloor/OfficeFloor.tsx` (SVG + zoom/pan — `react-zoom-pan-pinch` veya custom viewBox; docs 5.6), `Tasks/TaskDialog.tsx` (hafif görev ver), `Settings/EngineInstallCard.tsx`.
- Yeni shadcn bileşenleri: `select`, `alert-dialog`, `switch`, `textarea`, `tooltip`, `dropdown-menu`, `progress`.
- Hook: `useEngineRegistry()` (detect/health yenileme + kurulum durumu).
- Doküman: `src/components/OfficeFloor/` klasörüne ayrım (docs 16).

---

## 5. İş Parçaları (Work Packages)

Öncelik: **P0** = Gate zorunlu, **P1** = kapsam, **P2** = iyileştirme/stretch. Süreler tek geliştirici için.

| # | İş | ADR/WP | Öncelik | Süre | Kabul (DoD) |
|:--:|:---|:---|:--:|:--:|:---|
| WP-0 | FAZ0 gate doğrulaması (kullanıcı makinesinde) | — | P0 | 0.5 sa | `cargo test && cargo clippy -D warnings` yeşil; `pnpm check/typecheck/build` yeşil; `claude --version` ≥ 2.1.90; `claude doctor` temiz |
| WP-1 | DB migration runner + starter company seed + settings komutları | ADR-1, ADR-9 | P0 | 4 sa | 0→2 migration testi; `agent_hire`/`agent_fire`/`agent_get`/`agent_update` unit testleri; mevcut DB uyumlu |
| WP-2 | `SpawnOptions` tam hali + `agent_spawn_engine` genişletme + TS tipleri | ADR-2 | P0 | 4 sa | Trait default'ları korunur; claude.rs + `pty/mod.rs` derlenir; TS `SpawnOptions` tipi eşleşir |
| WP-3 | codex/gemini/opencode/aider adaptörleri + registry + `agent_install_engine` | ADR-3 | P0 | 12–16 sa | Her adaptör: detect/health/spawn_cli unit testi (mock binary); `with_builtins()` 6 adaptör; Settings'te kurulu/değil + kurulum ipucu |
| WP-4 | OutputParser iskeleti + claude stream-json + opencode JSONL + Progress event | ADR-4 | P1 | 8 sa | Fixture tabanlı parser testleri; `PtyEventKind::Progress` uçtan uca akar; JSONL'a yazılır |
| WP-5 | Worktree otomasyonu: ensure/prepare_env/`.env.local` + port offset + silme seçenekleri | ADR-5 | P0 | 8 sa | İki ajan eşzamanlı spawn → farklı worktree + farklı PORT; repo köküne düşme yok; `.env.local` gitignore |
| WP-6 | Repo seçici + settings kalıcılığı + onboarding mini-dialog | ADR-7 | P0 | 4 sa | Dialog ile seçim → `settings`'e yazılır; yeniden başlatmada hatırlanır; TopBar çipi güncellenir |
| WP-7 | Hire Wizard UI (3 adım) + preset roller (docs 6.3) + masa atama | ADR-1, ADR-10 | P0 | 10–12 sa | Rol→motor/yetenek→kişilik akışı; "İşe Al" → DB kaydı + ofiste masa; özel rol desteklenir |
| WP-8 | Fire onay akışı (docs 6.2) + Inspector'a buton | ADR-1 | P0 | 4 sa | Onay diyaloğu 4 seçenekli; ajan `fired`; worktree seçenekleri uygulanır; açık görevler backlog'a |
| WP-9 | Ofis katı SVG v1: AgentDesk + StatusBadge + zoom/pan + seçim | ADR-10, docs 5.4–5.6 | P0 | 10 sa | 6 durum rengi (docs 5.4); tıklama → Inspector; zoom/pan; `prefers-reduced-motion` |
| WP-10 | Görev protokolü: task_create/assign + AGENT_TASK.md + tamamlanma algılama | ADR-6 | P0 | 10–12 sa | "Görev Ver" → worktree'de `AGENT_TASK.md` → non-interactive spawn → `TASK_COMPLETE`/parser ile `tasks` güncellenir; bütçe/turn flag'leri iletiliyor |
| WP-11 | JSONL oturum kaydı + serialize bağlama | ADR-8 | P1 | 4 sa | `<home>/.agentcompany/logs/...` dosyaları oluşur; stop'ta buffer serialize edilir; `.gitignore` güncel |
| WP-12 | Sağlık paneli genişletme + kurulum akışı (PTY'de) | ADR-3 | P1 | 4 sa | Settings "Motorlar" sekmesinde her adaptör rozet + "Kur" (onaylı); kurulum terminal sekmesinde akar |
| WP-13 | Bütçe/effort aktarımı + cost telemetri iskeleti | ADR-2, ADR-4 | P1 | 4 sa | Hire'daki değerler spawn'a ulaşır; Progress cost JSONL'da; TopBar CostMeter'a placeholder gerçek sayaç (M2) |
| WP-14 | Test/CI/doküman: mock-CLI matrisi, CI güncelleme, DEVELOPERS/README, kapanış denetimi | ADR-3, ADR-9 | P0 | 6 sa | CI 3 OS yeşil; dokümanlar güncel; FAZ1 kapanış raporu (FAZ0 §12 formatında) |

**Toplam:** ~90–110 saat ≈ 3 hafta (Docs 17 M1).

> **Sprint dosyaları:** Her WP için uygulanabilir sprint kartları `Sprints/` klasöründedir —
> başlangıç noktası [`Sprints/00-INDEX.md`](./Sprints/00-INDEX.md) (takvim, bağımlılıklar, yürütme kuralları).

---

## 6. Bağımlılık Grafiği ve Zaman Çizelgesi

```
WP-0 (ön koşul)
  └─► WP-1 (DB) ───────────────► WP-7 (Hire UI) ──► WP-8 (Fire) ──► WP-12 (Sağlık)
  └─► WP-2 (SpawnOptions) ──► WP-3 (adaptörler) ──► WP-4 (parser)
  └─► WP-6 (repo seçici) ──► WP-5 (worktree otomasyonu) ──► WP-10 (görev protokolü)
  └─► WP-9 (ofis katı) ─────────────────────────────────────────┘
  └─► WP-11 (JSONL) ──► WP-13 (bütçe) ──► WP-14 (test/CI/doküman + Gate)
```

### Hafta 1 — "Veri + motor katmanı" (WP-0, WP-1, WP-2, WP-3, WP-6)

| Gün | Hedef | Çıktı / kontrol |
|:--:|:---|:---|
| 1 | WP-0 + WP-1 | `cargo test` yeşil; `agent_hire/fire` unit testleri; migration 0→2 |
| 2 | WP-2 + WP-6 | `SpawnOptions` + TS tip; repo seçici çalışıyor (dialog) |
| 3–5 | WP-3 | codex + gemini adaptörleri (detect/health/spawn_cli + mock test) |
| 5 | WP-3 devam | opencode + aider; registry 6 adaptör; `agent_install_engine` |

**Hafta 1 Gate (dahili):** Settings'te 4 yeni motor "kurulu/değil + sürüm" gösteriyor; `agent_hire` DB'ye gerçek kayıt yazıyor.

### Hafta 2 — "İşe alım + ofis" (WP-5, WP-7, WP-8, WP-9, WP-11)

| Gün | Hedef | Çıktı / kontrol |
|:--:|:---|:---|
| 6–7 | WP-5 | İki ajan eşzamanlı spawn → ayrı worktree + port offset; repo köküne düşme yok |
| 8–9 | WP-7 | Hire Wizard 3 adım; preset roller; masa atama |
| 10 | WP-8 + WP-11 | Fire akışı; JSONL kaydı + serialize |
| 11–12 | WP-9 | SVG ofis: AgentDesk + StatusBadge + zoom/pan |

**Hafta 2 Gate (dahili):** "İşe Al" → ofiste masa + inspector; "İşten Çıkar" → onay + worktree seçenekleri; eşzamanlı 2 oturum sorunsuz.

### Hafta 3 — "Görev + kapanış" (WP-4, WP-10, WP-12, WP-13, WP-14)

| Gün | Hedef | Çıktı / kontrol |
|:--:|:---|:---|
| 13–14 | WP-4 + WP-10 | `AGENT_TASK.md` akışı; tamamlanma algılama; `tasks` güncelleme |
| 15 | WP-12 + WP-13 | Kurulum akışı; bütçe/effort bayrakları; cost telemetri |
| 16–17 | WP-14 | Mock-CLI test matrisi; CI güncelleme; dokümanlar; **Gate denetimi** |

**Hafta 3 sonu:** FAZ1 Gate doğrulaması + kapanış denetimi (FAZ0 §12 formatında).

---

## 7. FAZ1 Kabul Kriterleri (Gate — genişletilmiş)

1. ✅ En az **2 farklı CLI motoru** (ör. claude + codex) `detect()` ile sürümünü raporluyor; Settings'te kurulu/değil + sürüm + kurulum ipucu görünüyor; kurulmamış motor için onaylı kurulum akışı çalışıyor.
2. ✅ **Hire Wizard** (3 adım: rol → motor/yetenek → kişilik) ile ajan işe alınabiliyor; DB'ye kayıt (config_json: model/effort/bütçe/turn/izin profili), ofiste masa + renk + durum rozeti.
3. ✅ Her ajan görev aldığında **kendi worktree'si otomatik oluşuyor** (`agent/<slug>` branch, `.git/agenthub-worktrees`), `.env.local` port offset ile yazılıyor; spawn **worktree cwd'sinde** oluyor (repo köküne düşme yok).
4. ✅ **"Görev Ver"** → worktree'de `AGENT_TASK.md` → non-interactive spawn → çıktı ajanın terminal sekmesinde akıyor → tamamlanma/hatada `tasks` satırı ve `events` güncelleniyor (bütçe/turn flag'leri CLI'ya iletilmiş).
5. ✅ **İşten çıkarma onay akışı:** açık görevler backlog'a, worktree sil/koru/commit'le-sakla seçenekleri, log saklama; ajan `fired` durumuna geçiyor, ofisten kalkıyor.
6. ✅ **Eşzamanlılık:** en az 2 ajan (farklı motorlar dahil) aynı anda çalışabiliyor; birini durdurmak (`agent_stop`) diğerini etkilemiyor (process-group/Job Object korunuyor).
7. ✅ **Repo seçici:** dialog ile seçim yapılıyor, `settings` tablosunda kalıcı; açılışta hatırlanıyor; TopBar proje çipi güncel yolu gösteriyor.
8. ✅ **Progress/cost iskeleti:** claude `stream-json`/opencode JSONL'den üretilen `Progress` olayları JSONL transcript'e yazılıyor (M2 sayaç UI'ının verisi hazır).
9. ✅ `pnpm check`, `pnpm typecheck`, `pnpm build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` temiz; **CI (3 OS) yeşil**; husky pre-commit aktif.
10. ✅ **Güvenlik regresyonu yok:** frontend keyfi `program/args` ile spawn edemiyor (tüm motor spawn'ları `engine_type` üzerinden); `.env*` worktree'lere kopyalanmıyor; `.agentcompany/` gitignore'da.
11. ✅ 10K+ satır çıktıda WebGL terminal akıcı kalıyor (regresyon yok); `xterm-addon-serialize` buffer persist çalışıyor.
12. ✅ Dokümantasyon: `DEVELOPERS.md` FAZ1 notları, `README.md` yol haritası `FAZ1 ✅`, bu planın kapanış denetimi bölümü doldurulmuş.

---

## 8. Riskler ve Önlemler

| Risk | Etki | Önlem |
|:---|:---|:---|
| CLI flag/sürüm kayması (codex/gemini/opencode/aider) | spawn kırılır, detect yanlış | `detect()`'te min. sürüm kapısı + capability ilanı; flag matrisi `--help` çıktısıyla uygulama sırasında doğrulanır; **fixture tabanlı mock-CLI testleri** (PATH'e sahte binary) sürüm bağımsız |
| Gerçek CLI'lar sandbox/CI'da yok | entegrasyon testi yapılamaz | Mock binary matrisi (unit) + kullanıcı makinesinde elle senaryo listesi (WP-14); CI'da yalnız derleme/test |
| `SpawnOptions` genişletmesinde TS↔Rust alan uyumsuzluğu | sessiz default, yanlış flag | `serde(rename_all="camelCase")` + tip testi; `pnpm typecheck` Gate |
| Worktree disk büyümesi (node_modules kopyası) | disk şişer | node_modules symlink/junction (platform korumalı); `worktree prune` dokümantasyonu; M2'de kota |
| `.env.local`/secret sızıntısı | güvenlik | Ana `.env` worktree'ye kopyalanmaz; `.env.local` yalnız offset değişkenleri; `.gitignore` kapsamı genişletilir; FAZ1 kapanış denetiminde taranır |
| Tek `Mutex<Connection>` çekişmesi (eşzamanlı spawn + olay yazma) | gecikme | FAZ1'de olay yazma seyrek (exit başına 1); gerekiyorsa `record_event`'i ayrı thread'e al; havuz M2 |
| Windows'ta Job Object + yeni CLI'lar (ConPTY uyumsuzluğu) | spawn hatası | `spawn_pty_isolated` tek nokta; hata mesajları UI'da; CI Windows matrisi + elle doğrulama |
| Hire Wizard kapsam şişmesi (özel rol, MCP, skill editör) | takvim kayar | FAZ1: 3 adım sabit, özel rol = metin alanı; MCP/skill yönetimi M2+; preset düzenleme M2 |
| `claude --worktree` bayrağı ile kendi worktree yöneticimiz çakışması | çift worktree | Kendi yöneticimiz path'i belirler; claude **`--worktree` bayrağı olmadan** worktree cwd'sinde spawn edilir (docs 10.1 m.4) |
| Parser'lar gerçek çıktıda yanlış eşleşir (onay/tamamlanma) | yanlış durum | Regex/JSONL eşleşmeleri dar tutulur; yanlış pozitifte ajan `blocked` değil `running` kalır (güvenli yön); tam onay köprüsü M2 |
| Eski DB (user_version 0) ile uyumsuzluk | açılış hatası | Migration runner v0→1→2 idempotent; tempfile üzerinde 0→2 testi (WP-1) |

---

## 9. Metrikler (FAZ1 başarı ölçümü)

| Metrik | Hedef | Nasıl ölçülür |
|:---|:---|:---|
| Time-to-hire | < 60 sn (3 adım sihirbaz) | Elle senaryo + `events` kayıtları |
| Time-to-first-task | < 2 dk (işe alım sonrası) | `events` (spawn → exit) zaman damgaları |
| Eşzamanlı oturum | ≥ 2 farklı motor | `pty_list_all_ids` + `sessions` |
| Spawn başarısızlık oranı | < %5 (worktree kuruluysa) | `events.event_type = 'spawn'/'spawn_engine'` vs `error` |
| Görev tamamlanma algılama doğruluğu | ≥ %90 fixture setinde | Parser unit testleri |
| Kalite kapıları | 0 hata | `cargo clippy -D warnings`, `pnpm check/typecheck`, CI |
| Cost takibi hazırlığı | `Progress` olayı her oturumda üretiliyor | JSONL transcript taraması |

---

## 10. Sonraki Adım (M2 Köprüsü)

FAZ1 gate'i geçildiğinde sıradaki milestone (AjanOfis "Faz 3: CEO Orkestrasyonu (4 Hafta) — M2"; repo README'de "Faz 3 (M2)"):
- Kanban panosu (`@dnd-kit` + react-virtual, WIP limitleri, swimlane) + `tasks` veri modelinin UI'a bağlanması (docs 8).
- CEO orkestratör: görev ayrıştırma, uygun ajana atama, paralel yönetim (docs 4.3; referans: CAO C1).
- Onay akışı (allow/deny/edit/always) + policy engine (regex/lexer; referans: `destructive_command_guard` C4; docs 15).
- Maliyet/token dashboard (FAZ1'in `Progress` telemetrisini tüketir; docs 11, 14.3).
- Tüm ajanları durdurma kill switch (Ctrl/Cmd+Shift+X); görev devri (handoff).

Bu fazın repo/kütüphane listesi, FAZ0 raporu §5'teki C1/C4 ve `@dnd-kit`, `react-virtual` üzerinden hazırdır; FAZ1 kapanış denetiminden sonra aynı doğrulama yöntemiyle ayrı bir M2 planı çıkarılabilir.

---

## 11. Ek A — Yeni Tauri Komutları Özeti

| Komut | Yön | Açıklama |
|:---|:---|:---|
| `agent_hire` | FE→BE | Ajan kaydı oluşturur (config_json dahil) |
| `agent_fire` / `agent_delete` | FE→BE | Pasifleştir / kalıcı sil (worktree seçenekleriyle) |
| `agent_update` / `agent_get` | FE→BE | Patch / tek kayıt |
| `agent_install_engine` | FE→BE | Motor kurulum komutunu onaylı PTY oturumunda çalıştırır |
| `task_create` / `task_assign` / `task_list` | FE→BE | Görev protokolü (hafif) |
| `settings_get` / `settings_set` | FE→BE | `settings` tablosu (repo_path, ana dal, tema...) |
| `repo_select` | FE→BE | Dialog sonucunu doğrular + `settings`'e yazar |
| `worktree_for_agent` / `worktree_prepare_env` | FE→BE | Spawn öncesi worktree durumu + `.env.local` |
| `pty_engine_install_status` | FE→BE | Kurulum ilerlemesi (Progress olayı) |

Mevcut komutlarda değişen: `agent_spawn_engine` (+ `options: SpawnOptions`), `PtyEventKind` (+ `Progress`), `DetectResult` (+ `install_hint`).

## 12. Ek B — Hızlı Doğrulama Komutları (her WP sonunda)

```bash
# Frontend
pnpm check && pnpm typecheck && pnpm build

# Backend
cd src-tauri && cargo check --locked && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked

# Elle senaryo (Tauri masaüstü)
pnpm tauri:dev
# 1) Settings → Motorlar: 6 adaptör listeleniyor (kurulu olanlar rozetli)
# 2) + İşe Al → 3 adım → ofiste yeni masa
# 3) Ajan → Görev Ver → worktree oluştu (.git/agenthub-worktrees/<slug>), AGENT_TASK.md yazıldı
# 4) İkinci bir ajan (farklı motor) → eşzamanlı çalıştır → port offset farkı (worktree/.env.local)
# 5) İşten Çıkar → onay → worktree seçenekleri → ajan fired
```

---

*Bu plan, FAZ0 kapanış denetimi (§12) ve Docs 17 M1 tanımıyla uyumludur. Uygulama `arena/019fecb5-agenthub` dalında, İngilizce commit mesajları ve `M1:` ön ekiyle ilerler; her WP kendi unit testi + CI doğrulamasıyla kapanır.*
