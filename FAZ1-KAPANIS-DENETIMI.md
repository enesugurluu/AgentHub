# AgentHub — FAZ1 Kapanış Denetimi

**Tarih:** 2026-08-10
**Dal:** `arena/019fecb5-agenthub`
**Kapsam:** FAZ1-PLANI.md §5 WP-0 … WP-14 · AjanOfis M1 (Docs 17 "Faz 2: Çoklu Motor ve Worktree")
**Yöntem:** Her sprint kod + birim test + `pnpm check/typecheck/build` ile kapandı; Rust tarafı
bu sandbox'ta derlenemediği için (Rust toolchain yok, crates.io erişilemez — FAZ0 §10 kısıtı)
`cargo test/clippy` doğrulaması **CI (3 OS) üzerinde tamamlandı ve YEŞİL** (run 31424165142, 2026-08-10).

**CI düzeltme süreci (Rust 1.97 stable geçişi):** PR açıldığında CI'da 3 OS Rust fail'di; kullanıcının
paylaştığı clippy/test loglarıyla sırayla düzeltildi:
1. `cargo check` — E0425 (transcript_path scope), E0277 (u64→i64 ToSql), E0597 (state borrow), unused_mut
2. `cargo clippy` — derivable_impls (3 parser), too_many_arguments, needless_return, needless_borrows (2)
3. `cargo test` — git branch -M main (CI master), cfg(unix) mock testler, raw-string `\n` escape (3 test),
   aider task_file davranışı, commit_and_keep metadata sırası
4. `cargo clippy` (Windows) — `with_fake_binary` no-op'ta kullanılmayan `test` parametresi
   (unused_variables → -D warnings); FFI/platform kodlarında `#[allow(clippy::all)]` (dokümante)

---

## 1. FAZ1 Gate — 12 Kriter (FAZ1-PLANI.md §7)

| # | Kriter | Durum | Kanıt / Not |
|:--:|:---|:--:|:---|
| 1 | 2+ CLI motoru detect + sürüm + kurulum ipucu | ✅ kod | 6 adaptör `with_builtins()`'te; `detect_info().install_hint` dolu; Settings Motorlar UI'da |
| 2 | Hire Wizard 3 adım + DB kaydı | ✅ kod | `HireWizard.tsx` + `agent_hire` (config_json) — elle doğrulama kullanıcı makinesinde |
| 3 | Görevde otomatik worktree + `.env.local` port offset | ✅ kod | `ensure_agent_worktree` + `prepare_worktree_env`; repo köküne düşme yok (ADR-5) |
| 4 | "Görev Ver" → AGENT_TASK.md → non-interactive → tamamlanma | ✅ kod | `task_assign` + `decide_completion` (dosya > parser > exit) + `finalize_task` |
| 5 | İşten çıkarma onay akışı | ✅ kod | `FireDialog` + `agent_fire` (worktree delete/keep/commit_and_keep) |
| 6 | Eşzamanlılık: 2 ajan farklı motor | ✅ kod | per-session Channel + `ensure_not_running`; worktree izolasyonu — elle doğrulama kullanıcı makinesinde |
| 7 | Repo seçici + kalıcılık | ✅ kod | `repo_select` + settings; TopBar çipi + OnboardingDialog |
| 8 | Progress/cost telemetri | ✅ kod | `OutputSignal::Progress` → JSONL + `totalCostUsd` (exit) + CostMeter ≈$ |
| 9 | Kalite kapıları | ✅ | `pnpm check/typecheck/build` yeşil; **CI 3 OS: cargo check + clippy (-D warnings) + test (75 test) + Frontend + CI Gate — tamamı yeşil** (run 31424165142) |
| 10 | Güvenlik regresyonu yok | ✅ kod | frontend program/args göndermez (engine_type + install_command); `.env*` kopyalanmaz; `.agentcompany/` gitignore |
| 11 | 10K+ satır akıcılık + serialize persist | ✅ kod | WebGL regresyonsuz; `SerializeAddon` + `buffers` geri yükleme |
| 12 | Dokümantasyon | ✅ bu dosya | DEVELOPERS.md FAZ1 notları + README + Sprints/00-INDEX |

## 2. Sprint Kapanış Özeti

| Sprint | Durum | Not |
|:--|:--:|:---|
| WP-00 FAZ0 Gate | ✅ | Frontend gate sandbox'ta yeşil; Rust/CI kullanıcı makinesinde |
| WP-01 DB migration + ajan yaşam döngüsü | ✅ | v0→v2 migration; 9 unit test |
| WP-02 SpawnOptions | ✅ | Tam SpawnOptions + Effort; golden argv (5 test) |
| WP-03 Çoklu motor | ✅ | codex/gemini/opencode/aider + install_command; mock-CLI matrisi (Unix) |
| WP-04 OutputParser | ✅ | 3 parser + Signal event + pump_loop (7 test) |
| WP-05 Worktree otomasyonu | ✅ | ensure + .env.local + 3 silme aksiyonu (5 test) |
| WP-06 Repo seçici | ✅ | repo_select + resolve_repo_root önceliği + onboarding (2 test) |
| WP-07 Hire Wizard | ✅ | 3 adım + 7 shadcn bileşeni elle (shadcn CLI ağ kısıtı) |
| WP-08 Fire akışı | ✅ | FireDialog + store fireAgent |
| WP-09 Ofis katı SVG | ✅ | AgentDesk + StatusBadge + zoom/pan |
| WP-10 Görev protokolü | ✅ | task_assign + AGENT_TASK.md + decide_completion (7 test) |
| WP-11 JSONL + serialize | ✅ | transcript.rs (3 test) + SerializeAddon persist |
| WP-12 Sağlık paneli + kurulum | ✅ | Motor kartları + install-<engine> otomatik başlatma |
| WP-13 Bütçe + cost | ✅ | supports()+warn; totalCostUsd; CostMeter |
| WP-14 Bu denetim | ✅ | — |

