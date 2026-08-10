# CLAUDE CODE RAPOR KALİTE DEĞERLENDİRMESİ VE DOĞRULUK RAPORU

**Tarih:** 2026-08-09
**Doğrulama Kaynakları:** Anthropic Resmi Dokümantasyonu, Claude Code v2.1.x değişiklik notları, güncel fiyatlandırma, endüstri best-practice yayınları, güvenlik araştırmaları.

---

## Değerlendirme Metodolojisi

Her bölüm 10 üzerinden şu kriterlere göre puanlandı:
1. **Doğruluk (4 puan):** Verilen komutlar, fiyatlar, özellikler güncel mi?
2. **Kapsam (3 puan):** Claude Code'un 2026'daki önemli feature'ları (MCP, Hooks, Skills, Subagents, Agent Teams) temsil ediliyor mu?
3. **Pratiklik (2 puan):** Bilgi doğrudan uygulanabilir mi?
4. **Stratejik Değer (1 puan):** Senior lead seviyesinde derinlik var mı?

**Kural:** Puanı 9+ olan bölümler olduğu gibi bırakıldı. 9 altı olanlar güncel bilgilerle güçlendirildi.

---

## Bölüm Puanları

| # | Dosya | Puan | Karar | Not |
|---|---|---:|---|---|
| 00 | KAPAK ve İçindekiler | 9.5 | ✅ Olduğu gibi bırakıldı | Belge yapısı sağlam |
| 01 | Yönetici Özeti | 9.0 | ✅ Olduğu gibi bırakıldı | Temel tez güncel ve doğru |
| 02 | Olgunluk Modeli L0→L4 | 8.5 | 🔧 Küçük düzeltme | L3-L4 seviyelerine MCP/Hooks/Subagents katmanları eklenecek |
| 03 | **Faz 0: Temel Kurulum** | **6.0** | 🔴 BÜYÜK GÜNCELLEME | npm kurulumu v2.1.15'te deprecated; native installer (`curl`/`irm`) öneriliyor. Birçok yeni komut ve özellik eksik. `claude login` yerine browser auth. |
| 04 | **Faz 1: Bireysel Ustalık** | **7.0** | 🟡 Güçlendirme | MCP Server'lar, Skills, Hooks, Subagents en kritik yeni feature'lar hiç anlatılmamış. Komutlarda `/cost` yerine `/usage`, `/effort`, `/skills` eksik. |
| 05 | **Faz 2: Takım Benimseme** | **7.0** | 🟡 Güçlendirme | Plugins ile takım dağıtımı, Agent Teams özelliği, resmi CLI `--worktree` flag'i, `claude-squad` gibi orkestratörler eksik. |
| 06 | **Faz 3: Otonom Fabrika** | **7.0** | 🟡 Güçlendirme | Scheduled Tasks (cloud cron) yerleşik feature, background agents `claude --bg` artık native, gereksiz özel script azaltma. |
| 07 | Model Karakteri | 9.0 | ✅ Olduğu gibi bırakıldı | Yeni Fable 5 modeli için tabloya ufak ekleme yapıldı |
| 08 | Context ve Graf Mühendisliği | 9.0 | ✅ Olduğu gibi bırakıldı | Vector search yetersizliği ve multi-hop GraphRAG tezi endüstri yayınları ve arxiv makaleleri ile tamamen doğrulandı. |
| 09 | Deterministik Kontrol | 9.0 | ✅ Olduğu gibi bırakıldı | Zero-context verifier pattern endüstride "Adversarial/Maker-Checker Review" olarak standart. Kod tamamen doğru. |
| 10 | Paralel Orkestrasyon | 8.5 | 🔧 Küçük ekleme | Worktree izolasyonu doğru; ancak runtime isolation (port/.env çakışması) sorunu ve mevcut orkestratör araçları (Claude Squad, Copilot App, Composio AO) notu eklenecek. |
| 11 | **Bütçe Fiziği** | **6.0** | 🔴 BÜYÜK GÜNCELLEME | Fiyatlar eski: Opus 5 $5/$25, Sonnet 5 $2/$10 (promo) / $3/$15, Haiku 4.5 $1/$5, Fable 5 $10/$50. Prompt Caching (%90 indirim), Batch API (%50 indirim) ve Pro/Max/Team planları doğru fiyatlarla güncellenecek. |
| 12 | **Güvenlik** | **7.0** | 🔴 GÜÇLENDİRME | v2.1.90 öncesi "50 subcommand bypass" güvenlik açığı (CVE benzeri) ve PreToolUse hook'ları ile gerçek guardrail, `destructive_command_guard`, `agent-guardrails` araçları eklenecek. Settings.json deny kuralları somut örneklerle verilecek. |
| 13 | Ölçümleme | 8.5 | 🔧 Küçük ekleme | `/usage` komutu ile yerleşik ölçümden bahsetme |
| 14 | **30/60/90 Yol Haritası** | **7.0** | 🟡 Güçlendirme | Yeni feature'ları (MCP, Skills, Plugins) yol haritasına entegre etme |
| 15 | Acil Durum Playbook'ları | 8.5 | 🔧 Küçük ekleme | Deny rule bypass/karmaşık komut şüphesi senaryosu eklenecek |
| 16 | Sonuç ve Vizyon | 9.5 | ✅ Olduğu gibi bırakıldı | Stratejik vizyon güncel ve doğru |
| 97 | **Ek A: Komut Özeti** | **6.0** | 🔴 GÜNCELLEME | Tüm komutlar native yükleyici ve yeni flag'lerle güncellenecek |
| 98 | **Ek B: Dizin Yapısı** | **6.0** | 🔴 GÜNCELLEME | Gerçek `.claude/` klasör yapısı (rules, agents, skills, hooks, settings katmanları), yükleme önceliği (priority hierarchy), global/proje/local ayrımı verilecek. |

