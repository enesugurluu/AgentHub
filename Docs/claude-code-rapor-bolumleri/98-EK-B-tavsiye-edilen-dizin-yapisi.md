## EK B: Tavsiye Edilen Dizin Yapısı

Ağustos 2026'da Claude Code'un resmi hiyerarşisine ve endüstri best practice'lerine uygun olarak, tam bir proje ve global kurulum dizini aşağıdadır.

---

### B.1 Global Dizin (`~/.claude/`) — Kişisel Tüm Projeler

Bu dizin ev dizininizde yer alır, size özgü ayarları içerir ve git ile paylaşılmaz.

```
~/.claude/
├── CLAUDE.md                      # Global kişisel talimatlar (tüm projelerde yüklenir)
├── settings.json                  # Global izinler ve ayarlar
├── settings.local.json            # Makineye özel kişisel ayarlar
│
├── rules/                         # Global kişisel kurallar (tüm projelerde otomatik)
│   ├── communication.md           # İletişim tarzı, yanıt formatı
│   ├── principles.md              # Kişisel mühendislik prensipleri
│   └── git-style.md               # Commit mesaj formatı vb.
│
├── commands/                      # Global slash komutlarınız (/user:<name> ile çağrılır)
│   ├── daily-standup.md
│   └── pr-desc.md
│
├── skills/                        # Global kişisel beceriler
│   ├── commit-message/
│   │   └── SKILL.md
│   ├── debug-tsx/
│   │   └── SKILL.md
│   └── refactor-safe/
│       └── SKILL.md
│
├── agents/                        # Global subagent persona'ları
│   ├── security-auditor.md        # Güvenlik denetçisi (read-only)
│   ├── code-reviewer.md           # Zero-context kod inceleme
│   ├── performance-expert.md      # Performans analizi
│   └── test-writer.md             # Test yazma uzmanı
│
├── workflows/                     # Çoklu adımlı iş akışları
├── output-styles/                 # Kişisel yanıt stili/ton
├── hooks/                         # Global hook scriptler (her projede çalışır)
│   └── pre-bash-dcg.sh            # dcg (destructive command guard) entegrasyonu
│
├── projects/                      # Projeye özel otomatik hafıza
│   └── <project-hash>/
│       └── memory/
│           ├── MEMORY.md          # Ana hafıza indeksi (her oturumda yüklenir)
│           ├── lessons.md         # Öğrenilen dersler
│           └── gotchas.md         # Tuzaklar
│
├── keybindings.json               # Özel klavye kısayolları
├── themes/                        # CLI için özel renk temaları
└── tasks/                         # Görev listeleri
    └── <task-list-id>/

~/.claude.json                    # Global MCP sunucuları, OAuth, önbellek ayarları
```

---

### B.2 Proje Dizini (Repository İçi) — Takım Paylaşımlı

Proje kökündeki `.claude/` ve kök dosyalar git ile takım arkadaşlarınıza dağıtılır.