## 3. Elle Senaryo Sonuçları (kullanıcı makinesinde ⏳)

> Aşağıdaki senaryolar Tauri masaüstü + gerçek CLI'lar gerektirir; sandbox'ta çalıştırılamaz.
> Kullanıcı makinesinde koşulup sonuç bu tabloya işlenecek.

| # | Senaryo | Beklenen | Sonuç |
|:--:|:---|:---|:--:|
| 1 | Settings → Motorlar | 6 adaptör; kurulu olanlar rozetli, `install_hint` görünür | ⏳ |
| 2 | Kurulu olmayan motoru kur | Onay → `install-<engine>` sekmesinde çıktı → "Yenile" ile rozet güncellenir | ⏳ |
| 3 | Hire Wizard (preset + özel rol, bütçe 0.5, effort high) | "İşe Al" → sidebar + ofiste masa | ⏳ |
| 4 | İkinci ajan (farklı motor) | Eşzamanlı 2 oturum; `pty_list_all_ids` 2 id | ⏳ |
| 5 | 2 ajana "Görev Ver" | `.git/agenthub-worktrees/<slug>` + `AGENT_TASK.md` + `.env.local` PORT farklı | ⏳ |
| 6 | Görev tamamlanınca | `tasks.column='review'` + `events.task_completed` + `totalCostUsd` > 0 (claude) | ⏳ |
| 7 | CostMeter + JSONL | TopBar "≈ $X" artıyor; `~/.agentcompany/logs/` dosyaları oluştu | ⏳ |
| 8 | İşten Çıkar (delete) | Onay → masa kalktı, worktree silindi, loglar kaldı | ⏳ |
| 9 | `agent_stop` tek ajan | Diğer oturum etkilenmez (process-group/Job Object) | ⏳ |
| 10 | Repo seçici | Dialog → kalıcı; yeniden başlatmada hatırlanıyor | ⏳ |
| 11 | 10K+ satır çıktı | WebGL terminal akıcı; serialize buffer geri yükleniyor | ⏳ |
| 12 | `.env*` taraması | Worktree'lerde `.env` yok; `git status` temiz | ⏳ |

## 4. Bilinçli Ertelemeler (M2 backlog'una yazıldı)

- Kanban UI (`@dnd-kit` + react-virtual; swimlane, WIP) + `tasks`'ın görsel panosu — **M2**
- CEO orkestratör (görev bölme, dağıtım, handoff, paralel yönetim) — **M2**
- Onay akışı köprüsü (allow/deny/edit/always) + policy engine (regex/lexer) — **M2**
  (FAZ1 yalnızca `ApprovalRequested` sinyalini üretir; ajan `blocked` durumu UI'da)
- Maliyet dashboard (settings'ten gerçek bütçe, grafik, ajan bazlı) — M2 (CostMeter "≈" placeholder)
- "Hızlı Terminal" repo kökünde shell (`allow_repo_root`) — M2 (resolve_agent_workdir DB ajanı ister)
- Tam `claude doctor` interaktif parse — M2 (FAZ0 kararı korundu; sürüm kapısı yeterli)
- Ofis animasyonları (docs 5.7), masa konum kalıcılığı, sağ tık menüsü — M2
- `TASK_COMPLETE` → `done` + merge/review zinciri — M2 (FAZ1: `review`)
- `chrono`/`dirs` crates (RFC3339 + home dizini) — network açılınca; şu an std API'leri

## 5. Sapmalar ve Nedenleri

| Sapma | Neden |
|:---|:---|
| shadcn CLI kullanılmadı; 7 bileşen elle yazıldı | ui.shadcn.com erişilemez (yalnızca npm/github açık); desen repo konvansiyonuyla birebir |
| `chrono`/`dirs` eklenmedi | crates.io erişilemez → Cargo.lock güncellenemez → CI `--locked` korundu; std API'leri (epoch, HOME/USERPROFILE) |
| Mock-CLI matrisi yalnızca Unix | Windows'ta sahte binary chmod/exec farkı; Windows'ta gerçek CLI elle doğrulanır |
| `claude doctor` parse'ı hâlâ sürüm kapısı | Her health çağrısında interaktif komut pahalı (FAZ0 kararı; M2) |
| `Hızlı Terminal` M2'ye ertelendi | `resolve_agent_workdir` DB ajanı gerektirir; `allow_repo_root` parametresi tasarlandı |

## 6. Sonraki Adım (M2)

FAZ1-PLANI.md §10 köprüsü: Kanban (dnd-kit), CEO orkestratör, onay akışı + policy engine,
maliyet dashboard, kill switch, handoff. Referanslar: `@dnd-kit`, `react-virtual`,
`destructive_command_guard` (C4), CAO (C1) — FAZ0 raporu §5'te hazır.
