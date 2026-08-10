# Sprint FAZ1-WP01 — DB Migration Runner ve Ajan Yaşam Döngüsü Komutları

> **Kart:** FAZ1-PLANI.md §5 WP-1 · ADR-1, ADR-9
> **Takvim:** Hafta 1 · Gün 1 (2026-08-10) · **Süre:** 4 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — kod + birim testler yazıldı; `cargo test` doğrulaması CI/kullanıcı makinesinde (WP-14 kapanışında nihai)

## 1. Hedef

SQLite katmanını "yazılabilir" hâle getirmek: (a) `PRAGMA user_version` tabanlı sıralı
migration altyapısı, (b) ajan yaşam döngüsü komutları (`agent_hire/fire/update/get/delete`),
(c) `settings` tablosu komutları (`settings_get/set`), (d) demo seed'den "starter company"
seed'ine geçiş. Bu sprint, Hire Wizard (WP-07) ve repo seçicinin (WP-06) backend omurgasıdır.

## 2. Definition of Done (DoD)

- [ ] `db.rs`'te `MIGRATIONS: &[&str]` + `PRAGMA user_version` runner; v0 (mevcut) DB otomatik 1→2'ye taşınır
- [ ] `agent_hire / agent_fire / agent_delete / agent_update / agent_get` komutları `lib.rs` invoke_handler'da
- [ ] `settings_get(key) / settings_set(key, value)` komutları kayıtlı
- [ ] `HirePayload`, `FireOptions`, `AgentPatch`, `AgentRecord` (genişletilmiş) tipleri TS ile eşleşiyor
- [ ] Starter company seed (migration 2) yalnızca boş `agents` tablosuna yazıyor; demo seed kaldırıldı
- [ ] 0→2 migration + idempotency + fire/delete geçişleri için unit testler ✅
- [ ] `pnpm typecheck` (ipc.ts tip güncellemesi dahil) yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-00 (FAZ0 gate yeşil).
- Çıkış bağımlıları: WP-02 (`agent_get` kullanır), WP-05 (agent kaydı + worktree bağlama),
  WP-06 (settings), WP-07 (hire), WP-08 (fire), WP-10 (tasks), WP-13 (config_json).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Migration runner | `SCHEMA` sabitini `MIGRATIONS[0]` yap; `open()` sonunda `migrate()` çağır | v0 DB'de 1→2 uygulanır; `user_version=2` |
| T-2 | Migration 2 SQL | `agents` boşsa starter company (Backend/Frontend/QA) insert; `events(agent_id, timestamp)` indeksi | Boş olmayan tabloya dokunmaz |
| T-3 | Ajan komutları | `agent_hire(payload)` → insert + `hired_at`; `agent_fire(id, opts)` → `fired_at` + durum + açık görevler backlog'a; `agent_delete` kalıcı sil; `agent_update` patch; `agent_get` | Her komutun unit testi |
| T-4 | Settings komutları | `settings_get/set` (key/value string) | Roundtrip testi |
| T-5 | Tip genişletme | `AgentRecord`'a `avatarColor, configJson, hiredAt, firedAt` (camelCase); `ipc.ts` güncelle | `pnpm typecheck` |
| T-6 | Seed temizliği | `seed_demo_agents()` kaldır → migration 2'ye taşı | Eski çağrı yok |

## 5. Teknik Talimatlar

### 5.1 Migration runner (db.rs)

```rust
const MIGRATIONS: &[&str] = &[
    // v1 — FAZ0 taban şeması (mevcut SCHEMA sabiti buraya taşınır)
    r#"CREATE TABLE IF NOT EXISTS agents (...); CREATE TABLE IF NOT EXISTS tasks (...);
       CREATE TABLE IF NOT EXISTS events (...); CREATE TABLE IF NOT EXISTS settings (...);"#,
    // v2 — FAZ1: indeks + starter company (yalnızca boşsa)
    r#"CREATE INDEX IF NOT EXISTS idx_events_agent_ts ON events(agent_id, timestamp);
       INSERT INTO agents (name, role, motor, status) SELECT 'Ada','Frontend Dev','claude','idle'
         WHERE NOT EXISTS (SELECT 1 FROM agents);"#,
];

fn migrate(conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    let current: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    for (i, sql) in MIGRATIONS.iter().enumerate().skip(current as usize) {
        let tx = conn.transaction()?;          // her migration tek işlemde
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", (i + 1) as i64)?;
        tx.commit()?;
    }
    Ok(())
}
```

- `open()`: `conn` kur → WAL/foreign_keys pragma → `migrate(&mut conn)` → `AppDb::new`.
- Mevcut DB'lerde `user_version = 0` → v1 (IF NOT EXISTS, idempotent) uygulanır; sorun çıkmaz.

