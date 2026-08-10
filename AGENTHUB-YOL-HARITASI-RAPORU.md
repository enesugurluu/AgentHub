# AgentHub: Mevcut Durum ve AjanOfis Entegrasyon Raporu

## 1. Giriş
Bu rapor, `AgentHub` adlı mevcut Tauri/Rust tabanlı projenin güncel durumunu analiz etmekte ve sağlanan `AJANOFIS-GITHUB-REPO-ONERILERI-RAPORU.md` belgesindeki tavsiyelerin (AjanOfis AAA Masaüstü Uygulaması vizyonu) bu kod tabanına en doğru, güvenli ve performanslı şekilde nasıl entegre edilebileceğini detaylandırmaktadır.

## 2. AgentHub'ın Mevcut Durumu Analizi
Mevcut `AgentHub` reposunu incelediğimizde:
- **Teknoloji Yığını:** Proje başarıyla Tauri v2, React 19, Vite ve TypeScript ile başlatılmış (`Faz 0` iskeleti kurulmuş).
- **Core (Çekirdek) Yapı:** `src-tauri` altında `portable-pty` crate'i kullanılarak bir Terminal/PTY motoru inşa edilmeye başlanmış.
- **EngineAdapter Mimarisi:** `EngineAdapter` trait'i ve `EngineAdapterRegistry` oluşturulmuş. Bu, `AJANOFIS` raporundaki `Faz 2`'de bahsedilen çoklu motor (Multi-engine) ve `AgentAdapter` trait vizyonuna birebir uyan, sağlam bir altyapı (pluggable PTY engine architecture).
- **Frontend İskeleti:** `xterm.js` ve eklentileri (`xterm-addon-fit`) `package.json` içinde tanımlanmış, terminal arayüzünün temelleri atılmış.
- **Güvenlik & İzolasyon:** Windows'ta `windows-sys` üzerinden "Job Objects" ile child process izolasyonu sağlanmış. `agenthub-worktrees` mantığı projenin bellek notlarında (Agent worktree paths must be resolved securely) belirtilmiş.

**Özetle:** AgentHub şu anda `AJANOFIS` yol haritasının `Faz 0`'ını büyük ölçüde tamamlamış ve `Faz 1/Faz 2`'nin çekirdek PTY süreç izolasyonu temellerini kusursuz bir mimari ile (trait bazlı registry sistemi) atmış durumdadır.

## 3. AjanOfis Raporu Işığında Uygulama Stratejisi ve Yol Haritası

Mevcut repoyu AjanOfis hedefine ulaştırmak için tavsiye edilen GitHub araçlarının kademeli entegrasyonu aşağıdaki gibi olmalıdır:

### 3.1. Faz 0 & 1 Eksiklerinin Tamamlanması (Frontend ve State)
Şu anda backend tarafı çok iyi kurgulanmış olsa da frontend tarafında state yönetimi ve UI bileşenleri geliştirilmelidir.
- **Zustand ve Shadcn UI:** Arayüz bileşenlerini hızlıca ayağa kaldırmak için `shadcn/ui` ve global state için `zustand` projeye eklenmelidir.
- **Xterm WebGL:** `package.json`'da sadece `xterm` var, 10K+ satır loglarda çökmemesi için `xterm-addon-webgl` acilen projeye dahil edilip PTY panellerine bağlanmalıdır (Tavsiye: `xtermjs/xterm.js` MUST maddesi).
- **SQLite + WAL Modu:** Ajan event'leri ve task'lar için `rusqlite` projenin `src-tauri/Cargo.toml` dosyasına eklenmeli ve Tauri'nin state management sistemine bağlanmalıdır.
- **Biomejs:** Pre-commit hook'ları ve hızlı lint/format işlemleri için `biome` kullanılmalıdır.

### 3.2. Faz 2: Çoklu Ajan ve Worktree İzolasyonu
Mevcut `EngineAdapter` yapısı harika bir temeldir. Bunu CLI orkestrasyonuna bağlamalıyız.
- **AgentAdapter Genişlemesi:** `EngineAdapter`'ı temel alan veya genişleten bir yapı ile, sadece PTY sürecini değil; Claude Code, Cursor, Aider gibi CLI araçlarının durumunu parse edebilen bir "Orkestratör" (`awslabs/cli-agent-orchestrator` referanslı) mantığı kurulmalıdır.
- **Git Worktree İzolasyonu:** Backend tarafında izole git worktree'leri oluşturmak için `.git/agenthub-worktrees` dizini altında güvenli (secure path resolution kullanılarak) yönetim sağlanmalıdır.

