## 18. Fark Analizi (Rakiplere Göre)

2026 yılı AI kodlama araçları ekosistemi üç ana kategoriye ayrılıyor. AjanŞirket üçüncü kategoride, kendine özgü konumda yer alıyor:

**Kategori 1 — AI IDE (editör-merkezli):** Cursor, Windsurf (Cognition), Zed, GitHub Copilot (VS Code/JetBrains), VS Code + Cline/Roo/Kilo/Continue
**Kategori 2 — CLI Agent Harness:** Claude Code, Codex CLI, Gemini CLI, OpenCode, Aider
**Kategori 3 — Orkestrasyon/görsel çalışma alanı:** Claude Squad (TUI), Vibe Kanban, Conductor (Melty Labs), Composio Agent Orchestrator, Emdash, Baton, Bernstein, Parallel Code, Nimbalyst, Superset, Capy, Devin (Cognition), Copilot Agent HQ

AjanŞirket Kategori 3'te konumlanıyor; aşağıdaki karşılaştırma tüm kategorilerdeki başlıca oyuncuları kapsar.

### 18.1 Kapsamlı Karşılaştırma Tablosu

| Araç | Tip | Görsel ofis metaforu | Rol hiyerarşisi | Bağlantılı hafıza grafı | Obsidian wiki-not | Hire/fire UX | Kanban | Worktree izolasyon | Çoklu motor | Desktop teknolojisi | Debat/toplantı |
|:---|:---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| **Claude Code** | CLI + Desktop | ❌ | ❌ | Sadece auto-MEMORY.md | ❌ | ❌ | ❌ | ✅ native | Tek (Claude) | Native | ❌ |
| **OpenAI Codex** | CLI + Cloud | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | Sınırlı | Tek (GPT) | Native | ❌ |
| **Gemini CLI** | CLI | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | Sınırlı | Tek (Gemini) | Terminal | ❌ |
| **OpenCode** | CLI TUI | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | Sınırlı | 75+ sağlayıcı | Terminal | ❌ |
| **Aider** | CLI | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | Çoklu | Terminal | ❌ |
| **Cursor** | AI IDE | ❌ | ❌ | Proje bilgi tabanı (lineer) | ❌ | ❌ | Composer temelli | ✅ cloud | Çoklu | Electron (Code fork) | ❌ |
| **Windsurf (Devin Desktop)** | AI IDE | ❌ | ❌ | Flow (kısıtlı) | ❌ | ❌ | Cascade akışı | ✅ cloud | Çoklu | Electron | ❌ |
| **GitHub Copilot** | IDE uzantısı + Agent HQ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ (Agent HQ) | ✅ cloud | Çoklu | IDE + web | ❌ |
| **Cline/Roo/Kilo** | IDE ajanı | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | Çoklu | VS Code/JetBrains | ❌ |
| **Claude Squad** | TUI orkestratör | ❌ | ❌ | ❌ | ❌ | ❌ | Yönetici paneli | ✅ tmux | 6+ CLI | Terminal (Rust) | ❌ |
| **Vibe Kanban** | Web board | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | 10+ | Web app (topluluk) | ❌ |
| **Conductor (Melty)** | Mac app | ❌ | ❌ | ❌ | ❌ | ❌ | Diff list | ✅ | Claude+Codex | Mac native | ❌ |
| **Composio AO** | Web dashboard + CLI | ❌ | Rol tanımı | ❌ | ❌ | ❌ | Milestone gate | ✅ | Çoklu | Web | ❌ |
| **Emdash** | Electron desktop | ❌ | ❌ | ❌ | ❌ | ❌ | Paralel dispatch | ✅ | ~22 CLI | Electron | ❌ |
| **Nimbalyst/Crystal** | Desktop | ❌ | ❌ | Doküman/diyagram/CSV | ❌ | ❌ | ✅ görsel workspace | ✅ | Claude+Codex | Electron | ❌ |
| **Superset** | Desktop workspace | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ 10+ ajan | 10+ | Desktop app | ❌ |
| **Capy** | Hosted dashboard | ❌ | ❌ | ❌ | ❌ | ❌ | Dashboard | ✅ (bulut) | Çoklu | Web | ❌ |
| **Devin (Cognition)** | Cloud agent | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ cloud VM | Tek | Web + Slack | ❌ |
| **Bernstein** | CLI + web | ❌ | ❌ | ❌ | ❌ | ❌ | Deterministik | ✅ | Çoklu | CLI | ❌ |
| **Baton** | CLI/Desktop | ❌ | ❌ | ❌ | ❌ | ❌ | Issue list | ✅ | Çoklu | CLI + Desktop | ❌ |
| **Parallel Code** | Electron | ❌ | ❌ | ❌ | ❌ | ❌ | Çoklu panel | ✅ | Çoklu | Electron | ❌ |
| **AjanŞirket** | Desktop (Tauri) | ✅ **Ofis metaforu** | ✅ **CEO → uzman rolleri** | ✅ **Graf (Sigma+SQLite+vec)** | ✅ **Obsidian vault (wiki-link)** | ✅ **Hire/Fire sihirbazı** | ✅ | ✅ | 10+ ilk dalga | **Tauri 2 + Rust (~5-12 MB)** | ✅ **Debat/toplantı** |

