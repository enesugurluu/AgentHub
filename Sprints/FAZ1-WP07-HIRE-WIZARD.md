# Sprint FAZ1-WP07 — Hire Wizard (İşe Alım Sihirbazı)

> **Kart:** FAZ1-PLANI.md §5 WP-7 · ADR-1, ADR-10
> **Takvim:** Hafta 2 · Gün 8–9 (2026-08-19 → 2026-08-20) · **Süre:** 10–12 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — UI uygulandı, `pnpm check/typecheck/build` yeşil
>
> **Uygulama notları (2026-08-10):**
> - shadcn CLI `ui.shadcn.com`'a erişemedi (ağ kısıtı) → 7 bileşen elle yazıldı (new-york/data-slot deseni):
>   `select, textarea, switch, alert-dialog, tooltip, dropdown-menu, progress` (+6 Radix paketi).
> - `pty_adapter_detect_info` komutu eklendi (DetectResult → install_hint/detected; WP-12 de kullanır).
> - `src/lib/presets.ts`: docs 6.3 tablosu (9 preset) + avatar renk paleti.
> - `src/hooks/useEngineRegistry.ts`: id + metadata + detectInfo (kurulu mu).
> - `src/components/Settings/HireWizard.tsx`: 3 adım + progress + doğrulama (motor kurulu, bütçe/turn sayısal, isim ≥ 2).
> - `agents` store'una `hireAgent`; `selectVisibleAgents` filtresi; AgentSidebar "+" → HireWizard.

## 1. Hedef

docs 6.1'deki 3 adımlı işe alım sihirbazını kurmak: **Rol → Motor ve Yetenekler →
Uzmanlık ve Kişilik**. "İşe Al" → `agent_hire` → DB kaydı → ofiste masa (WP-09) →
artık görev verilebilir (WP-10). AjanŞirket metaforunun "işe alım" deneyimi FAZ1'de canlanır.

## 2. Definition of Done (DoD)

- [ ] shadcn bileşenleri eklendi: `select`, `textarea`, `switch`, `alert-dialog`, `tooltip`, `dropdown-menu`, `progress`
- [ ] `src/components/Settings/HireWizard.tsx`: 3 adım + ilerleme göstergesi + adım doğrulama
- [ ] Preset roller (docs 6.3 alt kümesi) TS sabiti: CEO, CTO, Backend, Frontend, QA, DevOps, Designer, PM, Memory Keeper
- [ ] Adım 2 motor listesi: `useEngineRegistry`'den kurulu adaptörler + kurulmamışlar hint'li (devre dışı)
- [ ] Adım 3: isim, avatar rengi, sistem promptu, biyografi; validasyon (isim boş olamaz)
- [ ] `agentHire` çağrısı + `agents` store'a `hireAgent`; başarıda seçim + sihirbaz kapanır
- [ ] AgentSidebar "+" butonu sihirbazı açar; `fired` ajanlar listede görünmez
- [ ] `pnpm typecheck/build` yeşil

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-01 (agent_hire), WP-03 (motor listesi), WP-06 (repo yolu — isteğe bağlı uyarı).
- Çıkış bağımlıları: WP-08 (fire), WP-09 (masa), WP-10 (görev), WP-13 (config → spawn).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | shadcn eklemeleri | `pnpm dlx shadcn@latest add select textarea switch alert-dialog tooltip dropdown-menu progress` | `components.json` güncel |
| T-2 | `useEngineRegistry` hook | `src/hooks/useEngineRegistry.ts`: `listEngines()` → id+metadata+detected; yenileme | WP-12 ile paylaşılır |
| T-3 | Preset roller | `src/lib/presets.ts`: docs 6.3 tablosu → `{ id, name, role, motor, effort, permissions, systemPrompt }` | Özel rol de var |
| T-4 | HireWizard (Adım 1) | Preset ızgara seçimi / "Özel rol"; seçim `role` + `systemPrompt` taslağını doldurur | Adımsız geçilemez |
| T-5 | HireWizard (Adım 2) | Motor select (kurulu adaptörler; kurulmamışlar `install_hint` + devre dışı), model (metin/select), effort (5 seviye), bütçe (USD), max turn, izin profili (full/standard/limited/custom) | Validasyon: motor seçili |
| T-6 | HireWizard (Adım 3) | İsim, avatar rengi (renk paleti), sistem prompt textarea, biyografi; "İşe Al" | İsim boş değil |
| T-7 | Store + çağrı | `agents.ts`: `hireAgent(payload)`; `ipc.ts`: `HirePayload` tipi + `agentHire` | Başarı → liste güncel |
| T-8 | Sidebar entegrasyonu | "+" → `HireWizard` (Dialog); `fired` filtresi (`agent.status !== 'fired'` → ayrıca `firedAt` boş) | Sidebar temiz |