```
project-root/
├── CLAUDE.md                      # ANA TAKIM TALİMATLARI (committed)
├── CLAUDE.local.md                # Kişisel proje notları (gitignored)
├── .mcp.json                      # Proje MCP sunucuları (committed)
├── .claudeignore                  # Claude'un taramayacağı dosyalar (committed)
├── .worktreeinclude               # Yeni worktree'lere kopyalanacak dosyalar
│
└── .claude/
    ├── settings.json              # TAKIM İZİNLERİ (committed)
    │                              # allow/deny/ask listeleri, hook tanımları
    ├── settings.local.json        # Kişisel izin tercihleri (gitignored)
    │
    ├── rules/                     # MODÜLER KURALLAR (otomatik yüklenir)
    │   ├── code-style.md          # Genel kod stili
    │   ├── testing.md             # Test yazma kuralları
    │   ├── api-conventions.md     # API tasarım prensipleri
    │   ├── security.md            # Güvenlik zorunlulukları
    │   └── frontend/              # Yol bazlı koşullu kurallar
    │       └── react-patterns.md  # Sadece frontend/ altındayken aktif
    │
    ├── skills/                    # Yeniden kullanılabilir iş akışları
    │   ├── code-review/
    │   │   ├── SKILL.md           # Beceri tanımı (ana dosya)
    │   │   └── review-checklist.md
    │   ├── deploy/
    │   │   ├── SKILL.md
    │   │   ├── pre-deploy.md
    │   │   └── templates/
    │   │       └── release-notes.md
    │   ├── tdd/
    │   │   └── SKILL.md
    │   ├── new-component/
    │   │   └── SKILL.md
    │   └── write-adr/
    │       └── SKILL.md
    │
    ├── agents/                    # ÖZEL SUBAGENT PERSONA'LARI
    │   ├── code-reviewer.md       # Sıfır-bağlam şüpheci göz
    │   ├── security-auditor.md    # Güvenlik uzmanı
    │   ├── test-engineer.md       # Test mühendisi
    │   ├── db-expert.md           # Veritabanı uzmanı
    │   └── architect.md           # Mimari inceleme
    │
    ├── commands/                  # Özel slash komutlar (eski sistem, halen çalışır)
    │   ├── review.md              # /project:review
    │   ├── fix-issue.md           # /project:fix-issue
    │   └── deploy.md              # /project:deploy
    │
    ├── hooks/                     # Yaşam döngüsü scriptleri
    │   ├── pre-tooluse-bash.sh    # Her Bash öncesi güvenlik kontrolü
    │   ├── post-edit-format.sh    # Dosya düzenleme sonrası format
    │   ├── pre-commit-check.sh    # Commit öncesi kontrol
    │   ├── session-start.sh       # Oturum açıldığında hoşgeldiniz özeti
    │   └── post-tooluse-test.sh   # Test komutları sonrası sonuç yorumu
    │
    ├── workflows/                 # Çok ajanlı orkestrasyon scriptleri
    │   ├── feature-dev.yaml       # Özellik geliştirme akışı
    │   └── bug-fix.yaml           # Hata düzeltme akışı
    │
    ├── output-styles/             # Takıma özel yanıt formatı
    │   └── technical-doc.md
    │
    ├── agent-memory/              # Alt ajanların kalıcı hafızası
    │   ├── code-reviewer/
    │   │   └── MEMORY.md
    │   └── security-auditor/
    │       └── MEMORY.md
    │
    ├── docs/                      # Ajanların başvurabileceği referans dokümanlar
    │   ├── architecture.md
    │   ├── coding-standards.md
    │   └── adr/
    │       ├── ADR-001.md
    │       └── ADR-002.md
    │
    ├── worktrees/                 # Claude Code tarafından oluşturulan worktree kayıtları
    │
    ├── sessions/                  # Uzun oturum özetleri (manuel)
    │   └── 2026-08-09-auth.md
    │
    └── logs/                      # Ajan çalışma logları
        └── parallel/
            └── agent-*.log
```

---

### B.3 Üst Seviye Proje Köşk Dosyaları

Proje kökünde olması tavsiye edilen tamamlayıcı dosyalar:

```
project-root/
├── CLAUDE.md                      # (yukarıda)
├── .claude/
├── .claudeignore                  # (yukarıda)
├── .mcp.json                      # MCP sunucu konfigürasyonu
├── .gitignore
├── TASK.md                        # (opsiyonel) tek seferlik görev tanımı, ajan başlatma
├── PROGRESS.md                    # (opsiyonel) uzun görevlerde ara durum özeti
├── FEEDBACK.md                    # (opsiyonel) deterministik döngüde hata geri beslemesi
├── handoff.md                     # (opsiyonel) strateji → uygulama katmanı el değişimi
└── package.json / pyproject.toml / Cargo.toml ...
```

---

### B.4 Yükleme Öncelik Sırası (Priority Hierarchy)

Çakışma durumunda hangi ayarın kazanacağı (en yüksek öncelik en üstte):

```
1. CLI flag'leri (örn: --max-turns, --permission-mode)
   ↓ en üstün
2. .claude/settings.local.json     (projede kişisel)
3. .claude/settings.json           (projede takım, committed)
4. ~/.claude/settings.local.json   (global kişisel)
5. ~/.claude/settings.json         (global)
   ↓
6. CLAUDE.local.md                 (projede kişisel)
7. CLAUDE.md                       (projede takım, committed)
8. ~/.claude/CLAUDE.md             (global kişisel)
9. Enterprise Managed Policy       (BT tarafından, kilitli, aşılamaz)
   ↓ en altta
```

