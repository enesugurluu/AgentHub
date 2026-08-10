# Sprint FAZ1-WP05 — Worktree Otomasyonu ve Runtime İzolasyonu

> **Kart:** FAZ1-PLANI.md §5 WP-5 · ADR-5
> **Takvim:** Hafta 2 · Gün 6–7 (plan: 2026-08-17 → 2026-08-18) · **Süre:** 8 sa · **Öncelik:** P0
> **Durum:** ⏳ Planlandı

## 1. Hedef

"Her ajan kendi worktree'sinde" koşulunu gerçek kılmak: worktree yoksa otomatik
oluşturma, `.env.local` + port offset (docs 10.3), `node_modules` paylaşımı, silme
seçenekleri (`delete | keep | commit_and_keep`). FAZ0'ın repo köküne düşme davranışı
**kaldırılır** — izolasyon sözü artık tavizsiz.

## 2. Definition of Done (DoD)

- [ ] `ensure_agent_worktree(repo, agent) -> WorktreeInfo` (yoksa `worktree_create`, varsa mevcut)
- [ ] `resolve_agent_workdir` repo köküne düşmüyor; hata mesajı: "önce işe al / görev ata"
- [ ] `worktree_prepare_env(agent)` → worktree'ye `.env.local` (PORT=3000+id*10, REDIS_DB, TEST_DB, AGENTHUB_*)
- [ ] `node_modules` bağıl symlink/junction denemesi (başarısızlık sessiz)
- [ ] `worktree_remove` seçenekleri: `delete | keep | commit_and_keep` (commit: `git add -A && git commit`)
- [ ] `worktree_for_agent(repo, agent_id)` komutu (spawn öncesi UI bilgisi)
- [ ] Testler: temp git repo'da ensure idempotent; port offset benzersiz; remove seçenekleri ✅
- [ ] clippy + `pnpm typecheck/build` yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-00, WP-01 (agent_get — isim/rol için), WP-06 (repo_path settings'ten — sıralama esnek: WP-06'nın `settings` komutları WP-01'de geldiği için `resolve_repo_root` önceliği bu sprintte de güncellenebilir).
- Çıkış bağımlıları: WP-07 (hire → worktree), WP-10 (görev → worktree), WP-08 (fire → silme).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | `ensure_agent_worktree` | `worktree_create`'i `NewBranchFrom { base_branch: settings "main_branch" veya "main", name: "agent/<slug>" }` ile çağırır; metadata'da agent_id dolu | İdempotent |
| T-2 | Workdir kararı | `pty/mod.rs::resolve_agent_workdir` → DB'den agent oku; worktree yoksa `ensure`; hata yoksa repo köküne **düşme** | Hata mesajı açıklayıcı |
| T-3 | `.env.local` | `worktree_prepare_env`: offset değişkenleri + AGENTHUB_* (aşağıdaki şablon); mevcut dosyayı koru (üzerine yazma) | İçerik doğru |
| T-4 | node_modules linki | Unix `std::os::unix::fs::symlink`; Windows `std::os::windows::fs::symlink_dir`; hedef yoksa/başarısızsa sessiz | Hata yutulur, loglanır |
| T-5 | Silme seçenekleri | `worktree_remove(path, opts)`: delete (mevcut), keep (sadece metadata sil, dizin kalır), commit_and_keep (git add -A + commit + keep) | Her seçenek testli |
| T-6 | Komutlar | `worktree_for_agent`, `worktree_prepare_env` (opsiyonel `#[tauri::command]`) `lib.rs`'e | invoke çalışıyor |
| T-7 | `.gitignore` | `.agentcompany/` (WP-11 logları) + worktree `.env.local` zaten `*.local` ile kapalı — doğrula | Git temiz |

## 5. Teknik Talimatlar

### 5.1 `.env.local` şablonu (docs 10.3)

```dotenv
PORT=3000            # -> 3000 + (agent_id * 10)
REDIS_DB=1           # -> agent_id
TEST_DB=test_1       # -> test_<agent_id>
AGENTHUB_AGENT_ID=1
AGENTHUB_WORKTREE=/abs/path/to/worktree
```

- **Sır politikası (docs 15.1):** ana `.env` worktree'ye KOPYALANMAZ; `.env.local` yalnızca
  offset + AGENTHUB değişkenlerini içerir. `*.local` zaten `.gitignore`'da — doğrula.
- Mevcut `.env.local` varsa: yalnızca eksik anahtarlar eklenir (ajanın elle yaptığı ayarlar korunur).

### 5.2 ensure_agent_worktree imzası

```rust
pub fn ensure_agent_worktree(
    repo_path: &str, agent: &db::AgentRecord, base_branch: &str,
) -> Result<WorktreeInfo, String>
```

- Mevcut worktree bulma: `worktree_list(repo_path)` içinde `agent_id == agent.id` eşleşmesi
  (mevcut `resolve_worktree_path_for_agent`'in yerine geçer veya onu kullanır).
- Branch adı: `agent/<sanitized-name>-<suffix>` — `sanitize_agent_name` mevcut.
- `base_branch` kaynağı: `settings_get("main_branch")` → varsayılan `"main"`; `origin/main`
  kontrolü `worktree_create` içinde zaten var.

### 5.3 pty/mod.rs değişikliği

```rust
fn resolve_agent_workdir(repo_path: &str, agent_id: &str, db: &AppDb) -> Result<String, String> {
    let agent = db.agent_get(agent_id)?;                 // WP-01
    let wt = ensure_agent_worktree(repo_path, &agent, "main")?;  // base_branch settings'ten
    Ok(wt.path)
}
```

- `agent_spawn` / `agent_spawn_engine` içinde `?` ile yayılır; hata UI'da görünür.
- **Regresyon:** FAZ0'daki "worktree yoksa repo köküne düş" testleri (varsa) güncellenir.

### 5.4 commit_and_keep

```bash
git -C <worktree> add -A
git -C <worktree> commit -m "agenthub: preserve worktree for <agent-name>"
# worktree dizini ve branch kalır; yalnızca .agenthub.json yönetimden çıkarılır (metadata silinir)
```

## 6. Test Planı

- `ensure_creates_and_is_idempotent`: temp repo (`git init` + ilk commit) → iki kez `ensure` → tek worktree.
- `workdir_falls_back_to_error_not_repo_root`: worktree'siz ajan → `Err` mesajı (repo kökü yok).
- `env_local_written_with_offset`: `PORT = 3000 + id*10`, `TEST_DB = test_<id>`; iki ajan farklı port.
- `env_local_preserves_existing_keys`: önceden var olan `PORT=9999` korunur (ya da belgelenen davranış).
- `remove_keep`: dizin kalır, metadata gider; `remove_commit_and_keep`: commit oluşur, branch korunur.
- `node_modules_link_attempt`: link başarılıysa hedef işaret eder; başarısızsa hata yok.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked worktree_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
# Manuel: iki ajan → Görev Ver → .git/agenthub-worktrees/<slug> oluştu; .env.local PORT'lar farklı
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| `base_branch` yoksa worktree_create zaten hata verir | settings'ten "main_branch" okunur; hata mesajı "Ana dalı Ayarlar'dan seç" |
| Windows junction ayrıcalık ister | Başarısızlık sessiz + `tracing::warn!`; disk paylaşımı iyileştirme |
| Worktree sayısı artarsa disk şişer | Dokümantasyon (`worktree prune`); M2'de kota/yaşlandırma |
| Repo köküne düşme davranışını kaldırmak eski akışları kırar | `agent_spawn` (shell) için shell oturumu da worktree ister; "Hızlı Terminal" aksiyonu repo kökünde açılabilir (bilinçli istisna, WP-12 notu) |

## 9. Sprint Gate

- DoD ✓; iki ajan eşzamanlı spawn → farklı worktree + farklı PORT; repo köküne düşme
  davranışı kodda yok (`rg "repo_path.to_string()" pty` temiz).

## 10. Çıktılar

- `src-tauri/src/worktree.rs` (ensure/prepare_env/remove seçenekleri) · `src-tauri/src/pty/mod.rs` (workdir kararı) · `src-tauri/src/lib.rs` (yeni komutlar).

## 11. Devir Notları (sonraki sprinte)

- WP-07: hire sonrası masa ataması `worktree_for_agent` ile gösterilir.
- WP-10: `task_assign` önce `ensure_agent_worktree` çağırır (bu sprintte hazır).
- WP-08: `FireOptions.worktree_action` bu sprintteki remove seçeneklerine bağlanır.
- WP-12: "Hızlı Terminal" istisnası (repo kökünde shell) not alındı.
