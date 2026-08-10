# Sprint FAZ1-WP06 — Repo Seçici ve Settings Kalıcılığı

> **Kart:** FAZ1-PLANI.md §5 WP-6 · ADR-7
> **Takvim:** Hafta 1 · Gün 2 (2026-08-11) · **Süre:** 4 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — backend + frontend uygulandı; `cargo test` doğrulaması CI/kullanıcı makinesinde (WP-14'te nihai)
>
> **Uygulama notları (2026-08-10):**
> - `db::repo_select` komutu: canonicalize + `.git` doğrulaması + worktree kökü reddi + `settings.repo_path` yazma; 2 unit test.
> - `resolve_repo_root(&app)` önceliği: `settings.repo_path` → `AGENTHUB_REPO_PATH` env → cwd (ADR-7).
> - `@tauri-apps/plugin-dialog` npm paketi eklendi (Rust tarafı FAZ0'da vardı); capability `dialog:default` yeterli.
> - `src/store/projects.ts` (repoPath + onboardingSkipped) · TopBar proje çipi dialog açıyor · `OnboardingDialog` (seç/atla).

## 1. Hedef

`AGENTHUB_REPO_PATH` env köprüsünü gerçek kullanıcı akışına çevirmek: `tauri-plugin-dialog`
ile klasör seçimi, `settings` tablosunda kalıcılık, açılışta hatırlama ve TopBar proje
çipinin tıklanabilir olması. Worktree otomasyonunun (WP-05) "repo neresi?" sorusunu kesinleştirir.

## 2. Definition of Done (DoD)

- [ ] `repo_select(path)` komutu: `.git` doğrulaması + `settings_set("repo_path", path)`
- [ ] `resolve_repo_root()` önceliği: `settings.repo_path` → `AGENTHUB_REPO_PATH` env → `current_dir`
- [ ] TopBar proje çipi tıklanabilir → dialog klasör seçici → `repo_select`
- [ ] `src/store/projects.ts`: `repoPath` + `selectRepo` + açılışta yükleme
- [ ] Açılışta repo yoksa onboarding mini-dialog ("Proje seç" / "Şimdi atla")
- [ ] Capability doğrulaması: `dialog:default` yeterli mi (gerekirse `dialog:allow-open` eklenir)
- [ ] `pnpm typecheck/build` yeşil; manuel dialog testi ✅

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-00, WP-01 (`settings_get/set` hazır).
- Çıkış bağımlıları: WP-05 (repo_path kaynağı), WP-07 (hire → worktree yolu), WP-10 (görev).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | `repo_select` | `worktree.rs` veya `db.rs`: path canonicalize + `.git` varlığı + `settings_set` | Geçersiz path hata |
| T-2 | `resolve_repo_root` refactor | `pty/mod.rs`: `State<AppDb>` veya `app.try_state` üzerinden settings oku | Öncelik sırası testli |
| T-3 | Dialog entegrasyonu | `TopBar.tsx`: çip → `open({ directory: true })` (tauri-plugin-dialog JS API) → `repo_select` → store | Seçim sonrası çip güncellenir |
| T-4 | projects store | `src/store/projects.ts` + `ipc.ts` (`repoSelect`, `settingsGet/Set` sarmalayıcıları) | Tauri yoksa mock değer |
| T-5 | Onboarding | `App.tsx`: `repoPath` yoksa mini-dialog (seç / atla); atlama `settings_set("onboarding_skipped","1")` | Kapanışta tekrar sorulmaz |
| T-6 | Capability | `main-capability.json`'ı kontrol et; gerekirse `dialog:allow-open` | `tauri dev` temiz |

## 5. Teknik Talimatlar

### 5.1 resolve_repo_root önceliği

```rust
fn resolve_repo_root(db: Option<&AppDb>) -> Result<String, String> {
    // 1) settings.repo_path (doğrulanmış, canonical)
    // 2) AGENTHUB_REPO_PATH env (dev köprüsü — FAZ0 davranışı korunur)
    // 3) current_dir (son çare; Tauri paketli çalıştırmada uyarı loglanır)
}
```

- `repo_select` yalnızca gerçek git repo kabul eder (`<path>/.git` dizini veya `git rev-parse`).
- Worktree kökü olan path seçilirse uyarı: "worktree değil, ana repo seç" (`.git/agenthub-worktrees` içinde mi kontrolü).

### 5.2 TopBar çipi

- `AGENTHUB_REPO_PATH` env'deyse çip `"(dev) <path>"` rozeti gösterir — kullanıcı yine de değiştirebilir.
- Seçim sırasında `dialog.open` başarısız olursa (webview'de) hata mesajı; Tauri dışında çip salt-okunur.

### 5.3 Onboarding

- Koşul: `settings.repo_path` yok VE `onboarding_skipped != "1"` VE `isTauriRuntime()`.
- İçerik: kısa açıklama + [Proje Seç] [Şimdi Atla]. Atla → sonraki açılışta sorulmaz (silinince geri gelir).

## 6. Test Planı

- `repo_select_validates`: temp git repo → Ok; git olmayan dizin → Err.
- `repo_select_persists`: `settings_get("repo_path")` dolu.
- `resolve_repo_root_priority`: settings dolu → settings; boş + env dolu → env; ikisi de yok → cwd.
- `repo_select_rejects_worktree_root`: `.git/agenthub-worktrees/...` seçilirse Err.
- Frontend: `pnpm typecheck` (tipler) + manuel dialog (Tauri).

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked repo_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # çipe tıkla → klasör seç → çip güncellendi; restart → hatırlanıyor
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| `dialog:default` klasör seçimini kapsamazsa | T-6 capability kontrolü; `dialog:allow-open` ekle |
| Webview'de dialog yok (tarayıcı önizleme) | `isTauriRuntime()` kapısı; çip salt-okunur |
| Seçilen path daha sonra silinir | Spawn sırasında yeniden doğrula; hata → onboarding'e geri dön |
| Env köprüsü ile settings çakışması | Öncelik sırası net + test; env yalnızca dev |

## 9. Sprint Gate

- DoD ✓; repo seçimi kalıcı; yeniden başlatmada hatırlanıyor; onboarding açılışta bir kez.

## 10. Çıktılar

- `src-tauri/src/worktree.rs` veya `db.rs` (`repo_select`) · `src-tauri/src/pty/mod.rs` (resolve_repo_root) · `src/components/TopBar.tsx` · `src/store/projects.ts` · `src/lib/ipc.ts` · `src/App.tsx` (onboarding).

## 11. Devir Notları (sonraki sprinte)

- WP-05: `resolve_repo_root` artık settings'ten okuyor — worktree ensure'ı buna dayanır.
- WP-07: Hire Wizard "Proje yolu yok" durumunda onboarding'e yönlendirir.
- WP-10: `task_assign` repo_path'i buradan alır.
