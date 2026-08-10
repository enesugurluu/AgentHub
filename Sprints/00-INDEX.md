# AgentHub FAZ1 — Sprint Dosyaları (İndeks)

Bu klasör, [`FAZ1-PLANI.md`](../FAZ1-PLANI.md) §5'teki her iş paketi (WP-0 … WP-14) için
ayrı sprint kartı içerir. Her kart; hedef, Definition of Done, görev listesi, teknik
talimatlar, test planı, doğrulama komutları, riskler ve devir notlarını taşır.

## Sprint Takvimi (planlanan; kayma olursa indeks güncellenir)

| Hafta | Tarih aralığı | Sprintler | Teması |
|:--:|:---|:---|:---|
| 1 | 2026-08-10 → 2026-08-14 | WP-00, WP-01, WP-02, WP-03, WP-06 | Veri + motor katmanı |
| 2 | 2026-08-17 → 2026-08-21 | WP-05, WP-07, WP-08, WP-11, WP-09 | İşe alım + ofis |
| 3 | 2026-08-24 → 2026-08-28 | WP-04, WP-10, WP-12, WP-13, WP-14 | Görev + kapanış |

## Sprint Kartları

| Sprint | WP | Başlık | Süre | Öncelik | Bağımlılık | Gate özeti |
|:--|:--:|:---|:--:|:--:|:---|:---|
| [FAZ1-WP00](./FAZ1-WP00-FAZ0-GATE-DOGRULAMASI.md) | 0 | FAZ0 Gate doğrulaması | 0.5 sa | P0 | — | ✅ Frontend gate sandbox'ta yeşil; Rust/CI kullanıcı makinesinde |
| [FAZ1-WP01](./FAZ1-WP01-DB-MIGRATION-VE-AJAN-YASAM-DONGUSU.md) | 1 | DB migration runner + ajan yaşam döngüsü + settings | 4 sa | P0 | WP-00 | ✅ Kod + 9 unit test (cargo doğrulaması CI'da) |
| [FAZ1-WP02](./FAZ1-WP02-SPAWNOPTIONS-VE-ENGINE-SPAWN.md) | 2 | `SpawnOptions` tam hali + `agent_spawn_engine` | 4 sa | P0 | WP-01 (agent_get) | ✅ Kod + golden argv testleri (cargo doğrulaması CI'da) |
| [FAZ1-WP03](./FAZ1-WP03-COKLU-MOTOR-ADAPTORLERI.md) | 3 | codex/gemini/opencode/aider adaptörleri + kurulum | 12–16 sa | P0 | WP-02 | ✅ 6 adaptör registry'de; mock-CLI + golden argv testleri |
| [FAZ1-WP04](./FAZ1-WP04-OUTPUT-PARSER-VE-PROGRESS-EVENT.md) | 4 | OutputParser iskeleti + Progress event | 8 sa | P1 | WP-03 | ✅ 3 parser + Signal event + pump_loop (7 fixture testi) |
| [FAZ1-WP05](./FAZ1-WP05-WORKTREE-OTOMASYONU.md) | 5 | Worktree otomasyonu + `.env.local` + silme seçenekleri | 8 sa | P0 | WP-01, WP-06 | ✅ Repo köküne düşme yok; ensure + port offset + 3 silme aksiyonu |
| [FAZ1-WP06](./FAZ1-WP06-REPO-SECICI-VE-SETTINGS.md) | 6 | Repo seçici + settings kalıcılığı | 4 sa | P0 | WP-01 | ✅ Dialog seçimi kalıcı; TopBar çipi + onboarding |
| [FAZ1-WP07](./FAZ1-WP07-HIRE-WIZARD.md) | 7 | Hire Wizard (3 adım) + preset roller | 10–12 sa | P0 | WP-01, WP-03, WP-06 | ✅ 3 adım + doğrulama; "İşe Al" → DB + seçim; 7 shadcn bileşeni elle |
| [FAZ1-WP08](./FAZ1-WP08-FIRE-AKISI.md) | 8 | Fire onay akışı + Inspector butonu | 4 sa | P0 | WP-07 | ✅ Onay diyaloğu + 3 worktree aksiyonu; store fireAgent |
| [FAZ1-WP09](./FAZ1-WP09-OFIS-KATI-SVG.md) | 9 | Ofis katı SVG v1 (AgentDesk + zoom/pan) | 10 sa | P0 | WP-07 | ✅ SVG zemin + zoom/pan + 6 durum rozeti; seçim → Inspector/terminal |
| [FAZ1-WP10](./FAZ1-WP10-GOREV-PROTOKOLU.md) | 10 | Görev protokolü (`AGENT_TASK.md`) | 10–12 sa | P0 | WP-02, WP-03, WP-04, WP-05 | ✅ task_assign + AGENT_TASK.md + decide_completion + TaskDialog UI |
| [FAZ1-WP11](./FAZ1-WP11-JSONL-VE-SERIALIZE.md) | 11 | JSONL oturum kaydı + serialize | 4 sa | P1 | WP-04 | ✅ transcript.rs + pump/input/exit kayıtları + SerializeAddon buffer persist |
| [FAZ1-WP12](./FAZ1-WP12-SAGLIK-PANELI-VE-KURULUM.md) | 12 | Sağlık paneli + kurulum akışı | 4 sa | P1 | WP-03 | ✅ Motor kartları + onaylı kurulum terminal sekmesinde |
| [FAZ1-WP13](./FAZ1-WP13-BUTCE-VE-COST-TELEMETRI.md) | 13 | Bütçe/effort aktarımı + cost telemetri | 4 sa | P1 | WP-02, WP-04, WP-07 | ✅ supports()+warn; totalCostUsd; CostMeter ≈$ |
| [FAZ1-WP14](./FAZ1-WP14-TEST-CI-DOKUMAN-KAPANIS.md) | 14 | Test/CI/doküman + kapanış denetimi | 6 sa | P0 | Hepsi | ✅ FAZ1-KAPANIS-DENETIMI.md + dokümanlar; elle senaryolar kullanıcı makinesinde |

## Sprint Yürütme Kuralları

1. **Dal/commit:** Tüm FAZ1 işi `arena/019fecb5-agenthub` dalında; commit mesajı
   `feat(m1): <iş>` / `fix(m1): <iş>` / `docs(m1): <iş>` önekiyle İngilizce.
2. **DoD kontrolü:** Her sprint kapanmadan §2 DoD listesindeki her kutu işaretlenmeli;
   §7 doğrulama komutları çalıştırılmış olmalı.
3. **Sıralama:** Bağımlılık sütunundaki sprintler önce kapanır (bkz. FAZ1-PLANI.md §6 grafik).
4. **Sandbox kısıtı:** Bu ortamda Rust toolchain yok → `cargo *` komutları kullanıcı
   makinesinde/CI'da koşulur; her kartta "Doğrulama" bölümü buna göre yazıldı.
5. **Kapanış:** Sprint bittiğinde `Durum` satırı `✅ Kapandı` yapılır; devir notları
   (§11) sonraki sprinte kopyalanır.
6. **Kararlar:** Kod uygulaması sırasında ADR'lerden sapma gerekirse, sapma gerekçesiyle
   FAZ1-PLANI.md ilgili ADR'ye ek not düşülür (ADR geçersiz kılınmaz).