### 18.2 Farklılaşma Eksenleri

1. **Metafor ve duygusal bağlanma:** Hiçbir rakip "şirket" metaforunu kullanmıyor; TUI/diff/board görünümleri kullanıcıları meta düzeyde bırakıyor. Ofis metaforu kullanıcıda "sahiplenme" hissi uyandırır.
2. **Yerel hafıza grafı:** Superset/Vibe Kanban hafıza değil, sadece görev durumu tutuyor. AjanŞirket production-grade sqlite-graph referans mimarisi ile çalışır, Obsidian ile uyumlu wiki-vault kullanır.
3. **Hafif Tauri dağıtımı:** Tüm desktop rakipler Electron (~150 MB, ~170+ MB RAM) kullanıyor; AjanŞirket 5-12 MB, 80 MB RAM altı.
4. **A2A + MCP standartları:** Rakip araçların çoğu kendi iç iletişim protokolünü kullanır; AjanŞirket açık standartları benimser, gelecekte başka araçlarla haberleşebilir.
5. **CEO-merkezli hiyerarşi:** Düz görev listesi yerine yönetim katmanı; CEO strateji kurar, uzmanlar yapar, toplantı modu tartışır. Bu, kullanıcının sorumluluğu "bireysel kodlayıcı"dan "teknik direktör"e taşıyan zihinsel modeldir.
6. **Terminal + Görsel sentez:** Hem gerçek terminal (xterm.js + PTY) hem de soyut görsel (ofis + kanban). Birçok rakip ya yalnız CLI ya da yalnız görsel. AjanŞirket ikisini birleştirir.
7. **Debat/toplantı özelliği:** Birden fazla ajanın aynı problem üzerinde tartışmasını izleme/yönlendirme özelliği diğer araçlarda yok.

### 18.3 Zayıf Yanlar ve Riskler

- **Küçük ekosistem:** Yeni proje; Cursor/Claude Code'un olgunluğuna erişmesi aylar alır.
- **WebView uyumsuzlukları:** Tauri 2'nin avantajı kadar riski de var; özellikle WebKitGTK üzerinde Edge Chromium'da davranan kod kırılabilir. Test matrisi şart.
- **CLI farklılıkları:** Her CLI'nin onay/çıkış/formatı farklı; sağlam parser geliştirme işi yüksektir.
- **Performans:** 12 paralel ajan + bilgi grafı + ofis görünümü iyi bir CPU/RAM kullanımı gerektirir; Tauri bunun için avantajlı olsa da dikkatli optimizasyon gerek.

### 18.4 Sonuç

AjanŞirket, 2026 yılında tam olarak doldurulmamış bir nişe oturuyor: "terminal ajanlarını şirket metaforu ve görsel ofis içinde yöneten, tüm bilgiyi yerel graf hafızada tutan, hafif Tauri tabanlı, açık standartları benimseyen CEO simülatörü". IDE'ler editör, CLI'lar araç, diğer orkestratörler düz liste iken; AjanŞirket bir "yönetim katmanı" sağlıyor.

---
