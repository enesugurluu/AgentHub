# Sprint FAZ1-WP12 — Sağlık Paneli Genişletme ve Motor Kurulum Akışı

> **Kart:** FAZ1-PLANI.md §5 WP-12 · ADR-3
> **Takvim:** Hafta 3 · Gün 15 (plan: 2026-08-26) · **Süre:** 4 sa · **Öncelik:** P1
> **Durum:** ⏳ Planlandı

## 1. Hedef

Ayarlar "Motorlar" sekmesini gerçek bir sağlık/operasyon paneline dönüştürmek: her
adaptör için kurulu mu / sürüm / capability / sağlık rozeti + `install_hint` ve onaylı
**Kur** akışı (docs 7.5). Kurulum, backend'in çözdüğü komutla ayrı bir terminal sekmesinde
akarken kullanıcı süreci izler (FAZ0 S5 korunur: frontend program göndermez).

## 2. Definition of Done (DoD)

- [ ] `useEngineRegistry` hook'u genişletildi: `detected`, `version`, `capabilities`, `installHint`, yenileme
- [ ] SettingsDialog "Motorlar": her adaptör kartı (id, engine_type, sürüm, capability rozetleri, durum rozeti)
- [ ] Kurulmamış adaptörde "Kur" butonu (hint + onay) → `agent_install_engine`
- [ ] Kurulum oturumu terminal sekmesinde akar (`install-<engine>` agentId, engine `pty`); bitince durum yenilenir
- [ ] `agent_install_engine` hata/kurulu-mu kontrolleri testli
- [ ] "Hızlı Terminal" (repo kökünde shell) aksiyonu — yalnızca bu panelden, bilinçli istisna (WP-05 notu)
- [ ] `pnpm typecheck/build` + clippy yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-03 (`agent_install_engine`, `install_hint`), WP-07 (`useEngineRegistry` ilk hâli).
- Çıkış bağımlıları: WP-07 Adım 2 "Kur" bağlantısı, WP-14 kapanış senaryosu.

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Hook genişletme | `useEngineRegistry`: `{ id, metadata, detectInfo, health }` + `refresh()` + `installing` durumu | Tek kaynak |
| T-2 | Adaptör kartı | SettingsDialog'da `EngineCard` bileşeni (rozetler, hint, Kur butonu) | Okunaklı |
| T-3 | Kur onayı | "Kur" → AlertDialog onay (komut önizlemesi: `installHint`) → `agent_install_engine` | Onay gerekli |
| T-4 | Kurulum sekmesi | `TerminalTabs` `install-<engine>` agentId'sini listeye alır (isim: "Kurulum: <engine>"); `PtyTerminal`'de ajan DB'de yoksa `engine:'pty'` ve oturum yalnız kurulum | Sekme görünür |
| T-5 | Durum yenileme | Kurulum exit → `useEngineRegistry.refresh()`; adaptör detect olursa rozet güncellenir | Canlı güncelleme |
| T-6 | Hızlı Terminal | Ayarlar'dan "Hızlı Terminal Aç" → repo kökünde shell oturumu (bilinçli istisna, worktree yok) | Çalışıyor |
| T-7 | Testler | `agent_install_engine`: kurulu motor → Err; bilinmeyen → Err; bilinen → SpawnResult | Backend testi |

## 5. Teknik Talimatlar

### 5.1 EngineCard veri akışı

```ts
type EngineInfo = {
  id: string
  metadata: EngineMetadata | null          // engineType, version, capabilities
  detectInfo: DetectResult | null          // detected, installHint
  installing: boolean
}
```

- SettingsDialog açılışında `refresh()`; kurulum exit'inde otomatik yenileme.
- `install_hint`'i WP-03 `DetectResult`'tan oku; `metadata` eski alanlar + hint.

### 5.2 Kurulum terminal sekmesi

- `agent_install_engine` (WP-03) `agent_id = "install-<engine_type>"` döner; frontend
  `terminalStore.startSession("install-<engine>", "pty")` + `agentSpawnEngine` yerine doğrudan
  `agentInstallEngine` (yeni ipc sarmalayıcı) çağırır.
- Sekme başlığı: "Kurulum: codex" — `TerminalTabs.agentName`'e özel durum.
- Kurulum komutu `install_command()` adaptörde tanımlı (ör. `bash -lc "npm i -g @openai/codex"`);
  frontend bu komutu **görmez**, yalnızca hint'i gösterir (onay metni için).

### 5.3 Hızlı Terminal (bilinçli istisna)

- WP-05 workdir kuralının tek istisnası: ayarlardan açılan shell oturumu repo kökünde çalışır
  (worktree'siz). `agent_spawn`'a `allow_repo_root: true` opsiyonel parametre eklenir
  (varsayılan false) — worktree kuralı varsayılan olarak korunur.

## 6. Test Planı

- `install_engine_already_detected`: detect true → `Err("zaten kurulu")`.
- `install_engine_unknown_type`: `Err("bilinmeyen engine")`.
- `install_engine_ok`: sahte `install_command`'lı test adaptörü → SpawnResult döner.
- Frontend: manuel — codex kurulu değilse "Kur" → onay → sekmede npm çıktısı → biter → rozet güncellendi.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked install_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # Settings → Motorlar → kurulum senaryosu
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| Kurulum komutu interaktif (OTP/onay) ister | Native installer'lar non-interactive değilse dokümantasyon; çıktı terminalde izlenir |
| Kurulum başarısız ama process exit 0 (kısmi) | Sonrasında `refresh()` detect'i kontrol eder; rozet yine kırmızı kalır |
| İki kurulum aynı anda | `installing` kilidi (hook state) — aynı motor için ikinci tıklama engellenir |
| S5 ihlali (frontend program gönderimi) | `agent_install_engine` yalnız adaptörün `install_command()`'ını kullanır; `agent_spawn` frontend program'ı hâlâ alır ama yalnız "Hızlı Terminal" senaryosunda kullanılır (dokümante) |

## 9. Sprint Gate

- DoD ✓; kurulum akışı uçtan uca (Settings → onay → terminal → rozet) çalışıyor.

## 10. Çıktılar

- `src/hooks/useEngineRegistry.ts` (genişletme) · `src/components/Settings/SettingsDialog.tsx` (+EngineCard) · `src/components/TerminalTabs.tsx` (install sekmesi) · `src/lib/ipc.ts` (`agentInstallEngine`) · `src-tauri/src/pty/mod.rs` (allow_repo_root).

## 11. Devir Notları (sonraki sprinte)

- WP-07: HireWizard Adım 2'de kurulmamış motor → "Ayarlar'da Kur" köprüsü bu sprintin akışını açar.
- WP-14: kapanış senaryosu listesine "kurulu olmayan motoru kur" eklendi.
