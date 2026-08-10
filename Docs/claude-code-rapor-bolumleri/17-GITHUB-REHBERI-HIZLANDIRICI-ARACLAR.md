# CLAUDE CODE WORKFLOW HIZLANDIRICI GITHUB REHBERİ

**Tarih:** 2026-08-09
**Araştırma:** 4 paralel ajan ile tarama: Awesome List'ler, Orkestrasyon Araçları, MCP Sunucuları, Güvenlik

Bu raporda Claude Code workflow'unuzu hızlandıracak, olgunluğu ve bakımı kanıtlanmış GitHub repoları kategorize edilmiş, puanlanmış ve doğrudan kurulum komutlarıyla birlikte sunulmaktadır.

---

## İçindekiler

1. [Karar Matrisi: Hangi Araç Size Uygun?](#1-karar-matrisi-hangi-araç-size-uygun)
2. [Kategori A: Başlangıç Toolkit ve Hazır Konfigürasyonlar (Hemen Kurun)](#2-kategori-a-başlangıç-toolkit-ve-hazır-konfigürasyonlar)
3. [Kategori B: Çoklu Ajan Orkestrasyon (Paralel İş)](#3-kategori-b-çoklu-ajan-orkestrasyonu-paralel-iş)
4. [Kategori C: En Değerli MCP Sunucuları (Dış Entegrasyonlar)](#4-kategori-c-en-değerli-mcp-sunucuları-dış-entegrasyonlar)
5. [Kategori D: Güvenlik ve Sertleştirme (Hook/Korumalar)](#5-kategori-d-güvenlik-ve-sertleştirme)
6. [Kategori E: Skills/Hazır Komutlar ve Subagent Koleksiyonları](#6-kategori-e-skills-hazır-komutlar-ve-subagent-koleksiyonları)
7. [Kategori F: GUI, TUI ve Arayüz Araçları](#7-kategori-f-gui-tui-ve-arayüz-araçları)
8. [Kategori G: Eğitim, Öğrenme ve Rehberler](#8-kategori-g-eğitim-öğrenme-ve-rehberler)
9. [Kurulum Sırası Önerisi (30 Dk'da Üretim Hazırı)](#9-kurulum-sırası-önerisi-30-dkda-üretim-hazırı)

---

## 1. Karar Matrisi: Hangi Araç Size Uygun?

| Durumunuz | İlk Kurulacak Araçlar |
|:---|:---|
| **Tek başıma CLI'dan çalışırım, sade setup istiyorum** | Claude Squad + cc-safe-setup + GitHub MCP + Context7 MCP |
| **Görsel arayüz, kanban tarzı istiyorum** | Vibe Kanban veya Parallel Code (Electron) |
| **Takım halinde çalışıyoruz, kurumsal standartlar lazım** | awesome-claude-code-toolkit + dwarvesf/claude-guardrails + Plugin paketi |
| **Çok paralel, uzun işler, 24/7 filo istiyorum** | Stoneforge veya Claude Agent Farm + Sentry/Linear MCP |
| **Güvenlik hassasiyetim var (prod erişimi)** | claude-hardening (bwrap sandbox) + dcg + cc-safe-setup |
| **Frontend/ağırlıklı iş yapıyorum** | Playwright MCP + Figma MCP + Vercel skill pack |
| **Yeni başlıyorum, her şey hazır olsun** | everything-claude-code veya awesome-claude-code-toolkit tek seferde |

---

## 2. Kategori A: Başlangıç Toolkit ve Hazır Konfigürasyonlar

Bunlar tek kurulumla CLAUDE.md, komutlar, agentlar, hook'lar ve MCP'leri size getiren "hepsi bir arada" paketler. İlk hafta bunlardan birini seçmek 20+ saat kazandırır.

### A1. ⭐ `rohitg00/awesome-claude-code-toolkit`
**⭐ 2.3K+ stars** — En kapsamlı topluluk toolkit'i

**Neler var:**
- **135 hazır subagent** (10 kategoride: core development, QA, DevOps, security, backend, frontend vb.)
- **35 uzman skill** (TDD, API design, DB optimizasyon, security hardening, Next.js, React, Kubernetes, AWS vb.)
- **42 slash komut** (/commit, /pr-create, /changelog, /release, /worktree, /fix-issue, /pr-review)
- **25 PreToolUse/PostToolUse hook scripti** (güvenlikten otomatik formatlamaya)
- **25+ stack için CLAUDE.md şablonu** (React, Next.js, FastAPI, Django, SvelteKit, Chrome Extension, CLI, MCP vb.)
- **16 hazır MCP konfigürasyonu** (Recommended, Full Stack, Kubernetes, Security, DevOps, Observability vb.)
- 176+ plugin

**Kim için:** Her seviye geliştirici; sıfırdan kurulum yapmak yerine hazırı kullanmak isteyenler.

**Kurulum:**
```bash
git clone https://github.com/rohitg00/awesome-claude-code-toolkit
cd awesome-claude-code-toolkit
# Kurulum scripti ile (interaktif)
./setup/install.sh
# Veya istediğiniz dosyaları manuel kopyalayın:
cp -r commands/* ~/.claude/commands/
cp -r skills/* ~/.claude/skills/
cp -r agents/* ~/.claude/agents/
cp -r hooks/* ~/.claude/hooks/
```

---

### A2. ⭐ `affaan-m/everything-claude-code`
**⭐ 141K+ stars** — En büyük topluluk derlemesi (listeleme/aggregator)

**Neler var:** Tüm ekosistemin toplu indeksi; agentlar, pluginler, MCP sunucuları, hook'lar, workflow'lar, CLI yardımcıları tek yerde listelenmiş.

**Kim için:** Araştırma yapmak ve olası tüm seçenekleri görmek isteyenler. Başlangıçta bu repodan 3-5 araç seçip ilerlemek mantıklı.

**URL:** https://github.com/affaan-m/everything-claude-code

---

### A3. `hesreallyhim/awesome-claude-code` (eski subinium/awesome-claude-code)
**⭐ 36.8K+ stars** — Bakımı en düzenli küratörlü liste

**Neler var:** Claude Code ekosisteminin tüm kategorilerde özenle seçilmiş en iyi araçları; GUI/IDE, proxy/router, MCP sunucuları, hook'lar, skill'ler, subagent'lar, orkestratörler, eğitim.

**Öne çıkanlar:** Güvenlik, orkestrasyon ve MCP listesi özellikle güncel.

**URL:** https://github.com/hesreallyhim/awesome-claude-code

---

### A4. `ccplugins/awesome-claude-code-plugins`
**Resmi plugin kataloğu**

**Neler var:** Claude Code Plugin'ler için topluluk kataloğu. Pluginler `/plugin install <repo>` ile tek komutla kurulabilen paketler.

**Kurulum:**
```
/plugin marketplace add ccplugins/awesome-claude-code-plugins
/plugin list
```

---

### A5. `leerob/directories`
Vercel'den Lee Robinson'ın paylaştığı, coding agent'lar için kural ve MCP sunucu derlemesi. Az ama öz, üretim seviyesinde.

**URL:** https://github.com/leerob/directories

---

## 3. Kategori B: Çoklu Ajan Orkestrasyonu (Paralel İş)

Bunlar birden fazla Claude (veya Codex/Gemini) oturumunu paralel koşturmak, worktree izolasyonunu otomatik yönetmek ve ajanlar arası koordine olmak için kullanılır. Senior lead seviyesinde en yüksek kaldıracı araçlar bu kategoridedir.

### B1. ⭐ `smtg-ai/claude-squad`
**⭐ 6.4K+ stars** — **Solo geliştiriciler için en iyi TUI seçimi**

**Ne yapar:** Tmux tabanlı Terminal UI; tek bir pencereden birden fazla Claude Code, Codex, Aider, Gemini, OpenCode veya Amp oturumunu yönetir.

**Özellikler:**
- Otomatik git worktree oluşturma ve izolasyon
- Tüm oturumları tek ekrandan görme, aralarında geçme
- Tek tuşla ajan başlatma
- Programlar arası geçiş desteği (cs -p codex, cs -p aider vb.)

**Kurulum:**
```bash
# Homebrew
brew install claude-squad
# Veya script
curl -fsSL https://raw.githubusercontent.com/smtg-ai/claude-squad/main/install.sh | bash
# Başlat
cs
```

**Puan:** 9.5/10 — Basit, hızlı, TUI sevenler için biçilmiş kaftan.

---

### B2. `stoneforge-ai/stoneforge`
Yeni ama iddialı; agentların context limitine yaklaşınca otomatik handoff, otomatik test/merge ve web dashboard

**Ne yapar:** Director ajan görevi parçalara böler, worker'lar izole worktree'de çalışır, steward ajan testleri çalıştırır ve kaliteliyse squash-merge yapar. Worker context limitine geldiğinde otomatik olarak yeni temiz context'te devam ettirir (sizin yapacağınız /compact + handoff işini otomatik yapıyor). SQLite+JSONL audit trail.

**Kurulum:**
```bash
npm install -g @stoneforge/smithy
cd your-project
sf init
sf run "auth sistemini yeniden yaz"
```

**Puan:** 9/10 — Özellikle uzun soluklu otonom görevler için tasarlanmış; production kullanımı için erken aşama ama konsept olarak çok güçlü.

---

### B3. `BloopAI/vibe-kanban`
Web tabanlı kanban panosu ile 10+ farklı ajanı yönetme

**Ne yapar:** Klasik kanban panosu (Todo, Doing, Done) üzerinden görev kartları açar, her kart için otomatik worktree oluşturup ajan atar. Claude Code, Codex, Gemini, Copilot, Amp, Cursor, Qwen Code gibi 10+ farklı CLI ajanı destekler. Mobil uyumlu.

**Kim için:** Görsel düşünen, sürükle-bırak sevenler.

**Kurulum:**
```bash
# Detaylar için repo
git clone https://github.com/BloopAI/vibe-kanban
cd vibe-kanban && npm install && npm run dev
```

---

### B4. `johannesjo/parallel-code`
Electron masaüstü uygulaması; Claude Code, Codex ve Gemini'yi yan yana worktree'de koşturur

**Öne çıkanlar:**
- Her ajan için otomatik worktree ve branch
- node_modules sembolik link paylaşımı (disk alan tasarrufu)
- Docker sandbox desteği (ajanları container içinde çalıştırma)
- Test coverage radar
- macOS ve Linux desteği (Windows yakında)

**URL:** https://github.com/johannesjo/parallel-code

---

### B5. `generalaction/emdash`
**YC W26** — Electron masaüstü; 22+ farklı CLI provider destekli

**Özellikler:** Paralel dispatch, insan onay noktaları, paralel çalıştırma.

**URL:** https://github.com/generalaction/emdash

---

### B6. `ComposioHQ/open-claude-cowork` / Composio AO
500+ SaaS entegrasyonu ile birlikte gelen açık kaynak Claude Cowork; web dashboard. Özellikle GitHub Issue → ajan → PR döngüsü için iyi.

---

### B7. `21st-dev/1code`
Masaüstü GUI; git worktree izolasyonu ve paralel ajan çalıştırma. Claude Code ve Codex birinci sınıf destek.

---

### B8. `Dicklesworthstone/claude_code_agent_farm`
20+ ajanı paralel koşturan Python orchestrator; 33 hazır config (Next.js, Python, Rust, Go, Java, C++, Terraform, Solana vb.), best-practice sweep ve otomatik bug fixing workflow. 35 best-practices rehberi ile geliyor.

**Kim için:** Mevcut codebase'i sistematik olarak iyileştirmek isteyenler (toplu bug fix, best-practice uygulama).

---

### B9. `indiekitai/claude-orchestrator`
Claude Code skill olarak çalışan orchestrator; büyük feature'ları görev sözleşmelerine ayırır, paralel worktree ajanları dağıtır, quality gate uygular ve birleştirir. Harici daemon gerektirmez, Claude Code'un kendi subagent mekanizmasını kullanır.

**Kurulum:**
```bash
git clone https://github.com/indiekitai/claude-orchestrator.git
ln -s "$(pwd)/claude-orchestrator" ~/.claude/skills/build-orchestrator
# Claude içinde:
/build-orchestrator
```

---

### B10. `FrankBria/ralph-claude-code`
**⭐ 7.9K stars** — Akıllı çıkış tespiti, rate limiting, circuit breaker ile otonom development loop. Özellikle ajanın ne zaman durup insana devredeceğini iyi ayarlamış.

---

### Karşılaştırma Tablosu

| Araç | Tip | UI | Desteklediği ajanlar | Kurulum zorluğu | Yıldız |
|:---|:---|:---|:---|:---:|:---|
| **Claude Squad** | TUI | Terminal | Claude, Codex, Aider, Gemini, Amp, OpenCode | Çok kolay | 6.4K |
| **Stoneforge** | CLI+Web | Dashboard | Claude, Codex, OpenCode | Orta | Yeni |
| **Vibe Kanban** | Web | Kanban | 10+ ajan | Orta | Yüksek |
| **Parallel Code** | Electron | Masaüstü | Claude, Codex, Gemini | Kolay | — |
| **Emdash** | Electron | Masaüstü | ~22 CLI | Orta | — |
| **Claude Agent Farm** | Python CLI | Terminal | Claude Code | Orta | — |
| **Claude Orchestrator** | Skill | Claude içi | Claude Code (subagent) | Çok kolay | — |
| **Ralph** | CLI | Terminal | Claude | Kolay | 7.9K |

**Tavsiyem:** Başlangıçta **Claude Squad** ile başlayın. İhtiyacınız büyüdükçe Stoneforge veya Vibe Kanban'a geçersiniz.

---

## 4. Kategori C: En Değerli MCP Sunucuları (Dış Entegrasyonlar)

MCP (Model Context Protocol) sunucuları Claude Code'un dış araçlara erişmesini sağlar. İyi seçilmiş 3-5 MCP günlük iş akışınızı tamamen değiştirir.

### Temel Öneri: İlk 3'ü Hemen Kurun
1. **GitHub MCP** (resmi) — Issue/PR/Code search/Actions
2. **Context7** — Güncel kütüphane dökümantasyonu (hallüsinasyonu azaltır)
3. **Playwright MCP** — Gerçek tarayıcı ile E2E test ve UI doğrulama

İhtiyaca göre ekleyeceğiniz MCP'ler:

### C1. `github/github-mcp-server` ⭐ 31K
**Resmi GitHub MCP sunucusu.** Claude direkt repo'yu okuyabilir, issue/PR açabilir, code search yapabilir, Actions çalıştırabilir, review yazabilir.

**Kurulum:**
```bash
claude mcp add --transport http github https://api.githubcopilot.com/mcp
```

### C2. `upstash/context7-mcp` ⭐ 58K
**En değerli MCP.** Kütüphanelerin güncel, sürüme özel dökümantasyonunu çeker. "React 19'un yeni form API'si nasıl?" diye sorduğunuzda Claude'un eğitim verisindeki eski bilgiyi değil, gerçek dokümantasyonu kullanmasını sağlar. API imza yanlışı (hallüsinasyon) oranını ciddi düşürür.

**Kurulum:**
```bash
claude mcp add --transport http context7 https://mcp.context7.com/mcp
```

### C3. `microsoft/playwright-mcp` ⭐ 34K
**Resmi Microsoft Playwright MCP.** Claude gerçek bir tarayıcıyı (Chromium) sürebilir, kendi yazdığı UI'yi açabilir, screenshot alabilir, E2E testi koşturabilir, domda element arayabilir. Özellikle frontend geliştirmede oyun değiştirici.

**Kurulum:**
```bash
claude mcp add playwright -- npx @playwright/mcp@latest
```

### C4. `modelcontextprotocol/servers` (resmi)
Anthropic'in resmi sunucu koleksiyonu. Filesystem, Postgres, SQLite, Fetch, Memory (kalıcı bilgi grafı), Slack, Brave Search, Sequential Thinking, Sentry gibi sunucuları içerir.

**Kurulum (Postgres örneği — readonly kullanıcıyla):**
```bash
claude mcp add postgres -- npx -y @modelcontextprotocol/server-postgres "postgresql://readonly:pass@localhost:5432/db"
```

### C5. `getsentry/sentry-mcp`
**Resmi Sentry MCP.** Hata stack trace'lerini direkt Claude'ın önüne getirir; triage ajanı için vazgeçilmez.

**Kurulum:** Tarayıcıda yetkilendirme ile çalışır.
```bash
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

### C6. `linear/linear-mcp` (resmi)
Linear issue'larınıza Claude'un erişmesi; kart okuma, durum güncelleme, yorum ekleme. Otonom bug-fix pipeline için ikisi bir arada (Sentry+Linear) şart.

```bash
claude mcp add --transport sse linear https://mcp.linear.app/sse
```

### C7. Firecrawl MCP
Web kazıma ve arama; rakipler, dökümantasyon veya herhangi bir site içeriğini temiz markdown olarak çeker.

### C8. Figma MCP (resmi beta)
Figma tasarımlarını okuyup doğrudan kod üretebilme. Frontend ekibi için çok değerli.

### MCP Yönetim İpuçları:
- Çok fazla MCP kurmayın; aktif sunucu sayısı arttıkça bağlam (tool listesi) şişer ve Claude yanlış tool seçmeye başlar. **İdeal aktif MCP sayısı 3-6.**
- Kullanmadıklarınızı devre dışı bırakın: `/mcp` komutu ile yönetebilirsiniz.
- Veritabanı gibi hassas MCP'leri **readonly kullanıcı** ile bağlayın.
- Toplu listeleme için: `punkpeye/awesome-mcp-servers` veya `appcypher/awesome-mcp-servers` repolarına bakın.

---

## 5. Kategori D: Güvenlik ve Sertleştirme

### D1. ⭐ `mckechniep/claude-hardening`
**En kapsamlı sertleştirme kiti.** OS seviyesinde koruma (bwrap ile namespace izolasyonu).

**Neler yapar:**
- `.ssh`, `.aws`, `.gnupg` gibi gizli bilgi dizinlerini boş tmpfs ile overlay eder ( ajan ne yaparsa yapın erişemez)
- Ağ egress kontrolü (izinli domainler dışına çıkamaz)
- 7+ PreToolUse hook (sandbox-exec, deny-destructive, network-egress, git-guard, secret-scan, mcp-guard, audit-log)
- Hash-zincirli denetim kaydı (audit log'lar sonradan değiştirilemez)
- 3 profil: standard, strict (ağ izolasyonu + readonly FS), readonly
- 201 test ile doğrulanmış

**Kim için:** Hassas verilerle çalışanlar, production erişimi olan makinelerde çalışanlar.

**Kurulum:**
```bash
git clone https://github.com/mckechniep/claude-hardening
cd claude-hardening
./install.sh
# Profil seçimi: standard / strict / readonly
```

**Puan:** 10/10 — Güvenlik ciddiye alındığında ilk kurulacak paket.

---

### D2. `Dicklesworthstone/destructive_command_guard` (dcg)
**⭐ Endüstri standardı PreToolUse güvenlik duvarı.** Rust ile yazılmış, sub-milisaniye hızında.

**Neler yapar:**
- 49+ güvenlik paketi (git, veritabanı, docker, kubectl, cloud CLI'lar, terraform vb.)
- Heredoc, AST (tree-sitter) ve obfuscation tespiti (base64 ile gizlenmiş komutlar dahil)
- Engellediği komut için güvenli alternatif önerisi
- Tek seferlik izin mekanizması (short code)
- Claude Code, Gemini CLI, Aider ile uyumlu

**Kurulum:**
```bash
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/destructive_command_guard/master/install.sh" | bash -s -- --easy-mode
# Otomatik olarak PreToolUse hook'u olarak kaydolur
```

**Test:**
```bash
echo '{"tool_name":"Bash","tool_input":{"command":"git reset --hard"}}' | dcg hook
# BLOCKED yanıtı almalısınız
```

---

### D3. `yurukusa/cc-safe-setup`
908 hook ile en geniş koruma seti. Tek komutla kurulur. Plugin olarak da kurulabilir.

**Neleri engeller:**
- `rm -rf` korumalı dizinlerde
- `git push --force`, doğrudan main'e push
- `git reset --hard`, `git clean -fd`
- Framework DB reset komutları (prisma migrate reset, rails db:migrate:reset, django flush vb.)
- Terraform destroy, AWS terminate, kubectl delete namespace
- `.env` dosyalarına yazma, servis account dosyaları
- 100KB üstü dosya okuma (token israfını önler)
- Toplam token bütçesi / subagent fan-out limiti (sınırsız recursive ajan çağrılarını engeller)
- Geri dönüşüm kutusu mekanizması (silinen dosyaları geri getirebilir)

**Kurulum:**
```bash
npx github:yurukusa/cc-safe-setup
# Veya plugin olarak:
/plugin marketplace add yurukusa/cc-safe-setup
/plugin install safety-essentials@cc-safe-setup
```

---

### D4. `dwarvesf/claude-guardrails`
Dwarves Foundation tarafından kullanılan production-hardened konfigürasyon. Full ve Lite olmak üzere iki profil.

- **Full versiyon:** 5 katman savunma (deny rules, 5 PreToolUse hook, PostToolUse prompt injection tarayıcı, CLAUDE.md güvenlik kuralları, OS sandbox rehberi)
- **Lite versiyon:** İç projeler için; temel deny rules + 3 hook

**URL:** https://github.com/dwarvesf/claude-guardrails

---

### D5. `roboticforce/agent-guardrails`
Hard-coded policy enforcment; terraform destroy, DROP TABLE gibi komutları runtime'da engeller. CLI ve Docker desteği var.

---

### D6. `mattpocock/skills` — Git Guardrails skill'i
Matt Pocock'un hazır hook seti; tehlikeli git komutlarını (push --force, reset --hard, branch -D) engelleyen PreToolUse hook.

**Kurulum (Claude içinden):**
```
# Matt Pocock skill'lerinden git-guardrails-claude-code'i yükle
```

---

## 6. Kategori E: Skills/Hazır Komutlar ve Subagent Koleksiyonları

### E1. `wshobson/agents` ⭐ 31K
112 uzman subagent, 72 plugin, 146 skill ve 79 geliştirme aracı. Organizasyon ve kategorizasyon çok iyi; istediğin ajanı kolayca bulabilirsin.

### E2. `VoltAgent/awesome-claude-code-subagents` ⭐ 14K
126+ hazır subagent'ın küratörlü listesi.

### E3. `vijaythecoder/awesome-claude-agents` ⭐ 4K
Orkestre sub-agent geliştirme ekibi; her rol için özelleşmiş ajan (Frontend Dev, Backend Dev, QA, DevOps, Architect vb.).

### E4. `jeffallan/claude-skills`
66 tam yığın beceri; Python/Go/TypeScript baştan sona.

### E5. `swarmclawai/andrej-karpathy-skills`
Andrej Karpathy'nın LLM kodlama dört kuralını sistematik olarak uygulayan skill paketi: düşün önce yaz, basitlik önce, cerrahi değişiklikler, hedef odaklı çalışma.

### E6. Superpowers (obolus/superpowers) ⭐ 94K
TDD zorunlu kılan 7-fazlı ajanik geliştirme workflow'u. Plan → Test → Implement → Refactor döngüsünü zorunlu tutar.

### E7. Vercel Skill Pack (topluluk)
- **Vercel React Best Practices:** 57 performans kuralı
- **Vercel Web Design Guidelines:** 100+ UX/accessibility kuralı
- **Vercel Composition Patterns:** Boolean prop cehennemini compound component paterni ile değiştirir

### E8. `vercel-labs/handoff` (öneri)
Oturumu handoff.md formatında özetleyip temiz oturuma geçmeyi sağlayan built-in skill; sizin manuel yapacağınız context sıfırlama işini standartlaştırır.

### E9. `mksglu/context-mode`
Shell çıktısından gürültüyü filtreleyerek %98'e varan context azaltması sağlayan hook+skill+MCP seti. 15+ ajan platformu destekliyor (Claude Code, Codex, Cursor, Kiro, Zed, OpenClaw). Uzun oturumlar için çok değerli.

### E10. `breferrari/obsidian-mind`
Obsidian vault'u ile kalıcı AI ajanı hafızası entegrasyonu. Claude Code, Codex ve Gemini CLI ile çalışır. Raporumuzdaki Hot katman hafızasının hazır implementasyonu.

---

## 7. Kategori F: GUI, TUI ve Arayüz Araçları

CLI dışında görsel arayüz tercih edenler için:

| Araç | Tip | Açıklama |
|:---|:---|:---|
| `smtg-ai/claude-squad` | TUI | Terminal, paralel oturum yönetimi (en olgun TUI) |
| `asheshgoplani/agent-deck` | TUI | Claude Code, Gemini, OpenCode, Codex için çoklu oturum |
| `21st-dev/1code` | Desktop (Electron) | Worktree izolasyonu + paralel ajan |
| `iOfficeAI/AionUi` | Desktop | Claude Code, Codex ve daha fazlası için multi-agent cowork GUI |
| `siteboon/claudecodeui` | Web+Mobil | Uzaktan yönetim, push notification |
| `slopus/happy` | Web+Mobil | Uçtan uca şifreli, push alert (ajan tamamlandı bildirimi) |
| `wbopan/cui` | Web | Hafif web UI |
| `The-Vibe-Company/companion` | Web | Açık kaynak web arayüz |
| `BloopAI/vibe-kanban` | Web | Kanban panelli görsel yönetim |
| `tiann/hapi` | Mobil+Web | Her yerde vibe coding (Android) |
| `coder/claudecode.nvim` | Neovim | WebSocket MCP protokolü ile saf Lua entegrasyon |
| `folke/sidekick.nvim` | Neovim | Claude Code, Codex, Gemini yan panel |
| `ComposioHQ/open-claude-cowork` | Web | 500+ SaaS entegrasyonlu |

**Tavsiyem:** CLI/TUI ile başladıysanız Claude Squad yeterli. Mobil müdahale için Happy veya hapi ekleyin.

---

## 8. Kategori G: Eğitim, Öğrenme ve Rehberler

- **`disler/claude-code-hooks-mastery`** — Hook'ları derinlemesine öğreten rehber
- **`davidkimai/Context-Engineering`** — Karpathy'den ilhamlı context engineering el kitabı
- **`humanlayer/advanced-context-engineering-for-coding-agents`** — Büyük codebase'lerde context yönetimi
- **`zebbern/claude-code-guide`** ve **`Cranot/claude-code-guide`** — Kurulum, komutlar, workflow'lar (2 günde bir otomatik güncellenen)
- **`ykdojo/claude-code-tips`** — 45 ipucu, başlangıçtan ileri seviyeye
- **`shareAI-lab/learn-claude-code`** — 50-550 satır Python arasında sıfırdan ajan paterni öğretici eğitim
- **`ghuntley/how-to-build-a-coding-agent`** — Kendi coding ajanı yapma atölyesi
- **`ChrisWiles/claude-code-showcase`** — Tam proje konfigürasyon örneği (hook, skill, agent, komut, GitHub Actions hepsi bir arada)
- **`Njengah/claude-code-cheat-sheet`** — Hile sayfası

---

## 9. Kurulum Sırası Önerisi (30 Dk'da Üretim Hazırı)

Sırayla uygularsanız 30 dakikada güçlü bir kuruluma sahip olursunuz:

### Adım 1: Claude Code Güncel (2 dk)
```bash
curl -fsSL https://claude.ai/install.sh | bash
claude --version  # ≥ 2.1.90 olsun (güvenlik açığı düzeltmesi)
```

### Adım 2: Güvenlik (5 dk)
```bash
# dcg (destructive command guard)
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/destructive_command_guard/master/install.sh" | bash -s -- --easy-mode

# cc-safe-setup (ek korumalar)
npx github:yurukusa/cc-safe-setup
```

### Adım 3: Temel MCP'ler (5 dk)
```bash
# GitHub
claude mcp add --transport http github https://api.githubcopilot.com/mcp
# Context7 (dokümanlar)
claude mcp add --transport http context7 https://mcp.context7.com/mcp
# Playwright (tarayıcı testi)
claude mcp add playwright -- npx @playwright/mcp@latest
```

### Adım 4: Orkestrasyon (5 dk)
```bash
# Claude Squad TUI
brew install claude-squad
# veya script ile
```

### Adım 5: Hazır Beceriler ve Agentlar (10 dk)
awesome-claude-code-toolkit'ten seçeceğiniz 10-15 beceri ve ajanı kopyalayın. Başlangıç için tavsiye edilenler:
- `/commit`, `/pr-create`, `/pr-review`, `/fix-issue` komutları
- `code-reviewer`, `security-auditor`, `test-engineer` subagentları
- `tdd`, `security-hardening` skill'leri
- İlgilendiğiniz stack için CLAUDE.md şablonu (Next.js, Python vb.)

### Adım 6: CLAUDE.md Özelleştirmesi (3 dk)
Kendi proje kurallarınızı ekleyin.

### Toplam: ~30 dakika
Kurulumdan sonra Claude Code basit bir CLI aracından; güvenli, çoklu ajanlı, dış dünya ile entegre bir yazılım fabrikasına dönüşür.

---

## Özet: İlk Bakışta Yük Kaldıran 10 Araç

| # | Araç | Kategori | Neden İlk 10'da |
|:---:|:---|:---|:---|
| 1 | **dcg (destructive_command_guard)** | Güvenlik | Her komutu denetler, tek satır kurulum |
| 2 | **cc-safe-setup** | Güvenlik | 908 koruma hook'u, jet gibi kurulur |
| 3 | **GitHub MCP** | MCP | Tüm GitHub iş akışınızı Claude'a açar |
| 4 | **Context7 MCP** | MCP | Dokümantasyon kaynaklı hataları %80+ azaltır |
| 5 | **Playwright MCP** | MCP | Claude kendi yazdığı UI'yi test edebilir |
| 6 | **Claude Squad** | Orkestrasyon | 5 dakikada paralel ajan yönetimi |
| 7 | **awesome-claude-code-toolkit** | Toolkit | 135 agent + 35 skill + 42 komut hazır |
| 8 | **claude-hardening** (hassas makinelerde) | Güvenlik | OS seviyesi sandbox, tam koruma |
| 9 | **Context Mode** | Bellek | Context şişmesini %98'e varan oranda azaltır |
| 10 | **Linear/Sentry MCP** | MCP | Otonom bug-fix pipeline için |

---

Bu kurulumla raporumuzdaki L3 seviyesine (Takım Çapında Benimseme) birkaç saat içinde ulaşabilir; sonraki haftalarda Stoneforge/Vibe Kanban gibi araçlarla L4 otonomiye geçebilirsiniz.