## 5. Teknik Talimatlar

### 5.1 Preset rol şeması (src/lib/presets.ts — docs 6.3 özeti)

```ts
export type PermissionProfile = 'full' | 'standard' | 'limited' | 'custom'
export type RolePreset = {
  id: string; name: string; role: string
  motor: string; effort: 'low'|'medium'|'high'|'xhigh'|'max'
  permissions: PermissionProfile
  systemPrompt: string   // görev talimatı taslağı
}
```

| Preset | Motor (varsayılan) | Effort | İzinler |
|:---|:---|:---|:---|
| CEO | claude | max | full |
| CTO | claude | xhigh | standard |
| Backend Dev | codex | medium | standard |
| Frontend Dev | claude | medium | standard |
| QA | aider | low | limited |
| DevOps | claude | high | standard |
| Designer | gemini | medium | limited |
| PM | claude | high | limited |
| Memory Keeper | claude | low | limited |

> Motor seçimi Adım 2'de değiştirilebilir; kurulu değilse preset yine de seçilebilir ama
> motor kurulana kadar "İşe Al" engellenir (veya kuruluma yönlendirilir — WP-12).

### 5.2 HirePayload eşlemesi (WP-01 tipiyle birebir)

```ts
export type HirePayload = {
  name: string; role: string; motor: string
  model?: string | null; effort?: string | null
  maxBudgetUsd?: number | null; maxTurns?: number | null
  permissionsProfile: PermissionProfile
  systemPrompt?: string | null; avatarColor?: string | null
  skills: string[]; mcpServers: string[]
}
```

### 5.3 Adım doğrulama kuralları

- Adım 1: rol seçili (preset veya özel boş değil).
- Adım 2: motor kurulu (`detected === true`); bütçe sayısal ≥ 0; max turn pozitif tam sayı.
- Adım 3: isim ≥ 2 karakter; avatar rengi varsayılan `hsl` paletinden.

### 5.4 Başarı akışı

```ts
await hireAgent(payload)          // store → agentHire
selectAgent(newRecord.id)         // Inspector'da aç
closeWizard()
```

## 6. Test Planı

- Frontend unit framework yok → DoD: `pnpm typecheck/build` + manuel senaryo:
  1) "+" → sihirbaz; 2) Backend Dev preset seç → motor `codex` (kuruluysa); 3) isim "Ayşe",
  renk seç; 4) "İşe Al" → sidebar + ofis kartı göründü; 5) `fired` ajan yokken filtre çalışır.
- Backend (WP-01 testleri zaten var): `agent_hire` validasyonu (motor registry'de mi) —
  bu sprintte `agent_hire`'a registry kontrolü eklenebilir (opsiyonel, `State<EngineAdapterRegistry>`).

## 7. Doğrulama Komutları

```bash
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # manuel sihirbaz senaryosu
cd src-tauri && cargo test --locked   # hire validasyonu eklendiyse
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| Sihirbaz kapsam şişer (MCP, skill editör) | FAZ1: 3 adım sabit; `mcpServers`/`skills` boş dizi gönderilir; düzenleme M2 |
| Preset motoru kurulu değil | Adım 2'de devre dışı + hint; "Kur"a yönlendirme WP-12 |
| Uzun form → kullanıcı bırakır | Adım başına `progress` göstergesi; "Geri" düğmesi; seçimler state'te korunur |
| `fired` ajanların görünmesi | Store filtresi tek yerde (`selectVisibleAgents`) — WP-08 de kullanır |

## 9. Sprint Gate

- DoD ✓; manuel senaryo geçti; `agent_hire` çağrısı DB'de gerçek kayıt üretiyor.

## 10. Çıktılar

- `src/components/Settings/HireWizard.tsx` · `src/lib/presets.ts` · `src/hooks/useEngineRegistry.ts` · `src/store/agents.ts` · `src/lib/ipc.ts` · `src/components/AgentSidebar.tsx` · `src/components/ui/*` (yeni shadcn).

## 11. Devir Notları (sonraki sprinte)

- WP-08: Inspector'daki ajan artık gerçek DB kaydı; fire butonu bu sprintin verisiyle çalışır.
- WP-09: masa ataması `agents` listesinden; `avatarColor` ilk kez kullanılır.
- WP-13: `configJson` hire'dan dolu — spawn eşlemesi için hazır.
- WP-12: "Kur" akışı sihirbazın Adım 2'sine bağlanabilir.
