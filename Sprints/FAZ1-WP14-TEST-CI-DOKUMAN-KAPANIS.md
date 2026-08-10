# Sprint FAZ1-WP14 — Test/CI/Dokümantasyon ve Kapanış Denetimi

> **Kart:** FAZ1-PLANI.md §5 WP-14 · ADR-3, ADR-9
> **Takvim:** Hafta 3 · Gün 17–18 (2026-08-27 → 2026-08-28) · **Süre:** 6 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — `FAZ1-KAPANIS-DENETIMI.md` + dokümanlar + indeks güncellendi;
> elle senaryo listesi kullanıcı makinesinde koşulacak (raporda ⏳)
>
> **Uygulama notları (2026-08-10):**
> - `FAZ1-KAPANIS-DENETIMI.md` oluşturuldu: 12 kriter tablosu, sprint özeti, 12 maddelik elle
>   senaryo listesi, bilinçli ertelemeler (M2 backlog), sapmalar ve nedenleri.
> - README durum "FAZ1 (M1) 🚧" + plan/sprint/kapanış bağlantıları; DEVELOPERS.md FAZ1 özeti eklendi.
> - Mock-CLI matrisi `agents::test_util::with_fake_binary` altında konsolide (Unix gerçek, Windows no-op).
> - CI: yeni bağımlılıklar lock'ta (`react-zoom-pan-pinch`, `@tauri-apps/plugin-dialog`, 6 Radix);
>   `cargo test --locked` CI'da; elle senaryolar kullanıcı makinesinde.

## 1. Hedef

FAZ1'i kapanışa hazırlamak: mock-CLI test matrisinin konsolidasyonu, CI'nın yeni
komut/bağımlılıklarla yeşil kalması, dokümantasyon (DEVELOPERS/README/index) ve FAZ0
§12 formatında **FAZ1 Kapanış Denetimi** raporu. Bu sprint, FAZ1 Gate'in resmî
doğrulama turudur.

## 2. Definition of Done (DoD)

- [ ] Mock-CLI test matrisi konsolide: her adaptör detect/health/flag testi tek yerde, `#[cfg(unix)]` kurallı
- [ ] CI (3 OS) yeşil: yeni komutlar derleniyor; `pnpm-lock.yaml` güncel (react-zoom-pan-pinch, chrono, dirs)
- [ ] `DEVELOPERS.md`'ye FAZ1 bölümü (yeni modüller, komutlar, mock-CLI deseni)
- [ ] `README.md` güncel: durum "FAZ1 ✅" veya "FAZ1 devam"; yol haritası işaretleri
- [ ] `Sprints/00-INDEX.md` tüm kartların durumu `✅ Kapandı`
- [ ] `FAZ1-KAPANIS-DENETIMI.md` (repo kökü) — FAZ0 §12 formatında, §7 Gate 12 kriteri + bilinçli ertelemeler (M2 backlog)
- [ ] Elle senaryo listesi (kullanıcı makinesinde) çalıştırıldı ve sonuçlar tabloya işlendi

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-00 … WP-13 (hepsi `✅ Kapandı`).
- Çıkış: FAZ1 Gate kararı + M2 köprüsü.

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Test matrisi konsolidasyonu | `src-tauri/src/agents/tests/` (mod içi `#[cfg(test)]`) veya `tests/`; `mock_binary` yardımcısı ortak | Tek tanım |
| T-2 | CI doğrulama | Yeni bağımlılıklar lock'ta; `cargo check --locked` CI'da; frontend build; gerekirse timeout ayarı | 3 OS yeşil |
| T-3 | DEVELOPERS.md | FAZ1 bölümü: SpawnOptions, adaptör ekleme deseni (güncel), transcript, task_assign, mock-CLI | Belgeli |
| T-4 | README | Durum rozeti + yol haritası + Sprints linki | Tutarlı |
| T-5 | İndeks kapanışı | `Sprints/00-INDEX.md` durumları + sapma notları | Güncel |
| T-6 | Kapanış denetimi raporu | `FAZ1-KAPANIS-DENETIMI.md`: Gate 12 kriter tablosu, elle senaryo sonuçları, bilinçli ertelemeler → M2 backlog | FAZ0 §12 formatı |
| T-7 | Elle senaryo listesi | Aşağıdaki §6 senaryoları kullanıcı makinesinde koşulur, sonuçlar raporda | Her satır ✅/❌ |

## 5. Teknik Talimatlar

### 5.1 Kapanış denetimi raporu şablonu (FAZ1-KAPANIS-DENETIMI.md)

