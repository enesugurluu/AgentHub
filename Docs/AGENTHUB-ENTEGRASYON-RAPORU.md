# AgentHub ve AjanOfis Mimari Entegrasyon Raporu

**Tarih:** 10 Ağustos 2026
**Konu:** Mevcut AgentHub kod tabanının incelenmesi ve hedeflenen "AjanOfis/AjanŞirket" mimarisi (Tauri 2 + Rust + React 19) ile nasıl bütünleştirileceğinin analizi.

## 1. Mevcut Durum Analizi (AgentHub)

Mevcut **AgentHub** reposu incelendiğinde, AjanOfis vizyonunun "Faz 0 ve Faz 1" olarak tanımlayabileceğimiz temel iskeletinin başarıyla kurulduğu görülmektedir:

- **Tauri 2 & React 19 İskeleti:** Proje Vite kullanılarak TypeScript tabanlı bir React frontend ve Rust (Tauri 2) backend ile oluşturulmuştur.
- **PTY Motoru (Portable-Pty):** Terminal etkileşimleri için `portable-pty` crate'i kullanılmış ve `PortablePtyAdapter` adında bir adaptör yazılmıştır. Bu sayede native işletim sistemi PTY'si açılabilmektedir.
- **EngineAdapterRegistry:** Backend'de, AI motorlarının (şu an sadece yerel PTY) kayıt ve takibini yapan, pluggable bir `EngineAdapter` trait'i ve `EngineAdapterRegistry` bulunmaktadır.
- **Git Worktree Yönetimi:** `.git/agenthub-worktrees` altında güvenli klasörler açarak farklı ajan oturumları için izole çalışma alanları sağlayan `worktree.rs` modülü aktiftir.

## 2. AjanOfis Vizyonuna Geçiş ve Açık Kaynak Kütüphane Entegrasyonları

AJANOFIS-GITHUB-REPO-ONERILERI-RAPORU.md dosyasında listelenen 🔴 **MUST** ve 🟠 **STUDY** repoları, mevcut AgentHub mimarisini uçtan uca bir AAA ürüne dönüştürmek için aşağıdaki adımlarla entegre edilmelidir:

### A. Çoklu AI Motor Adaptör Katmanı (AgentAdapter)

Mevcut `EngineAdapter` yapısı, sadece bir komutu PTY'de çalıştırmaya yönelik (low-level) bir yapıdadır. Hedeflenen `AgentAdapter` (07-MOTOR-ADAPTOR-KATMANI referansı) ise; AI CLI'larını (Claude Code, Codex, Aider vb.) algılayan (detect), kuran, `SpawnOptions` (bütçe, workdir, prompt) ile başlatan ve çıktılarını parse eden yüksek seviyeli bir abstraction olmalıdır.

*   **Aksiyon:** `src-tauri/src/pty/adapters/mod.rs` içindeki `EngineAdapter` trait'i, `Docs/ajanofis-buyuk-parcalar/07-MOTOR-ADAPTOR-KATMANI.md` dosyasındaki asenkron `AgentAdapter` yapısına genişletilmelidir.
*   **Önerilen Repolar (STUDY):** `agent-of-empires/agent-of-empires` ve `asheshgoplani/agent-deck` repolarından ajanların (Claude/Aider) çıktılarının nasıl parse edildiği ve onay (approval) akışlarının nasıl yönetildiği (regex yakalama vb.) öğrenilip AgentHub'a entegre edilmelidir.
*   **Event Bus:** Mevcut IPC (invoke) yerine, `tokio::sync::broadcast` ve Tauri Channel (Tauri v2 event sistemi) kullanılarak `AgentEvent` (Spawned, Output, ApprovalRequested, Completed) akışları asenkron olarak React'e basılmalıdır.

### B. Terminal Yönetimi ve UI

Mevcut durumda React içerisinde `PtyTerminal.tsx` bileşeni muhtemelen `xterm.js` ile basit bir entegrasyon içermektedir.

