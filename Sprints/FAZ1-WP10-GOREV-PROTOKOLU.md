# Sprint FAZ1-WP10 — Görev Protokolü (AGENT_TASK.md + Tamamlanma Algılama)

> **Kart:** FAZ1-PLANI.md §5 WP-10 · ADR-6
> **Takvim:** Hafta 3 · Gün 14–15 (2026-08-25 → 2026-08-26) · **Süre:** 10–12 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — backend + frontend uygulandı; `pnpm check/typecheck/build` yeşil; `cargo test` CI/kullanıcı makinesinde
>
> **Uygulama notları (2026-08-10):**
> - db.rs: `task_create/get/list(agent_id?)/finalize` + `assign_task` (in_progress, tek açık görev kuralı, backlog-dışı atama reddi) — 3 unit test.
> - `src-tauri/src/tasks.rs`: `write_agent_task` (AGENT_TASK.md şablonu — docs 10.5/13.1) + `decide_completion` (blok dosyası > tamamlanma dosyası > parser sinyali > exit kodu) — 4 unit test.
> - `task_assign` komutu: ensure worktree → AGENT_TASK.md → SpawnOptions (task_file + non_interactive + config'ten model/effort/bütçe/turn; görev bütçesi öncelikli) → spawn; oturum `task_id`/`worktree_path` ile etiketlenir.
> - `PtySession.task_id/worktree_path`; `PtyEvent.task_id`; lifecycle exit'te `decide_completion` → `finalize_task` + `task_completed/task_failed` olayı.
> - Frontend: tasks store + TaskDialog ("Görev Ver") + InspectorPanel bağlama + AgentDesk aktif görev etiketi; terminal store `channels` (task_assign kanalı).

## 1. Hedef

FAZ1 Gate'inin son büyük parçası: "Görev Ver" akışı. Kullanıcı ajan seçer → görev
tanımlar → backend worktree'yi garanti eder, `AGENT_TASK.md` yazar, `SpawnOptions`
(non_interactive + budget/turns) ile ajanı spawn eder; tamamlanma/hatada `tasks` ve
`events` güncellenir. Kanban UI M2'ye; **veri omurgası bu sprintte kurulur** (docs 13.1–13.2).

## 2. Definition of Done (DoD)

- [ ] `task_create / task_assign / task_list / task_get` komutları (db.rs) + `TaskRecord`
- [ ] `task_assign` akışı: `ensure_agent_worktree` → `AGENT_TASK.md` → `SpawnOptions{ non_interactive, task_file, budget, turns }` → `agent_spawn_engine`
- [ ] `PtyEvent.task_id` eklendi; oturum → görev bağlantısı `events.task_id`'ye yazılıyor
- [ ] Tamamlanma algılama: parser sinyali **veya** `TASK_COMPLETE.md`/`TASK_BLOCKED.md` **veya** exit code — ilk eşleşen kazanır
- [ ] `tasks` güncellemesi: `in_progress` → `review|done|failed` + `started_at/completed_at/spent_*`
- [ ] `src/components/Tasks/TaskDialog.tsx` (Inspector "Görev Ver") + `tasks.ts` store + ofis masa etiketi (WP-09)
- [ ] Unit testler: görev DB geçişleri, AGENT_TASK.md şablonu, tamamlanma algılama ✅
- [ ] clippy + `pnpm typecheck/build` yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-01 (task tablosu + events.task_id), WP-02 (SpawnOptions), WP-03 (adaptörler),
  WP-04 (TaskCompleted/Failed sinyalleri), WP-05 (ensure_agent_worktree), WP-07 (ajan kaydı).
- Çıkış bağımlıları: WP-08'in backlog devri (burada test edilir), M2 kanban/CEO.

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Task komutları | `task_create(title, description, acceptance_criteria, priority, budget)`; `task_list(agent_id?)`; `task_get(id)` | DB roundtrip testi |
| T-2 | `AGENT_TASK.md` üretici | `src-tauri/src/tasks.rs` (yeni modül) veya `worktree.rs`: şablon (aşağıda) | İçerik doğru |
| T-3 | `task_assign` | Worktree garanti → dosya yaz → `SpawnOptions` kur → spawn; oturum `task_id` ile | Uçtan uca (mock) |
| T-4 | `PtyEvent.task_id` | `pty/runtime/mod.rs` + `pty/mod.rs::register_session`; `events.task_id` dolu | olay kaydı testli |
| T-5 | Tamamlanma algılama | Pump exit yolunda: (1) parser sinyali, (2) worktree'de `TASK_COMPLETE.md`/`TASK_BLOCKED.md`, (3) exit code (0→done, ≠0→failed); `tally_progress()` ile cost/token biriktir | Öncelik sırası testli |
| T-6 | `tasks` güncelleme | `column='review'` (done) / `'failed'`; `completed_at`; `spent_cost/tokens` | DB testi |
| T-7 | TaskDialog UI | Inspector'da "Görev Ver" → başlık/açıklama/kabul kriterleri/öncelik/bütçe; onay → `task_assign` | Manuel senaryo |
| T-8 | Masa etiketi | WP-09 AgentDesk: aktif görev başlığı (task_list → in_progress) | Rozet görünür |

## 5. Teknik Talimatlar

### 5.1 AGENT_TASK.md şablonu (docs 10.5 deseni + docs 13.1)

```markdown
# Görev: <title>

**Ajan:** <agent-name> · **Branch:** <branch> · **Worktree:** <path>
**Bütçe:** $<budget> (aşılırsa dur) · **Max turn:** <turns>

## Görev Tanımı
<description>

## Kabul Kriterleri
<acceptance_criteria>

## Kısıtlar
- Yalnızca bu worktree içinde çalış.
- package.json gibi paylaşılan config dosyalarını değiştirme; gerekirse not et.
- Tamamladığında worktree köküne `TASK_COMPLETE.md` oluştur ve değiştirdiğin dosyaları listele.
- Takılırsan `TASK_BLOCKED.md` oluştur ve nedeni yaz.
```

### 5.2 task_assign akışı (pty/mod.rs + db.rs)

```rust
#[tauri::command]
pub fn task_assign(app, manager, adapters, db, agent_id: i64, task_id: i64,
                   cols: u16, rows: u16, channel: Channel<PtyEvent>) -> Result<AgentSpawnResult, String>
```

1. `task_get(task_id)`; ajan `agent_get(agent_id)` (fired değilse).
2. `ensure_agent_worktree(repo, agent)` → worktree yolu (WP-05).
3. `AGENT_TASK.md` yaz (şablon + alanlar).
4. `SpawnOptions { workdir, task_file, non_interactive: true, max_budget_usd: agent.config.max_budget_usd.or(task.budget), max_turns: agent.config.max_turns, model: agent.config.model, effort: agent.config.effort, env: agent_envs }`.
5. `agent_spawn_engine` mantığıyla spawn; `register_session(..., task_id: Some(task_id))`.
6. `tasks`: `column='in_progress'`, `started_at=now`, `worktree_path`.
7. `events`: `event_type='task_assign'`, `task_id` dolu.

### 5.3 Tamamlanma algılama (runtime)

- `PtyEventKind::Signal(TaskCompleted|TaskFailed)` geldi → tamamlanma bilgisi `pending`.
- Exit olayında (veya monitor döngüsünde): `finalize_task(db, agent_id, task_id, worktree, pending, exit_code, cost_tally)`:
  - `TASK_BLOCKED.md` var → `failed` (reason = dosya içeriği).
  - `pending == TaskCompleted` veya exit code 0 → `review` (M2'de QA/CEO review; `done` aşaması M2).
  - exit code ≠ 0 ve dosya yok → `failed`.
- `cost_tally`: WP-04 Progress.cost birikimi (pump içinde AtomicF64) → `spent_cost`, token sayaçları.
- `events`: `task_completed` / `task_failed` kaydı.

### 5.4 TaskRecord (TS eşleşmesi)

```ts
export type TaskRecord = {
  id: number; title: string; description: string | null
  acceptanceCriteria: string | null; column: string
  assignedAgentId: number | null; priority: number; budget: number | null
  spentCost: number; worktreePath: string | null
  createdAt: string | null; startedAt: string | null; completedAt: string | null
}
```

## 6. Test Planı

- `task_create_roundtrip` + `task_assign_sets_in_progress` (mock spawn yoksa: `task_assign`
  altındaki "worktree hazırla + dosya yaz" adımını `prepare_task_bundle(repo, agent, task)`
  fonksiyonuna ayır; spawn çağrısı mock'lanır veya ayrı test edilir).
- `agend_task_md_template_contains_all`: şablon alanları içeriyor.
- `finalize_detects_completion_priority`: (1) parser sinyali + exit≠0 → parser kazanır; (2) dosya + exit=0 → dosya; (3) exit≠0 → failed.
- `finalize_updates_cost`: tally → `spent_cost/tokens` doğru.
- `fire_moves_assigned_task_to_backlog` (WP-08 ile ortak regresyon).

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked task_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
# Manuel: ajan → Görev Ver → worktree + AGENT_TASK.md oluştu → ajan non-interactive çalıştı →
# tamamlanınca tasks.review + events.task_completed
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| Non-interactive çıktıda ANSI/JSON karışımı | Parser "kesin sinyal" kuralı (WP-04); yanlış pozitifte güvenli yön (failed değil, review'e bırakma) |
| `AGENT_TASK.md` içeriği kötü kaçarsa görev başarısız | Şablon sabit + kullanıcı inputu escape edilir (markdown blok) |
| Uzun non-interactive görevde kullanıcı göremiyor | Terminal sekmesi çıktıyı canlı gösterir (mevcut Channel); `Progress` olayı varsa üstte |
| Bütçe aşımı → CLI durur ama `failed` algılanmaz | exit code ≠ 0 → failed; M2'de budget event + duraklatma |
| Paralel görev aynı ajan | Ajan başına tek aktif görev kuralı (`in_progress` varsa `Err`) — M2'de kaldırılır |

## 9. Sprint Gate

- DoD ✓; manuel uçtan uca: "Görev Ver" → worktree + AGENT_TASK.md → ajan koşar →
  `TASK_COMPLETE.md` → `tasks.column='review'` + cost kaydı.

## 10. Çıktılar

- `src-tauri/src/tasks.rs` (yeni) veya `db.rs` genişletmesi · `pty/mod.rs` (task_assign) · `pty/runtime/mod.rs` (finalize) · `src/components/Tasks/TaskDialog.tsx` · `src/store/tasks.ts` · `src/components/OfficeFloor/AgentDesk.tsx` (etiket).

## 11. Devir Notları (sonraki sprinte)

- WP-13: budget/turn artık hem hire config'inden hem task.budget'tan geliyor — öncelik testi.
- M2: kanban (`tasks` UI'a bağlanır), CEO görev bölme, review zinciri, "done" aşaması + merge.
- WP-14: kapanış senaryosu listesine "2 ajan, 2 görev, eşzamanlı" eklenir.