```markdown
# AgentHub — FAZ1 Kapanış Denetimi

**Tarih:** <uygulama sonu tarihi> · **Dal:** arena/019fecb5-agenthub

## FAZ1 Gate — 12 kriter (FAZ1-PLANI.md §7)
| # | Kriter | Durum | Kanıt |
|:--:|:---|:--:|:---|
| 1 | 2+ CLI motoru detect + kurulum akışı | | Settings ekran görüntüsü / test |
| … | … | | … |

## Elle Senaryo Sonuçları
| Senaryo | Beklenen | Sonuç | Not |
|:---|:---|:--:|:---|
| … | … | | |

## Bilinçli Ertelemeler (M2 backlog'una yazıldı)
- Onay akışı köprüsü (allow/deny/edit/always) — M2
- Kanban UI + CEO orkestratör — M2
- Maliyet dashboard (settings budget, grafik) — M2
- `claude doctor` tam interaktif parse — M2
- Ofis animasyonları / masa konum kalıcılığı — M2
- Tam `TASK_COMPLETE` → `done` + merge akışı — M2
```

### 5.2 Elle senaryo listesi (kullanıcı makinesi)

1. Settings → Motorlar: 6 adaptör listeleniyor; kurulu olanlar rozetli, `install_hint` görünür.
2. Kurulu olmayan motoru kur (onay → terminal sekmesi → rozet güncellenir).
3. Hire Wizard: preset + özel rol; bütçe 0.5 USD, effort high; "İşe Al" → ofiste masa.
4. İkinci ajan (farklı motor) işe al → eşzamanlı 2 oturum → `pty_list_all_ids` 2 id.
5. Görev Ver (2 ajana) → worktree'ler oluştu, `.env.local` PORT'lar farklı, `AGENT_TASK.md` içeriği doğru.
6. Görev tamamlanınca `tasks.column='review'` + `events.task_completed` + `totalCostUsd` > 0 (claude).
7. TopBar CostMeter "≈ $" artıyor; JSONL dosyaları `~/.agentcompany/logs/` altında.
8. İşten Çıkar → seçenekler (delete) → masa kalktı, worktree silindi, loglar kaldı.
9. `agent_stop` tek ajanı durdurur, diğeri etkilenmez.
10. Repo seçici: dialog → seçim kalıcı; yeniden başlatmada hatırlanıyor.
11. 10K+ satır çıktı (yes komutu gibi) → WebGL terminal akıcı.
12. `.env*` worktree'lerde yok; `git status` temiz (`.agentcompany/` gitignore'lu).

### 5.3 CI notları

- `pnpm-lock.yaml`'a yeni bağımlılıklar eklendi (`react-zoom-pan-pinch`, `chrono`/`dirs` Cargo.lock).
- `cargo test --locked` CI'da mock-CLI testleri `cfg(unix)` olduğu için Linux'ta tam, Win/macOS'ta derlenir (Windows'ta skip mantığı).
- Gerekirse `rust` job timeout'u 40 dk'da kalır (test sayısı arttı).

## 6. Test Planı (bu sprintin kendisi)

- `cargo test --locked` (tümü) + `cargo clippy --locked --all-targets -- -D warnings` + `pnpm check/typecheck/build`.
- CI'da PR açılıp `ci-gate` yeşil görülür.
- Kapanış raporundaki 12 kriter tablosu doldurulur.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # elle senaryo listesi §5.2
git status --short   # sürüklenen dosya yok
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| Bir WP kapalı değilken kapanış denetimi | T-5 indeks durumu kontrolü; kapalı olmayan kart önce kapatılır |
| CI'da Windows mock-CLI eksik | `cfg(unix)` + Windows skip; kapsam dokümante |
| Elle senaryolarda CLI olmaması (codex kurulu değil) | Kurulum akışıyla çözülür; olmuyorsa senaryo "kurulmadı" notuyla işaretlenir ve detect doğru "kurulu değil" diyor |
| Lockfile sürüklenmesi | `--frozen-lockfile` / `--locked` kuralı (CI'da zaten) |

## 9. Sprint Gate (FAZ1 Kapanış)

- FAZ1-PLANI.md §7'deki **12 kriterin tamamı** ✅ (kanıtlı) → FAZ1 resmî olarak kapanır.
- Ertelemeler M2 backlog'una yazılır → FAZ1-PLANI.md §10 güncellenir.

## 10. Çıktılar

- `FAZ1-KAPANIS-DENETIMI.md` (repo kökü) · `Sprints/00-INDEX.md` (kapanış) · `DEVELOPERS.md` · `README.md` · `src-tauri` test matrisi.

## 11. Devir Notları (sonraki milestone)

- M2 planı (FAZ1-PLANI.md §10 köprüsü) bu raporun "Bilinçli Ertelemeler" listesiyle
  başlar; `FAZ2/M2-PLANI.md` ayrı dosya olarak üretilir (FAZ0 §9 deseni).