### 3.3. Faz 3: CEO Onay Mekanizması ve MCP (Model Context Protocol)
- **Güvenlik ve Onay Katmanı:** Tauri'nin channel'ları (IPC) kullanılarak backend'deki child process'ten gelen "komut çalıştırma" talepleri yakalanıp (intercept), frontend'de "CEO (Kullanıcı) Onayı" diyaloglarına dönüştürülmelidir. Yıkıcı komutları (rm -rf vb.) yakalamak için `HardyKrustacean/destructive_command_guard` benzeri Rust temelli regex korumaları (guard) kurulabilir.
- **rmcp (Rust MCP SDK):** Ajanların yerel sistem araçlarına güvenli erişimi için `rmcp` kütüphanesi (v3+) entegre edilmelidir. AgentHub kendisi bir MCP server gibi davranarak yetkilendirdiği dosyalara erişim vermelidir.

### 3.4. Faz 4: Bilgi Grafı (Knowledge Graph) ve Vektör Veritabanı
Gelecekte ajanların ofis hafızasını (memory) paylaşabilmesi için:
- **sqlite-vec ve sqlite-knowledge-graph:** `rusqlite` üzerine bu iki modül eklenerek tek bir `.db` dosyasında ajanların RAG (Retrieval-Augmented Generation) yapabilmesi sağlanmalıdır.
- **Frontend Grafiği:** `sigma.js` ve `@react-sigma/core` React frontend'e entegre edilerek, `AgentHub` içinde "Bilgi Grafı" ve "Bağımlılık" (Toplantı) görünümleri oluşturulmalıdır.
- **Local Embedding:** Tamamen Rust tabanlı, C++ bağımlılığı olmadan embedding üretmek için `huggingface/candle` veya performansı ön planda tutan `ort` crate'i ile ONNX modelleri kullanılabilir.

### 3.5. Faz 5: A2A (Agent-to-Agent) İletişimi
Birden fazla ajan örneği (örneğin CTO ajanı ile Developer ajanı) eş zamanlı çalıştırıldığında, aralarında haberleşmeleri için `a2a-protocol-sdk` entegre edilmelidir. PTY çıktılarını regex ile parse etmek yerine yapılandırılmış (structured) A2A mesajlarıyla görev devri (handoff) yapılmalıdır.

## 4. Pratik "Nerede Nasıl Kullanılır" Özet Tablosu

| Önerilen Paket/Araç | AgentHub İçindeki Yeri | Nasıl Kullanılmalı? |
| :--- | :--- | :--- |
| `portable-pty` | Zaten mevcut | `EngineAdapter` içinde PTY stream'lerini işletmek sistemi olarak (var olduğu gibi). |
| `rusqlite` + `sqlite-vec` | `src-tauri/src/db` (Yeni Klasör) | Ajan taskları, audit logları ve vektörel hafızası için. App state içine enjekte edilecek. |
| `rmcp` (MCP SDK) | `src-tauri/src/mcp` (Yeni Klasör) | PTY üzerinden çalışan ajanlara (client), güvenli tool (araç) sağlamak için bir dahili server katmanı olarak kullanılacak. |
| `awslabs/cli-agent-orchestrator` | Referans Mimari | Hangi ajan türü nasıl tespit edilir algoritmaları için incelenecek ve `EngineAdapterRegistry`'ye adapte edilecek. |
| `sigma.js` & `react-sigma-v2` | `src/components/GraphView` | Frontend'de `sqlite-knowledge-graph` verilerini görselleştirmek (Agent hiyerarşisi/hafıza) için kullanılacak. |
| `@dnd-kit/core` | `src/components/Kanban` | Ajanlara sürükle-bırak ile görev atamak için UI katmanında Kanban Board oluşturulurken. |
| `tauri-plugin-*` (Resmi) | `src-tauri/Cargo.toml` | `tauri-plugin-dialog`, `tauri-plugin-shell`, `tauri-plugin-fs` vb. dosya sistemi onayları, ajanların süreç izinleri için yetkilendirilip (capabilities) eklenecek. |

## 5. Sonuç ve Öneriler
`AgentHub` projesi halihazırda son derece doğru bir temel üzerine (Tauri 2, React 19, Rust `portable-pty` + Windows Job Objects) oturtulmuştur. `EngineAdapter` mimarisi, AjanOfis'in ihtiyaç duyduğu esnek çoklu-ajan orkestrasyonunu destekleyecek sağlamlıktadır.

Mevcut repo geliştirilirken en dikkat edilmesi gereken temel husus; PTY oturumlarının `executionId` ile izole edilmesini sağlayan mevcut yaklaşımın (hafıza ve state sızıntısını önleme) korunması ve geri dönük uyumluluk için arayüzlere yeni metotlar eklenirken (örn: policy/bütçe) default implementasyonların kullanılmasıdır. `AJANOFIS` raporundaki `MUST` seviyeli bağımlılıkları, sıfırdan kurmak yerine mevcut `EngineAdapterRegistry` üzerinden modüler şekilde projeye dahil etmek vizyona giden en güvenli ve temiz yol olacaktır.