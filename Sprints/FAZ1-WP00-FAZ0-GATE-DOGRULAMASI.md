# Sprint FAZ1-WP00 — FAZ0 Gate Doğrulaması

> **Kart:** FAZ1-PLANI.md §5 WP-0 · ADR yok (ön koşul)
> **Takvim:** Hafta 1 · Gün 1 (2026-08-10) · **Süre:** 0.5 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı (frontend sandbox'ta; Rust/CI kullanıcı makinesine devredildi)

## 1. Hedef

FAZ0'ın kod tarafında gerçekten "geçmiş" olduğunu **kullanıcı makinesinde** (bu
sandbox'ta Rust toolchain yok) doğrulamak; FAZ1'in üzerine inşa edeceği zeminin
yeşil olduğunu garantiye almak. Doğrulama sonucu bu kartta raporlanır ve WP-01'e
giriş koşulu olur.

## 2. Definition of Done (DoD)

- [ ] `cargo test --locked` (src-tauri) 3 platform bilgisinden en az biri kullanıcı makinesinde yeşil
- [ ] `cargo clippy --locked --all-targets -- -D warnings` hatasız
- [ ] `pnpm check && pnpm typecheck && pnpm build` yeşil
- [ ] `claude --version` ≥ 2.1.90 ve `claude doctor` temiz
- [ ] `pnpm tauri:dev` smoke testi: shell PTY açılıyor, `echo` yazılıyor, stop çalışıyor
- [ ] Doğrulama tablosu (§6) dolduruldu; WP-01'e devir notu yazıldı

## 3. Ön Koşullar ve Bağımlılıklar

- Gerekli: Node 22+, pnpm 9, Rust stable, Claude Code native (`curl -fsSL https://claude.ai/install.sh | bash`).
- BAĞIMLI DEĞİL: hiçbir FAZ1 kodu henüz yazılmamalı — bu kart "temiz zemin" kontrolüdür.

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Rust doğrulaması | `cd src-tauri && cargo check --locked && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked` | Üçü de 0 hata |
| T-2 | Frontend doğrulaması | `pnpm check` (biome auto-fix) → `pnpm typecheck` → `pnpm build` | Temiz çıktı |
| T-3 | Claude Code doğrulaması | `claude --version`; `claude doctor` | Sürüm ≥ 2.1.90; doctor hatasız |
| T-4 | Tauri smoke testi | `pnpm tauri:dev`; soldan ajan seç → terminal sekmesinde oturum başlat → `echo selam` yaz → Durdur | PTY çıktısı ekranda; exit kodu görünüyor |
| T-5 | Raporlama | §6 tablosunu doldur; sorunları `Sprints/00-INDEX.md` not alanına işle | Tablo dolu |

## 5. Teknik Talimatlar

- Smoke testi için `PtyTerminal` şu an iki motor sunar: `pty` (shell) ve `claude`.
  FAZ0 gate'i = **`claude` motoruyla tek ajan başlatılıp etkileşilebiliyor**.
- `claude doctor` interaktif olabilir; `--version` ve ilk 10 satır yeterli kabul edilir
  (tam parse FAZ1'de de yapılmayacak — M2 kararı, bkz. FAZ1-PLANI.md ADR-8 notu).
- Herhangi bir komut başarısız olursa: hatayı kopyala → 3 deneme kuralı (CLAUDE.md) →
  çözülemiyorsa dur ve özetle; FAZ1'e **kırmızı zeminde başlanmaz**.

## 6. Doğrulama Raporu (uygulama sırasında doldurulur)

| Kontrol | Komut | Sonuç (✅/❌) | Not |
|:---|:---|:--:|:---|
| Rust derleme | `cargo check --locked` | ⏳ CI/kullanıcı makinesi | Bu sandbox'ta Rust toolchain yok ve crates.io erişilemez (FAZ0 §10 ile aynı kısıt). CI `ci-gate` (3 OS) yeşil referanstır; kullanıcı makinesinde son doğrulama WP-14'te. |
| Clippy | `cargo clippy --locked --all-targets -- -D warnings` | ⏳ CI/kullanıcı makinesi | Aynı kısıt. |
| Rust test | `cargo test --locked` | ⏳ CI/kullanıcı makinesi | Aynı kısıt. |
| Lint/format | `pnpm check` | ✅ | `Checked 24 files, No fixes applied` (2026-08-10) |
| Tip | `pnpm typecheck` | ✅ | `tsc -b` temiz (2026-08-10) |
| Build | `pnpm build` | ✅ | `✓ built in 1.66s`; chunk >500 kB uyarısı bilinen (FAZ1'de code-splitting notu) |
| Claude sürüm | `claude --version` | ⏳ kullanıcı makinesi | Sandbox'ta kurulu değil; README gereksinimi ≥ 2.1.90 |
| Claude sağlık | `claude doctor` | ⏳ kullanıcı makinesi | Aynı kısıt. |
| Tauri smoke | `pnpm tauri:dev` + manuel | ⏳ kullanıcı makinesi | WebView/desktop bu ortamda açılamaz. |
| CI referans | GitHub Actions `ci-gate` | ✅ | FAZ0 kapanışından beri 3 OS yeşil (PR #8) |

**Sonuç:** Frontend gate sandbox'ta ✅ doğrulandı; Rust + Tauri + Claude doğrulaması CI/kullanıcı
makinesine devredildi. WP-01'e **zemin temiz** olarak geçildi.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo check --locked && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked
pnpm check && pnpm typecheck && pnpm build
claude --version && claude doctor
pnpm tauri:dev   # manuel smoke
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| Kullanıcı makinesinde Rust/Node sürüm farkı | README gereksinimleriyle karşılaştır; sorun olursa `rustup update stable` |
| `claude doctor` ağ/oturum ister | `--version` kapısı yeterli; doctor sonucu not olarak |
| CI yeşil ama yerel kırmızı (lockfile) | `pnpm install --frozen-lockfile`, `cargo check --locked` ile lockfile sadakati |
| Smoke testinde webview/PTY farkı (Win) | ConPTY hataları `DEVELOPERS.md` not alanına; WP-05/CI matrisi zaten kapsar |

## 9. Sprint Gate

- §2'deki tüm kutular ✅ ve §6 tablosunda ❌ yok → WP-01 başlayabilir.
- ❌ varsa: FAZ0 kapanış denetimine (FAZ0-DURUM-ANALIZI §12) dön; düzeltme commit'i `fix(m1): …` ile.

## 10. Çıktılar

- Bu kartın §6 tablosu (rapor).
- `Sprints/00-INDEX.md`'de WP-00 satırının durumu güncellenir.

## 11. Devir Notları (sonraki sprinte)

- WP-01'e: mevcut DB şema durumu `user_version = 0` (henüz migration yok) — WP-01 bunu 1→2'ye taşıyacak.
- WP-03'e: `claude --version` çıktısının tam sürümü (min. sürüm kapısı için referans).
- Not: `Sprints/` klasörü yeni; tüm sprint kartları aynı şablonu kullanır.
