## 5. Ofis Görünümü (Office Floor UI)

### 5.1 Konsept
AjanŞirket'in ana görünümü, üstten görünüm (top-down) bir ofis katı krokisidir. Her ajan bir masa + avatar + durum etiketiyle temsil edilir. Kullanıcı (CEO) masası merkezde; çalışan masaları etrafta. Kanban panosu solda, toplantı odası sağda, terminal ekranı altta, hafıza grafı arka planda soluk bir "siber-uzay" dekoru olarak.

Bu tasarım benzer orkestrasyon araçlarının (Vibe Kanban kart listesi, Conductor diff listesi) düz iş listesi görünümüne radikal bir alternatiftir; kullanıcıya şirketini yönetme hissi verir.

### 5.2 Yerleşim (Layout)

```
┌──────────────────────────────────────────────────────────────────┐
│  Menü çubuğu: Proje, Ajanlar, Ayarlar, Maliyet: $12.34  ⚙️      │
├────────────┬─────────────────────────────────────┬───────────────┤
│            │                                     │ Toplantı      │
│  KANBAN    │         OFİS KATI (canvas)          │ Odası         │
│  ┌───────┐ │   ┌──────┐  ┌──────┐  ┌──────┐     │  ┌─────────┐ │
│  │ Todo  │ │   │ CTO  │  │Backend│ │ Frontend│    │  │ Debat   │ │
│  │  3 kart│ │   │ 💻   │  │ 🛠️   │ │ 🎨    │     │  │ oturumu │ │
│  ├───────┤ │   └──────┘  └──────┘  └──────┘     │  │ aktif   │ │
│  │In Prog│ │                                     │  └─────────┘ │
│  │  2    │ │   ┌──────┐  ┌──────┐  ┌──────┐     │               │
│  ├───────┤ │   │ QA   │  │DevOps│ │  PM  │      │  Mini graf:   │
│  │Review │ │   │ 🧪   │  │ 🚀   │ │ 📋  │      │  "Şirket      │
│  │ 1     │ │   └──────┘  └──────┘  └──────┘     │   hafızası"   │
│  ├───────┤ │                                     │               │
│  │Done 5 │ │        [CEO Masası — Sen] 👤        ├───────────────┤
│  └───────┘ │         Masa üstünde laptop,         │ Ajan Detay    │
│            │         üzerinde aktif görev kartı    │ (seçili ajan) │
│            │                                     │ Terminal      │
│            │   Kahve makinesi, bitki, raf (decor) │ Butonlar:      │
│            │                                     │ Durdur/İzin  │
├────────────┴─────────────────────────────────────┴───────────────┤
│  Terminal: [ajan stdout akışı — sekmeli]                         │
└──────────────────────────────────────────────────────────────────┘
```

### 5.3 React Bileşen Ağacı

```tsx
<App>
  <TopBar>
    <ProjectSwitcher />
    <CostMeter spent={2.34} budget={50} />
    <GlobalSearch />
    <SettingsButton />
  </TopBar>

  <main className="grid grid-cols-[260px_1fr_280px]">
    <KanbanSidebar>
      <Column status="todo" />
      <Column status="in_progress" />
      <Column status="review" />
      <Column status="done" />
    </KanbanSidebar>

    <OfficeFloor>   {/* SVG/Canvas 2D ofis, zoom pan */}
      <AgentDesk agent={ceo} isPlayer />
      {agents.map(a => <AgentDesk agent={a} />)}
      <MeetingRoom />
      <CoffeeCorner />
      <PlantDecor />
    </OfficeFloor>

    <aside>
      <MeetingRoomPanel />
      <AgentInspector selected={selectedAgent} />
      <MemoryMiniMap />        {/* Sigma.js küçük graf */}
    </aside>
  </main>

  <TerminalTabs>
    <XTerminal session={agent.pty} />
  </TerminalTabs>
</App>
```

### 5.4 Ajan Masası (AgentDesk) Bileşeni

Her masa bir SVG düğümü olarak render edilir. Durumuna göre renk/animasyon değişir:

| Durum | Renk | İkon | Animasyon |
|:---|:---|:---|:---|
| idle | Gri | Kahve fincanı | Hafif sallantı |
| thinking | Mavi | Düşünme baloncuğu | Nefes alma |
| working | Yeşil | Klavye tuşları | Yazma animasyonu |
| blocked | Turuncu | El işareti | Yanıp sönme (onay bekleniyor) |
| error | Kırmızı | Ünlem | Hızlı titreşim |
| meeting | Mor | Konuşma balonu | Dalga halkaları |

Masa tıklandığında sağ paneldeki `AgentInspector` açılır: ajan rolü, açık görevler, istatistikler (tamamlanan görev, harcanan token, başarı oranı), Skill listesi, durdur/yeniden başlat/işten çıkar butonları.

### 5.5 Etkileşimler

- **Sürükle bırak:** Kanban kartını alıp bir ajan masasına bırakmak görevi o ajana atar.
- **Ajanlar arası kart sürükleme:** Bir masadan diğerine kart aktarmak (handoff).
- **Çift tık:** Ajanın terminal sekmesini açar.
- **Sağ tık:** Ajan menüsü (yeteneklerini ayarla, skill ekle, zeka seviyesi, bütçe).
- **Kaydırma/zoom:** Ofis katında yakınlaştırma.
- **Toplantı odasına sürükle:** İki ajanı toplantıya al (debat/beyin fırtınası).
- **Hire butonu:** Yeni ajan yaratma sihirbazı (Bölüm 6).
- **Kahve molası butonu:** Tüm ajanları anlık duraklat.

### 5.6 Render Teknolojisi

- **Ofis katı:** React + SVG (elle çizilmiş basit ofis elemanları) veya düşük poligon canvas sahnesi. 10-20 ajan + dekor için SVG yeterince performanslı ve tema/animasyon dostu.
- **Yakınlaştırma/平移:** `react-zoom-pan-pinch` veya custom SVG viewBox transform.
- **Ajan avatarları:** 64x64 yuvarlak resim veya rol ikonu (CTO için gözlük/silgi, Backend için çekiç/anahtar, Frontend için fırça, QA için büyüteç, DevOps için roket, PM için pano...).
- **Arka plan:** Hafif ızgara deseni + duvar/sütun hatları, gölgeler.
- **Küçük hafıza grafı mini-map:** Sigma.js WebGL, soluk renk, ana panelde tıklanınca büyük grafa geçiş.
- **Tema:** Açık/koyu tema; tercihe göre "gerçekçi ofis" veya "siber-futuristik" görünüm seçimi.

### 5.7 Animasyon ve Mikro-Etkileşim

- Ajan çalışırken masasının üzerinde uçuşan küçük kod parçacıkları (rastgele değişken isimleri) yanar/söner.
- Yeni görev atandığında kağıt uçak animasyonu CEO masasından ilgili masaya kayar.
- Görev bittiğinde küçük konfeti.
- Ajan onay bekliyorsa masasının üzerinde titreşen bir zil ikonu + bildirim sesi (kapatılabilir).
- Boştaki ajanlar ara sıra kahve almak için ayağa kalkar (decoratif).

### 5.8 Erişilebilirlik

- Tüm durum ve renkler ekran okuyucu ile okunabilir (`aria-label`, `role="status"`).
- Klavye kısayolları: Tab ile ajanlar arası gezinme, Space seçme, Enter terminal açma, `ctrl+k` global arama.
- "High-contrast" mod ve büyük font seçenekleri.
- Animasyonlar için `prefers-reduced-motion` desteği.

---
