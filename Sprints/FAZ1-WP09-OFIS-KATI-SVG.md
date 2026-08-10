# Sprint FAZ1-WP09 — Ofis Katı SVG v1 (AgentDesk + StatusBadge + Zoom/Pan)

> **Kart:** FAZ1-PLANI.md §5 WP-9 · ADR-10, docs 5.4–5.6
> **Takvim:** Hafta 2 · Gün 11–12 (2026-08-20 → 2026-08-21) · **Süre:** 10 sa · **Öncelik:** P0
> **Durum:** ✅ Kapandı — UI uygulandı; `pnpm check/typecheck/build` yeşil
>
> **Uygulama notları (2026-08-10):**
> - `react-zoom-pan-pinch@4` eklendi; `TransformWrapper` + `ZoomControls` (zoom/pan/sıfırla).
> - `StatusBadge`: 6 durum (docs 5.4) — renk + lucide ikon + animasyon sınıfı; `resolveDeskStatus` (oturum → durum); `motion-reduce` desteği.
> - `AgentDesk`: HTML kart (SVG zeminde) — avatar (avatarColor), isim, rol, rozet; klavye erişilebilir (Enter/Space, aria-label).
> - `OfficeFloor`: SVG zemin (ızgara + dekor), CEO masası merkez, deterministik masa grid'i (3 sütun), boş ofis CTA; `fired` filtresi.
> - Tıklama → `selectAgent` + `setActive` (terminal sekmesi).

## 1. Hedef

Statik masa kartları yerine docs 5'teki **interaktif ofis katı**: SVG çizim, her ajana
masa + avatar + durum rozeti (6 durum, docs 5.4), zoom/pan, tıklama → Inspector/terminal.
FAZ1 gate'inin görsel kimliği bu sprintte gelir; M2'de animasyon/dekor katmanı eklenir.

## 2. Definition of Done (DoD)

- [ ] `react-zoom-pan-pinch` eklendi (sürüm pinli)
- [ ] `src/components/OfficeFloor/` klasörü: `OfficeFloor.tsx` (SVG sahne), `AgentDesk.tsx`, `StatusBadge.tsx`
- [ ] SVG: zemin ızgarası, CEO masası (merkez), çalışan masaları (ızgara konumları), dekor (kahve köşesi, bitki, raf)
- [ ] `StatusBadge`: docs 5.4 6 durum (idle/thinking/working/blocked/error/meeting) + renk + ikon + `aria-label`
- [ ] Zoom/pan (klavye + fare); `prefers-reduced-motion` destekli
- [ ] Masa tıklama → `selectAgent` + `setActive` (terminal sekmesi açılır)
- [ ] `fired` ajanlar çizilmez; boş ofiste "İşe Al" CTA'sı
- [ ] `pnpm typecheck/build` yeşil; manuel zoom/pan testi ✅

## 3. Ön Koşullar ve Bağımlılıklar

- Giriş: WP-07 (gerçek ajan listesi + avatarColor), WP-08 (fired filtresi — `selectVisibleAgents`).
- Çıkış bağımlıları: WP-10 (masada aktif görev rozeti), M2 (animasyon, sürükle-bırak).

## 4. Görev Listesi