---

## Önemli Bulgular (Web Research Sonucu)

### 1. Kurulum Metodunda Temel Değişiklik
- **npm kurulumu v2.1.15+ itibarıyla DEPRECATED.** Artık resmi tavsiye edilen yöntem native binary installer:
  - macOS/Linux/WSL: `curl -fsSL https://claude.ai/install.sh | bash`
  - Windows: `irm https://claude.ai/install.ps1 | iex`
  - Homebrew: `brew install --cask claude-code`
- Kurulum Node.js gerektirmiyor; native binary arka planda geliyor.
- Otomatik arka plan güncellemesi varsayılan olarak açık.

### 2. Claude Code Uzantı Mimarisi (2026'da Kritik Beş Katman)
1. **CLAUDE.md** — Her oturumda yüklenen kurallar (her request'e girdiği için kısa tutulmalı)
2. **MCP (Model Context Protocol)** — Dış araçlar, veritabanı, GitHub, Sentry vb. bağlantısı
3. **Skills** — Yeniden kullanılabilir prosedürler, otomatik tetiklenebilir veya `/skill` ile çağrılır (eski `/commands` artık skill olarak birleşti)
4. **Hooks** — Yaşam döngüsü olaylarında (PreToolUse, PostToolUse, SessionStart, PreCompact, vb.) otomatik scriptler; güvenlik ve otomasyon için kullanılır
5. **Subagents/Agent Teams** — İzole context'te çalışan yan ajanlar; paralel görev ve research için, ana context'i kirletmez

### 3. Yerleşik Yeni Özellikler
- `claude --worktree <branch>` → Otomatik izole worktree oluşturup orada başlat
- `claude --bg "görev"` → Arka plan ajanı
- `claude agents` → Çalışan/tamamlanmış ajanları listele
- `/effort` → low/medium/high/xhigh/max zeka seviyesi ayarı (overthinking'i önler)
- `/usage` → token ve maliyet gösterimi (eski `/cost` yerine)
- `/skills` → mevcut becerileri listele
- `--max-budget-usd $X` → harcama limiti (script modunda)
- `--max-turns N` → maksimum tur limiti
- Scheduled Tasks: bulut cron ile plansız tetikleme

### 4. Fiyatlandırma (Ağustos 2026)
| Model | Input/MTok | Output/MTok |
|:---|---:|---:|
| Claude Fable 5 (yeni, en güçlü) | $10 | $50 |
| Claude Opus 5 | $5 | $25 |
| Claude Sonnet 5 (promo) | $2 | $10 |
| Claude Sonnet 5 (normal, 1 Eylül sonrası) | $3 | $15 |
| Claude Haiku 4.5 | $1 | $5 |
| **Cache Hit (tüm modeller)** | **%10** | — |
| Batch API (tüm modeller) | **%50 indirim** | **%50 indirim** |

**Abonelik Planları:**
- Free: $0, sınırlı Sonnet
- Pro: $20/ay
- Max 5x: $100/ay
- Max 20x: $200/ay (70M token/ay'a kadar API'ye denk)
- Team: $100/kullanıcı/ay (minimum 5 kişi)

### 5. Güvenlik Açığı
- v2.1.90 öncesinde **50+ subcommand içeren birleşik komutlarda deny kuralları atlanabiliyordu.**
- Düzeltme v2.1.90 ile geldi.
- Sadece LLM-prompt temelli kurallara güvenmek yerine **PreToolUse hook'ları ile donanım seviyesi engelleme** yapılması endüstri standardı.
- Popüler araçlar: `destructive_command_guard (dcg)`, `agent-guardrails`, `Cloudanix`

### 6. Paralel Orkestrasyon Endüstri Standardı
- Git worktree izolasyonu tüm araçlar tarafından benimsenmiş (GitHub Copilot App bile kendi içinde otomatik worktree kullanıyor).
- Runtime isolation ihtiyacı: worktree dosya sistemini ayırır ama port, localhost, veritabanı paylaşılır. Her ajan için ayrı port/.env.local gerekir.
- Mevcut orkestratörler: Claude Squad (TUI, tavsiye edilen), Composio AO, Baton, Vibe Kanban.
- Ağustos 2026 itibarıyla Claude Code'un yerleşik **Agent Teams** özelliği birden fazla ajanın P2P haberleşerek koordinasyonunu sağlıyor.

### 7. `.claude/` Dizin Yapısı (Kesin Hierarşi)
```
Proje:
CLAUDE.md                    ← Takım talimatları (committed)
CLAUDE.local.md              ← Kişisel ayarlar (gitignored)
.mcp.json                    ← MCP sunucu konfigürasyonu
.claude/
├── settings.json            ← Takım izinleri ve ayarları (committed)
├── settings.local.json      ← Kişisel izinler (gitignored)
├── rules/                   ← Modüler talimat dosyaları (*.md)
│   ├── code-style.md
│   └── testing.md
├── skills/                  ← Otomatik/manual beceriler
│   └── <skill-name>/SKILL.md
├── agents/                  ← Özel subagent persona'ları
│   ├── code-reviewer.md
│   └── security-auditor.md
└── hooks/                   ← Yaşam döngüsü scriptleri

Global (~/.claude/):
Aynı yapı artı: projects/<hash>/memory/MEMORY.md otomatik hafıza, keybindings.json
```

Yükleme önceliği (en yüksekten en düşüğe):
1. CLI flags
2. `.claude/settings.local.json`
3. `.claude/settings.json`
4. `~/.claude/settings.local.json`
5. `~/.claude/settings.json`
6. CLAUDE.local.md → CLAUDE.md → ~/.claude/CLAUDE.md → Managed Policy

---

## Güncellenecek Dosyaların Listesi
1. 02-Olgunluk Modeli (küçük ekleme)
2. 03-Faz 0 Kurulum (ana güncelleme)
3. 04-Faz 1 Bireysel Ustalık (MCP/Skills/Hooks/Subagents ekle)
4. 05-Faz 2 Takım (Plugins, Teams)
5. 06-Faz 3 Fabrika (Background agents, Scheduled Tasks)
6. 10-Paralel Orkestrasyon (Runtime isolation, araçlar)
7. 11-Bütçe Fiziği (güncel fiyatlar, caching, batch)
8. 12-Güvenlik (CVE, hook guard, deny rules)
9. 13-Ölçümleme (küçük)
10. 14-Yol Haritası (özellik entegrasyonu)
11. 15-Acil Durum (güvenlik senaryosu)
12. 97-Ek A Komutlar (tam güncelleme)
13. 98-Ek B Dizin Yapısı (tam güncelleme)
