## 17. MVP Yol Haritası

Toplam süre: ~16 hafta (tek bir geliştirici için); 2 kişilik ekipte ~10 hafta. Fazlar arası kapı (gate) modeli uygulanır; her faz sonunda çalışan bir ürün ve demo hedefi vardır.

### Faz 1: İskelet (2 Hafta) — M0
**Hedef:** Tauri 2.9 + React 19 iskeleti, tek ajan (Claude Code), terminal paneli çalışıyor.

- [ ] Tauri 2 proje iskeleti (Cargo.toml, package.json, Vite + React + Tailwind 4 + shadcn/ui)
- [ ] Ana pencere layout (TopBar + ana grid + alt terminal alanı)
- [ ] SQLite şeması (agents, tasks, events, settings) + ilk migration
- [ ] Claude Code adaptörü: `detect()`, `spawn()`, `stop()`
- [ ] PTY alt yapısı (portable-pty) + Tauri Channel ile frontend'e stream
- [ ] xterm.js WebGL renderer ile ilk terminal paneli çalışıyor
- [ ] İlk "echo" testi: CLI stdin/stdout köprüsü doğrulandı
- [ ] `claude doctor` benzeri sağlık kontrolü

**Gate:** Kullanıcı tek bir Claude Code ajanını terminalden başlatıp etkileşebilir.

### Faz 2: Çoklu Motor ve Worktree (3 Hafta) — M1
**Hedef:** Birden fazla CLI adaptörü, işe alım/çıkarma, otomatik worktree oluşturma.

- [ ] Codex, Gemini, OpenCode, Aider adaptörleri
- [ ] Adaptör kayıt sistemi ve sağlık tespiti
- [ ] `git worktree` yöneticisi (oluşturma, silme, branch atama)
- [ ] İşe alım sihirbazı (Hire Wizard) UI
- [ ] Ajanlar listesi + işten çıkarma akışı (worktree temizleme onayı)
- [ ] Runtime izolasyonu (port offset, .env.local, process group)
- [ ] Ofis katı temel görünümü (SVG, statik masalar, durum rozetleri)

**Gate:** Kullanıcı birden fazla CLI ajanı işe alabilir, her biri kendi worktree'sinde görev alabilir.

### Faz 3: CEO Orkestrasyonu (4 Hafta) — M2
**Hedef:** Kanban, görev dağılımı, onay akışı, politika motoru.

- [ ] Kanban panosu (dnd-kit + react-virtual, WIP limitleri, swimlane)
- [ ] Görev modeli (parent task, öncelik, bütçe, acceptance criteria)
- [ ] CEO orchestrator: görev ayrıştırma, uygun ajana atama, paralel yönetim, senkronizasyon
- [ ] A2A Task lifecycle (submitted/working/input-required/completed/failed/canceled)
- [ ] Onay akışı (regex çözümleme, UI bildirim, allow/deny/edit/always dörtlüsü)
- [ ] Policy engine (regex/lexer tabanlı komut denetimi, varsayılan deny rule)
- [ ] Maliyet/token takibi UI'sı (anlık sayaç + grafik)
- [ ] Tüm ajanları durdurma kill switch (Ctrl/Cmd+Shift+X)

**Gate:** Kullanıcı kanban kartı sürükleyip bir ajan masasına bıraktığında CEO görev bölümlemesi yapar, ajan worktree'de çalışır, onay gerektiğinde kullanıcıya sorar, bitince review akışına alır.

### Faz 4: Bilgi Grafı ve Hafıza (3 Hafta) — M3
**Hedef:** Obsidian-vault, wiki-linkler, indeksleme, graf görünümü, Memory Keeper.

- [ ] Vault klasör yapısı + markdown dosya izleme (notify crate)
- [ ] Wiki-link çözümleyici (remark-wiki-link-plus uyumlu `shortestPossible`)
- [ ] SQLite şeması (entities, edges, FTS5, sqlite-vec)
- [ ] Artımlı indeksleme (değişen dosyaları yeniden işle)
- [ ] Sigma.js ile 2D graf görünümü (WebGL, renk/type, tıklayınca detay)
- [ ] Not editörü (markdown) + backlinks paneli
- [ ] Yerel gömme modeli (all-MiniLM-L6-v2, 22 MB) ile semantic arama
- [ ] MCP sunucusu olarak Knowledge Graph (ajanlar da grafa erişsin)
- [ ] Memory Keeper ajanı (gecelik episodic → semantic/prosedürel damıtma, onaylı)

**Gate:** Kullanıcı not yazabilir, `[[link]]` verebilir, graf görsel olarak inceleyebilir, ajanlar öğrendiklerini grafa ekleyebilir.

### Faz 5: MCP, Entegrasyon ve Cila (3 Hafta) — M4
**Hedef:** MCP Hub, toplantı özelliği, paketleme, ilk sürüm yayın.

- [ ] Resmi Rust MCP SDK `rmcp` entegrasyonu; child-process ve Streamable HTTP
- [ ] Tool registry ve MCP izinleri UI
- [ ] Hazır MCP reçeteleri: GitHub, Linear, Sentry, Playwright, Figma
- [ ] Toplantı (debate) odası UI; çoklu ajanın sırayla konuşması, tutanak kaydı
- [ ] Tema (açık/koyu, tercihe göre ofis/siber görünüm)
- [ ] Import/export ayar, şirket şablonları
- [ ] macOS (imzalı/notarized), Windows (imzalı), Linux (AppImage/deb) paketleme
- [ ] Auto-updater (imzalı), crash reporter, telemetry (opsiyonel)
- [ ] Performans optimizasyonu (6 pencere ≤172 MB hedef)
- [ ] Dokümantasyon, hızlı başlangıç rehberi, video demo

**Gate:** İlk kararlı sürüm (v1.0.0) yayınlanır.

### Sonrası (v1.x+)

- Mobil companion (Tauri 2 iOS/Android): bildirim ve onay
- A2A üzerinden dış ajanlarla (başka kullanıcıların AjanŞirketleri, kurumsal servisler) haberleşme
- Bulut senkronu (opsiyonel, uçtan-uca şifreli)
- Eklenti pazarı (kullanıcı kendi ajan/skill/MCP paketlerini paylaşır)
- Konuşkan sesli asistan (isteğe bağlı)
- "Kurumsal Şirket": çok kullanıcılı, rol tabanlı erişim

---