*   **Aksiyon:** Terminal görünümü sağlamlaştırılmalı, React + xterm.js tam performanslı hale getirilmelidir.
*   **Önerilen Repo (MUST):** `xtermjs/xterm.js` eklentilerinden WebGL renderer, fit addon ve search addon kullanılmalıdır.
*   **UI Mimarisi (STUDY & MUST):** UI klasör yapısı `MrLightful/create-tauri-react` modelinde olduğu gibi *Feature-based* olmalıdır. Stil ve komponentler için **Tailwind v4** ve **shadcn/ui** entegre edilmelidir. Durum yönetimi (Ajanlar, Görevler, Çalışan listesi) için global store olarak **Zustand** (`pmndrs/zustand`) tercih edilmelidir.

### C. Hafıza ve Bilgi Grafı (Memory Graph)

Ajanların bağlam (context) kaybetmemesi ve ofisteki diğer ajanların/işlerin geçmişini görebilmesi için yerel veri tabanı entegrasyonu gereklidir.

*   **Aksiyon:** Backend'e `rusqlite` (WAL mode) eklenmeli ve ajanların, görevlerin, event'lerin kalıcılığı sağlanmalıdır.
*   **Önerilen Repo (MUST):** `rusqlite` ve FTS5 eklentisi. Vektör aramalar için `sqlite-vec`. Bilgi grafı (Knowledge Graph) yönetimi için `sqlite-knowledge-graph` kütüphaneleri `Cargo.toml`'a eklenmelidir.
*   **Önerilen Repo (MUST):** Ajan hafızasındaki wiki tarzı notların render edilmesi için frontend'de `@flowershow/remark-wiki-link` (Obsidian tarzı bağlantılar) kullanılmalıdır. Grafın görselleştirilmesi için `Sigma.js` (ve `graphology`) React bileşenlerine entegre edilebilir.

### D. İzolasyon ve MCP (Model Context Protocol)

Mevcut `worktree.rs` modülü dosya sistemi izolasyonunda harika bir başlangıçtır. Bunun ötesine geçilerek Ajanların sistem araçlarına kısıtlı ve standart bir yoldan ulaşması sağlanmalıdır.

*   **Aksiyon:** Ajanların dış dünyayla etkileşimi (veri çekme, sorgu yapma) standart MCP araçlarıyla (Tool Registry) yönetilmelidir.
*   **Önerilen Repo (MUST):** `modelcontextprotocol/rust-sdk` (veya raporlarda önerilen `rmcp`) backend'e eklenerek ajanların MCP protokolüyle yeteneklerini genişletmesi sağlanmalıdır.

## 3. Yol Haritası ve Uygulama Planı

1.  **Faz 0 (Konsolidasyon):**
    *   Mevcut repo `biome` linter ve `husky` ile standartlaştırılmalı.
    *   TailwindCSS v4, shadcn/ui ve Zustand frontend'e eklenip `App.tsx` klasör yapısına bölünmeli.
2.  **Faz 1 (Gelişmiş Adaptör & SQLite):**
    *   `rusqlite` tabanlı `MemoryStore` backend'e eklenmeli.
    *   `EngineAdapter`, asenkron `AgentAdapter` yapısına evrilmeli ve Claude Code adaptörü yazılmalı.
    *   Tauri event sistemi ile PTY stream ve EventBus entegrasyonu tamamlanmalı.
3.  **Faz 2 (Kanban & Worktree Zenginleştirmesi):**
    *   Frontend'e `dnd-kit` eklenerek Kanban board yapılmalı.
    *   Mevcut `worktree.rs` ajan atamaları ve board state'i ile entegre edilmeli.
4.  **Faz 3 (Bilgi Grafı ve MCP):**
    *   `sqlite-vec` ile embedding yeteneği ve `rmcp` ile MCP tool server entegrasyonu yapılmalı.

**Sonuç:** Mevcut AgentHub reposu oldukça sağlam, modern ve Tauri 2 best-practice'lerine uygun temeller barındırmaktadır. Tavsiye edilen GitHub repolarıyla donatıldığında, hedeflenen "AjanOfis" masaüstü uygulamasına giden yolda doğrudan bu repodan ilerlenebilir.
