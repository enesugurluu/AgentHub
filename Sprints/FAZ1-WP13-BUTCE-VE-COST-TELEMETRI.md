# Sprint FAZ1-WP13 — Bütçe/Effort Aktarımı ve Cost Telemetri İskeleti

> **Kart:** FAZ1-PLANI.md §5 WP-13 · ADR-2, ADR-4
> **Takvim:** Hafta 3 · Gün 16 (2026-08-27) · **Süre:** 4 sa · **Öncelik:** P1
> **Durum:** ✅ Kapandı — backend + frontend uygulandı; `pnpm check/typecheck/build` yeşil
>
> **Uygulama notları (2026-08-10):**
> - `EngineAdapter::supports(feature)` default'u (capability listesinden); `agent_spawn_engine`
>   ve `task_assign`'de desteklenmeyen budget/turns/effort için `tracing::warn!`.
> - Pump: `Progress.cost` birikimi (Mutex<f64>) → exit payload + JSONL `totalCostUsd`.
> - config → SpawnOptions akışı WP-10 `task_assign`'de inline kuruldu (görev bütçesi öncelikli).
> - Frontend: session `totalCostUsd` + `addCost`; TopBar CostMeter "≈ $X / $50" (M2'de gerçek dashboard).

## 1. Hedef

Hire Wizard'da seçilen değerlerin (model/effort/bütçe/turn) gerçekten CLI'ya ulaştığını
garanti etmek ve maliyet telemetrisini görünür kılmak: `Progress.cost` birikimi → TopBar
CostMeter'da "≈ $X" (salt-okunur; gerçek dashboard M2). FAZ0'ın "SpawnOptions tam hali"
ertesindeki **uçtan uca değer akışı** bu sprintte kapanır.

## 2. Definition of Done (DoD)

- [ ] `agent_spawn_engine` / `task_assign`: options alanları boşsa ajan `config_json`'dan doldurulur (model, effort, budget, turns)
- [ ] Her adaptörün `build_<engine>_command` golden testlerinde budget/turn/effort doğru flag'ler
- [ ] Budget/turn desteklemeyen motorlarda capability filtresi + `tracing::warn!` (sessiz yok sayma değil, görünür)
- [ ] Pump: `Progress.cost` birikimi (`AtomicF64`); exit olayında `events` payload'ına `totalCostUsd`
- [ ] Terminal store: `session.totalCostUsd` birikimi; TopBar CostMeter "≈ $<sum> / $50" (placeholder budget sabiti — M2'de settings)
- [ ] JSONL'a Progress satırları zaten yazılıyor (WP-11) — doğrulandı
- [ ] clippy + `pnpm typecheck/build` yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-01 (config_json), WP-02 (SpawnOptions), WP-04 (Progress), WP-07 (hire değerleri), WP-10 (task.budget), WP-11 (JSONL).
- Çıkış bağımlıları: M2 maliyet dashboard'ı (bu sprint veri omurgasını bırakır).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Config → SpawnOptions | `pty/mod.rs`: `options_from_agent(db, agent, overrides) -> SpawnOptions` (config_json parse; task_assign override'ları öncelikli) | Öncelik testi |
| T-2 | Capability filtresi | Adaptör `supports("budget")` değilse `options.max_budget_usd` yok sayılır + `tracing::warn!` | Log + test |
| T-3 | Golden flag genişletme | Tüm adaptörler: budget/turn/effort kombinasyonları (destekleyenler için) | Golden tablo |
| T-4 | Cost birikimi | Pump'ta `AtomicF64` totalCost; exit payload `{ executionId, code, outputBytes, totalCostUsd }` | events testi |
| T-5 | Store + TopBar | `session.totalCostUsd`; CostMeter `≈ $sum` (0.01 yuvarlama); araç ipucu "M2'de tam dashboard" | Canlı güncelleme |
| T-6 | JSONL doğrulama | WP-11 Progress satırları `cost` içeriyor | Fixture |

## 5. Teknik Talimatlar

### 5.1 options_from_agent (öncelik sırası)

```rust
fn options_from_agent(db: &AppDb, agent: &AgentRecord, overrides: SpawnOptions) -> SpawnOptions {
    let cfg: AgentConfig = serde_json::from_str(agent.config_json.as_deref().unwrap_or("{}"))
        .unwrap_or_default();
    // Sıra: açık override (task_assign) > ajan config > varsayılan
    // model:      overrides.model.or(cfg.model)
    // effort:     overrides.effort.or(cfg.effort)
    // budget:     overrides.max_budget_usd.or(cfg.max_budget_usd)
    // turns:      overrides.max_turns.or(cfg.max_turns)
}
```

- `task_assign` çağrısında `task.budget` ajan config'ini **ezmez** mi? Karar: görev bütçesi
  önceliklidir (docs 8.2 "Tahmini token/maliyet"); açıkça belgele.
- `AgentConfig` serde yapısı WP-01 `HirePayload.config_json` ile birebir.

### 5.2 Capability kontrolü

- `EngineAdapter`'a `fn supports(&self, feature: &str) -> bool { self.metadata().capabilities.iter().any(|c| c == feature) }` default'u eklenir.
- `spawn_cli` içinde: `opts.max_budget_usd.is_some() && !self.supports("budget")` →
  `tracing::warn!(adapter = self.id(), "budget unsupported, ignoring")`.

### 5.3 CostMeter (TopBar)

```tsx
const total = Object.values(sessions).reduce((acc, s) => acc + (s.totalCostUsd ?? 0), 0)
<span className="tabular-nums text-muted-foreground">≈ ${total.toFixed(2)}</span>
```

- Budget sabiti `$50` placeholder (FAZ0'daki gibi); M2'de `settings`'ten okunur.

## 6. Test Planı

- `options_from_agent_priority`: override > config > default.
- `budget_unsupported_warns`: `supports("budget")==false` adaptör + budget dolu → warn log (test `tracing` capture veya return değeri üzerinden).
- `golden_flags_all_adapters`: her adaptörün budget/turn/effort flag üretimi (destekleyenler).
- `pump_accumulates_cost`: sahte Progress akışı → exit payload'da `totalCostUsd` doğru.
- Frontend: manuel — claude görevi çalışırken Progress olayı → CostMeter artıyor.

## 7. Doğrulama Komutları

```bash
cd src-tauri && cargo test --locked cost_ && cargo clippy --locked --all-targets -- -D warnings
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # manuel: hire'da budget 0.5 → görev → claude --max-budget-usd 0.5 argv'de (log)
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| Cost yalnızca stream-json'da var (claude/opencode) | Diğer motorlarda `Progress.cost=0`; sayaç "≈" (yaklaşık) etiketiyle dürüst |
| Budget iki kaynaktan (config/task) | Öncelik testi + dokümantasyon |
| `config_json` şeması değişirse parse kırılır | `unwrap_or_default()`; hata loglanır, spawn engellenmez |
| TopBar her Progress'te render | Zustand seçici (`useShallow`) ile sadece toplam değişince render |

## 9. Sprint Gate

- DoD ✓; hire'da girilen bütçe gerçekten CLI argv'sinde (golden + manuel log); CostMeter canlı.

## 10. Çıktılar

- `src-tauri/src/pty/mod.rs` (options_from_agent) · `pty/adapters/mod.rs` (supports) · `pty/runtime/mod.rs` (cost birikimi) · `src-tauri/src/agents/*.rs` (golden testler) · `src/components/TopBar.tsx` · `src/store/terminal.ts`.

## 11. Devir Notları (sonraki sprinte)

- WP-14: kapanış denetiminde "budget aktarımı" senaryosu.
- M2: maliyet dashboard (settings'ten budget, grafik, ajan bazlı); batch API ipuçları (docs 11).