Kurallar için de benzer bir hiyerarşi geçerlidir: proje `.claude/rules/` global `~/.claude/rules/` üzerine biner.

---

### B.5 Özellik Karar Rehberi: Hangi Özellik Nerede Kullanılır?

Claude Code'un 5 uzantı katmanı için "ne zaman hangisini kullanmalıyım":

| İhtiyacınız | Kullanılacak Özellik | Neden |
|:---|:---|:---|
| Her zaman geçerli olan kural ("asla `any` kullanma") | CLAUDE.md veya `.claude/rules/` | Her oturum/istekte otomatik yüklenir |
| Belirli bir prosedür ("deploy öncesi checklist") | Skill | Çağrıldığında yüklenir, ana context'i şişirmez |
| Dış araçlara bağlanmak (PostgreSQL, GitHub, Sentry) | MCP Sunucusu (.mcp.json) | Protokol seviyesinde entegrasyon |
| Bir komut çalışmadan önce otomatik kontrol (güvenlik) | Hook (PreToolUse) | Runtime seviyesinde engelleme, prompt'a güvenmez |
| Ana context'i kirletmeden yoğun araştırma/paralel iş | Subagent (`.claude/agents/`) | İzole pencerede çalışır, sadece özet döner |
| Birden fazla ajanın koordine çalışması | Agent Teams / Workflow | Paralel görev dağılımı ve P2P haberleşme |
| Takımınıza ayarlarınızı tek dosyada dağıtmak | Plugin | Tüm skill/agent/hook/ayarları tek paket |

**Pratik kural:**
- Eğer bir komut **her zaman** doğru olmalı → **Hook/Permission** (zorla yaptır)
- Eğer bilgi her zaman görünür olmalı → **CLAUDE.md/Rules**
- Eğer bir prosedür bazen gerekli → **Skill**
- Eğer iş ana oturumu kirletiyorsa → **Subagent**
- Eğer dış sistemle entegrasyon → **MCP**
- Eğer takımın her üyesinde aynı kurallar olsun → **Plugin** ile paketle

---

### B.6 `.mcp.json` Örnek Konfigürasyonu

MCP (Model Context Protocol) sunucuları Claude'a GitHub, veritabanı, Sentry, Slack vb. dış araçlara doğrudan erişim sağlar.

```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
      }
    },
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres", "${DATABASE_URL}"]
    },
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "./"]
    },
    "puppeteer": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-puppeteer"]
    }
  }
}
```

⚠️ MCP sunucuları eklerken güvendiğiniz kaynakları kullanın. Kötü niyetli bir MCP sunucusu ajan tarafından çalıştırılacak komutları etkileyebilir ve güvenlik riski oluşturur.

---

### B.7 Başlangıç İçin Minimum Dizin (İlk Hafta)

İlk kurulumda bütün yapıyı oluşturmak zorunda değilsiniz. İşte 15 dakikada kurabileceğiniz minimum:

```
project-root/
├── CLAUDE.md                   # Takım temel kuralları (30-80 satır yeterli)
├── .claudeignore               # 20 satırlık basit dışlama listesi
└── .claude/
    └── settings.json           # allow/deny/ask ile temel izinler
```

Özellikleri ihtiyaç duydukça ekleyin:
- Tekrarlayan bir iş mi yapıyorsunuz → Skill
- Dış araca mı ihtiyaç var → MCP
- Güvenlikten endişe mi ediyorsunuz → Hook
- Paralel/Araştırma işi mi var → Subagent

**Önemli:** CLAUDE.md'yi çok uzun tutmaktan kaçının. Claude Code dokümantasyonu ve topluluk tecrübesi uzun CLAUDE.md'lerin (500+ satır) hem cache performansını düşürdüğünü hem de Claude'un ortadaki kuralları kaçırmasına yol açtığını gösteriyor. Uzun açıklamaları Skill veya Rules dosyalarına bölün.