### 5.2 Ajan yaşam döngüsü — serileştirilebilir payload'lar

```rust
#[derive(Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct HirePayload {
    pub name: String, pub role: String, pub motor: String,
    pub model: Option<String>, pub effort: Option<String>,
    pub max_budget_usd: Option<f64>, pub max_turns: Option<u32>,
    pub permissions_profile: String,          // full|standard|limited|custom
    pub system_prompt: Option<String>, pub avatar_color: Option<String>,
    pub skills: Vec<String>, pub mcp_servers: Vec<String>,
}
```

- `config_json = serde_json::json!({model, effort, max_budget_usd, max_turns,
  permissions_profile, skills, mcp_servers})` — WP-13 bu alanları spawn'a taşır.
- `agent_fire(id, FireOptions { worktree_action: delete|keep|commit_and_keep,
  move_open_tasks_to_backlog: bool, keep_logs: bool })` — worktree aksiyonu **WP-05'te**
  implement edilir; bu sprintte yalnızca DB geçişi + görevleri backlog'a alma.
- `agent_fire` sonrası ajan `status='fired'`; ofis/sidebar `fired` kayıtları göstermez (WP-07/09 UI tarafı).
- `agent_delete` yalnızca `fired` kayıtlar için izinli (yanlışlıkla aktif ajan silinmesin).

### 5.3 settings komutları

```rust
#[tauri::command] pub fn settings_get(db: State<AppDb>, key: String) -> Result<Option<String>, String>;
#[tauri::command] pub fn settings_set(db: State<AppDb>, key: String, value: String) -> Result<(), String>;
// INSERT INTO settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value
```

### 5.4 Frontend tip güncellemesi (ipc.ts)

```ts
export type AgentRecord = {
  id: number; name: string; role: string; motor: string; model: string | null;
  status: string; worktreePath: string | null; createdAt: string | null;
  avatarColor: string | null; configJson: string | null;
  hiredAt: string | null; firedAt: string | null;
}
export function agentHire(payload: HirePayload): Promise<AgentRecord> { return invoke('agent_hire', { payload }) }
// + agentFire, agentDelete, agentUpdate, agentGet, settingsGet, settingsSet
```

## 6. Test Planı

- `db_migration_v0_to_v2`: tempfile DB'de eski şema (v0) kur → `AppDb::open` → `user_version==2`, tablolar var, seed yazılmış.
- `db_migration_idempotent`: aynı DB'de ikinci `open` → hata yok, çift seed yok.
- `agent_hire_roundtrip`: hire → `agent_get` → alanlar (config_json dahil) eşleşir.
- `agent_fire_transitions`: fire → `status=='fired'`, `fired_at` dolu; açık görevler backlog'a döndü.
- `agent_delete_only_fired`: aktif ajanda hata; fired ajanda silinir.
- `settings_roundtrip`: set→get→update→get.
- Mevcut `record_event` davranışı (sayısal olmayan agent_id → NULL) regresyon testi korunur.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked db_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| `PRAGMA user_version` unutulursa migration her açılışta koşar | `skip(current)` mantığı + unit test (ikinci open idempotent) |
| `AgentRecord` genişletmesi frontend'de kırılma | `ipc.ts` aynı sprintte güncellenir; `pnpm typecheck` gate |
| Seed'in yanlışlıkla üzerine yazması | `NOT EXISTS (SELECT 1 FROM agents)` koşulu |
| `agent_fire`'da worktree aksiyonu yok (WP-05 bekliyor) | API imzası hazır; worktree kısmı `todo!()` yerine `Ok(())` döner ve WP-05'te doldurulur |

## 9. Sprint Gate

- DoD ✓ + `cargo test --locked` (db_*) + clippy + `pnpm typecheck/build` yeşil.
- `agent_hire` ile oluşan kayıt `agent_list_all`'da görünüyor (manuel/unit).

## 10. Çıktılar

- `src-tauri/src/db.rs` (migration runner + komutlar) · `src/lib/ipc.ts` (tipler + sarmalayıcılar) · `src-tauri/src/lib.rs` (invoke_handler).

## 11. Devir Notları (sonraki sprinte)

- WP-02: `agent_get` hazır → SpawnOptions doğrulamasında kullan.
- WP-05: `FireOptions.worktree_action` alanı hazır; worktree davranışı orada doldurulacak.
- WP-06: `settings_get/set` hazır → repo_path kalıcılığı bunları kullanır.
- WP-13: `config_json` şeması netleşti → hire değerleri → SpawnOptions eşlemesi.