| # | Görev | Detay | Kabul |
|:--:|:---|:---|:---|
| T-1 | Bağımlılık | `pnpm add react-zoom-pan-pinch` | Lockfile güncel |
| T-2 | StatusBadge | 6 durum eşlemesi (docs 5.4 tablosu) — renk/ikon/animasyon sınıfı | ARIA okunabilir |
| T-3 | AgentDesk | SVG masa (dikdörtgen + ekran), avatar dairesi (renk = avatarColor), isim, StatusBadge, "aktif görev" etiketi (WP-10 hazır olduğunda) | Tıklanabilir |
| T-4 | OfficeFloor SVG | `viewBox 1000×640`; ızgara deseni; CEO masası merkez; çalışan masaları ızgara koordinatları; dekor | Tema uyumlu (CSS değişkenleri) |
| T-5 | Zoom/pan | `react-zoom-pan-pinch` sarmalayıcı; araç çubuğu (+/−/sıfırla); `motion-reduce:transition-none` | Klavye erişilebilir |
| T-6 | Etkileşim | Tıklama → select+active; sağ tık menüsü (M2'ye not); boş ofis CTA | Akış testi |
| T-7 | Erişilebilirlik | `role="button"`, `tabIndex`, `aria-label`; durumlar `role="status"` bölgesinde | Lighthouse benzeri kontrol |

## 5. Teknik Talimatlar

### 5.1 Durum eşlemesi (docs 5.4)

| Durum | Renk (sınıf) | İkon (lucide) | Animasyon |
|:---|:---|:---|:---|
| idle | gri (`text-muted-foreground`) | Coffee | — |
| thinking | mavi (`text-blue-400`) | Brain | breathe |
| working | yeşil (`text-emerald-400`) | Keyboard | blink |
| blocked | turuncu (`text-amber-500`) | Hand | pulse |
| error | kırmızı (`text-red-500`) | AlertTriangle | shake |
| meeting | mor (`text-purple-400`) | MessagesSquare | rings |

- `status` kaynağı: `terminalStore.sessions[id]?.status` (canlı) → yoksa `agent.status` (DB).
- Sıra (öncelik): `running→working`, `thinking→thinking`, `waiting→blocked`, `error→error`,
  `exited→idle` (özel: exit sonrası "son görev bitti" — M2'de `done` rozeti).

### 5.2 Masa konumlandırma (deterministik)

- Çalışan masaları: 3 sütun × N satır grid; koordinatlar `(x, y)` = `f(index)` sabit fonksiyon
  (store'da konum yok — M2'de `desk_position` alanı eklenebilir).
- CEO masası: `(500, 320)` merkez, kullanıcı (sen).
- Dekor: kahve köşesi sol üst, bitki sağ alt, raf üst — statik `<rect>`/`<path>` grupları.

### 5.3 Zoom/pan

```tsx
import { TransformWrapper, TransformComponent } from 'react-zoom-pan-pinch'
<TransformWrapper initialScale={1} minScale={0.5} maxScale={2.5} doubleClick={{ disabled: true }}>
  <TransformComponent wrapperClass="h-full w-full" contentClass="h-full w-full">
    <svg viewBox="0 0 1000 640" className="h-full w-full">…</svg>
  </TransformComponent>
</TransformWrapper>
```

- Çift tıklama M2'de terminal açma olacak (docs 5.5) — FAZ1'de tıklama seçim.
- Klavye: `+`/`-`/`0` zoom; ok tuşları pan (wrapper API'si).

### 5.4 Ofis → terminal köprüsü

```ts
const openDesk = (agentId: number) => { selectAgent(agentId); setActive(String(agentId)) }
```

## 6. Test Planı

- Frontend unit framework yok → manuel senaryo:
  1) 3 ajan → masalar farklı konumlarda; 2) ajan spawn → `working` rozeti canlı değişir;
  3) tıklama → Inspector + terminal sekmesi; 4) zoom/pan + sıfırla; 5) fire → masa kaybolur;
  6) `prefers-reduced-motion` altında animasyon yok.
- `pnpm typecheck/build` gate.

## 7. Doğrulama Komutları

```bash
pnpm check && pnpm typecheck && pnpm build
pnpm tauri:dev   # manuel ofis senaryosu
```

## 8. Riskler ve Önlemler

| Risk | Önlem |
|:---|:---|
| SVG büyük ajan sayısında karmaşıklaşır | 10-20 ajan hedefi (docs 5.6); grid fonksiyonu; M2'de sanallaştırma gerekmez |
| `react-zoom-pan-pinch` sürüm uyumu (React 19) | Sürüm pin + `pnpm ls`; sorun olursa custom viewBox transform (yedek) |
| WebView farkları (WKWebView SVG) | Basit SVG primitifleri; test matrisi CI'da derleme + elle 3 OS |
| Animasyonlar rahatsız eder | `motion-reduce` + tercih ayarı (settings) |

## 9. Sprint Gate

- DoD ✓; 6 durumdan en az 3'ü canlı senaryoda doğrulandı (working/blocked/idle); tıklama akışı çalışıyor.

## 10. Çıktılar

- `src/components/OfficeFloor/` (OfficeFloor, AgentDesk, StatusBadge) · `package.json` (+react-zoom-pan-pinch) · `src/store/agents.ts` (filtre kullanımı).

## 11. Devir Notları (sonraki sprinte)

- WP-10: masa üzerinde aktif görev etiketi (TaskDialog'dan gelen `task.title`).
- M2 notu: sürükle-bırak (kanban → masa), sağ tık menüsü, animasyonlar (docs 5.7), masa konumlarını kalıcı yap.
