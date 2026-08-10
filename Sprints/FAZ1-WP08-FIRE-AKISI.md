# Sprint FAZ1-WP08 — Fire Onay Akışı (İşten Çıkarma)

> **Kart:** FAZ1-PLANI.md §5 WP-8 · ADR-1
> **Takvim:** Hafta 2 · Gün 10 (2026-08-21) · **Süre:** 4 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — UI + store akışı uygulandı; `pnpm check/typecheck/build` yeşil
>
> **Uygulama notları (2026-08-10):**
> - `FireDialog` (AlertDialog): açık görevler → backlog (Switch), worktree sil/koru/commit'le-sakla (radio), log saklama (Switch); aktif oturum uyarısı.
> - Store `fireAgent` (agentFire + liste + seçim temizleme); InspectorPanel'de `FireButton` + "Görev Ver" placeholder (WP-10).
> - `Kalıcı Sil` (agent_delete) backend'de hazır; UI yalnızca `fired` kayıtlarda görünür — Inspector görünür ajanlar `fired` içermediği için WP-14'te elle doğrulanır.
> - `selectVisibleAgents` filtresi Inspector'da da kullanıldı.

## 1. Hedef

docs 6.2'deki işten çıkarma deneyimi: Inspector'da "İşten Çıkar" → onay diyaloğu
(açık görevler, worktree seçenekleri, log saklama) → `agent_fire` → ajan pasif/`fired`,
ofis ve listeden kalkar. Worktree davranışı WP-05'teki remove seçeneklerini kullanır.

## 2. Definition of Done (DoD)

- [ ] `src/components/Settings/FireDialog.tsx` (shadcn `AlertDialog` tabanlı)
- [ ] Seçenekler: açık görevler → Backlog'a (checkbox, varsayılan açık) · worktree: sil / koru / commit'le sakla (radio) · konuşma loglarını sakla (varsayılan açık)
- [ ] InspectorPanel'e "İşten Çıkar" (destructive) + "Görev Ver" placeholder (WP-10)
- [ ] `agent_fire` backend davranışı: `status='fired'`, `fired_at`, açık görevler backlog'a, worktree aksiyonu WP-05 fonksiyonlarıyla
- [ ] Store: `fireAgent(id, options)`; sonrasında seçim temizlenir, liste/ofis güncellenir
- [ ] `agent_delete` yalnız `fired` kayıtlar için (Inspector'da "Kalıcı Sil" ikincil)
- [ ] Unit testler: fire geçişleri + worktree seçenekleri ✅; `pnpm typecheck/build` yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-01 (agent_fire), WP-05 (worktree remove seçenekleri), WP-07 (Inspector verisi).
- Çıkış bağımlıları: WP-09 (fired masa kaldırma), WP-10 (açık görevlerin devri).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | FireDialog bileşeni | AlertDialog; onay metni ajana göre; seçenek grubu | Görsel onay akışı |
| T-2 | Backend bağlama | `agent_fire(id, FireOptions)` → DB geçişi + worktree aksiyonu (WP-05) | İki koldan testli |
| T-3 | Store eylemi | `fireAgent(id, opts)`; başarıda `selectAgent(null)` + `fetchAgents` | State temiz |
| T-4 | Inspector butonu | "İşten Çıkar" (destructive) + çalışan oturum varsa uyarı; "Görev Ver" (WP-10 placeholder) | Aksiyonlar görünür |
| T-5 | Kalıcı sil | `agent_delete` "fired" için; FireDialog'da ikincil düğme | Onay ikinci kez istenir |
| T-6 | Görünürlük | `selectVisibleAgents` filtresi (`firedAt` boş) sidebar+ofis+inspector'da ortak | Tek filtre kaynağı |

## 5. Teknik Talimatlar

### 5.1 FireOptions (WP-01'de tanımlı, burada doldurulur)

```ts
export type FireOptions = {
  worktreeAction: 'delete' | 'keep' | 'commit_and_keep'
  moveOpenTasksToBacklog: boolean
  keepLogs: boolean          // JSONL log dizinini silme (varsayılan true)
}
```

- Backend akışı (`agent_fire`):
  1. Ajanı `status='fired'`, `fired_at=now` yap.
  2. `move_open_tasks_to_backlog`: `UPDATE tasks SET column='backlog', assigned_agent_id=NULL
     WHERE assigned_agent_id=? AND column IN ('todo','in_progress','review')`.
  3. `worktreeAction`: `delete` → `worktree_remove(path, force:false)`; `keep` → metadata sil,
     dizin kalır; `commit_and_keep` → WP-05 `commit_and_keep_worktree`.
  4. `keepLogs == false` ise `~/.agentcompany/logs/<slug>` temizlenir (WP-11 dizini).
  5. `events`'e `fire` kaydı.

### 5.2 Çalışan oturum uyarısı

- Ajanın aktif PTY oturumu varsa (`terminalStore.sessions[id]` running): FireDialog uyarı
  gösterir; onayda önce `agentStop` çağrılır (mevcut `agent_stop`), sonra fire.

### 5.3 Kalıcı sil

- `agent_delete` yalnızca `fired` kayıtlar: DB'de ajan + ilişkili `events.agent_id` satırları
  (FK olduğu için önce events, sonra ajan; worktree zaten fire'da halledildi).
- Inspector'da "Kalıcı Sil" yalnız `fired` ajanlar için görünür.

## 6. Test Planı

- `fire_moves_open_tasks_to_backlog`: todo/in_progress görevler → backlog + atama NULL; done dokunulmaz.
- `fire_worktree_delete`: temp repo + worktree → fire(delete) → dizin yok.
- `fire_worktree_commit_and_keep`: kirli worktree → fire(commit_and_keep) → commit var, dizin duruyor, metadata yok.
- `fire_keep_logs_false`: sahte log dizini silinir; `true` ise kalır.
- `delete_only_fired`: aktif ajan → Err; fired → Ok + events temiz.
- Frontend: manuel — işe al → görev ver (WP-10 sonrası) → işten çıkar → onay akışı.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked fire_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # manuel: Inspector → İşten Çıkar → seçenekler → ajan ofisten kalktı
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| FK ihlali (events.agent_id) | Kalıcı silmede önce events temizlenir; fire ise FK'ya dokunmaz (kayıt audit için kalır) |
| Yanlışlıkla aktif ajan silme | `agent_delete` yalnız fired; fire zaten onay diyaloğu |
| Worktree silme başarısız (kirli) | Hata UI'da gösterilir; `keep`'e düşme önerisi; `force:false` korunur |
| Açık görevler kaybolur | Varsayılan checkbox backlog'a taşıma açık; done'a dokunulmaz |

## 9. Sprint Gate

- DoD ✓; fire akışı uçtan uca (DB + worktree + UI) çalışıyor; `fired` ajan listede/ofiste yok.

## 10. Çıktılar

- `src/components/Settings/FireDialog.tsx` · `src/components/InspectorPanel.tsx` · `src/store/agents.ts` · `src-tauri/src/db.rs` (agent_fire/delete tamamlanması) · `src-tauri/src/worktree.rs` (bağlama).

## 11. Devir Notları (sonraki sprinte)

- WP-09: `selectVisibleAgents` filtresi ofis katında da kullanılır.
- WP-10: "Görev Ver" placeholder'ı gerçek TaskDialog ile değişir; fire'ın backlog devri bu veriyle test edilir.
- WP-14: kapanış denetiminde fire senaryosu elle tekrar koşulur.
