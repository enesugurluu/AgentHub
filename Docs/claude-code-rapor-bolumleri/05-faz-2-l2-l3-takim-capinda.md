## 5. FAZ 2: L2 → L3 — Takım Çapında Benimseme (Ay 2-3)

Bu faz, kurulumları **sizden takım arkadaşlarınıza** ölçeklendirmeyi ve lead olarak başkalarının da bu sistemi kullanabilmesi için altyapı kurmayı kapsar. Bu seviyede Claude Code'un kurumsal özellikleri (Plugins, Agent Teams, MCP Paylaşımı) devreye girer.

### 5.1 Organizasyonel Standartlar ve Plugin Paketleme

Takım olarak her üyenin aynı kuralları kullanmasını elle CLAUDE.md kopyalamak yerine **Plugin** (eki paket) olarak paketleyin. Pluginler skill, hook, agent, MCP ve settings dosyalarını tek bir dağıtılabilir birimde birleştirir.

**Takım için kurumsal plugin yapısı:**
```
org-claude-config/         (ayrı bir git repo'su)
├── plugin.json            # Plugin manifestosu
├── CLAUDE.md              # Kurumsal temel talimatlar
├── skills/                # Tüm takımın kullanacağı beceriler
│   ├── deploy/
│   │   └── SKILL.md
│   ├── tdd/
│   │   └── SKILL.md
│   └── security-review/
│       └── SKILL.md
├── agents/                # Takım subagentları
│   ├── code-reviewer.md
│   └── security-auditor.md
├── rules/                 # Modüler kurallar
│   ├── code-style.md
│   └── testing.md
├── hooks/                 # Takım güvenlik hook'ları
│   └── pre-bash.sh
└── mcp.json               # Paylaşılan MCP sunucular
```

`plugin.json` örneği:
```json
{
  "name": "@sirket/claude-standard",
  "version": "1.2.0",
  "description": "Şirketimiz tüm projeleri için standart Claude Code kurulumu",
  "skills": ["skills/*/SKILL.md"],
  "agents": ["agents/*.md"],
  "rules": ["rules/*.md"],
  "hooks": [
    {"event": "PreToolUse", "matcher": "Bash", "script": "hooks/pre-bash.sh"}
  ],
  "mcp": "mcp.json"
}
```

Takım arkadaşlarınız bu plugini tek komutla kurabilir:
```bash
claude plugin add github:sirket/claude-standard
```

Bu sayede her yeni projede sıfırdan kurulum yapmak yerine standart yapılandırma otomatik olarak gelir; güncellemeler merkezi olarak yapılır ve tüm takıma anında yayılır.

### 5.2 CLAUDE.md Şablonu ve Organizasyon Hiyerarşisi

Merkezi plugin'in yanı sıra her proje için standart bir CLAUDE.md şablonunu `.github/CLAUDE_TEMPLATE.md` olarak repoda tutun. Şablon en az şu bölümleri içermeli:

```markdown
# [Proje Adı]

## Takım İletişimi
- Takım kanalı: #proj-[isim]
- Tech Lead: [İsim]
- Onay gereken durumlar: Migration, yeni bağımlılık, mimari değişiklik

## Tech Stack
- Tam listeleme

## Geliştirme Ortamı Kurulumu
\`\`\`bash
[Kurulum komutları]
\`\`\`

## Mimari Genel Bakış
[Kısa açıklama]

## Dizin Yapısı
[Kritik dizinler]

## Sık Kullanılan Komutlar
- Kurulum, test, lint, build

## Kod Standartları
- Biçim, test, isimlendirme kuralları

## Güvenlik Kuralları
- Kesin yasaklar

## Takım Workflow'u
- Branch, PR, review kuralları
```

**Kurumsal Politika (Managed Policy):** Enterprise/Team plan kullanıyorsanız BT tarafından dağıtılan ve kullanıcıların aşılamayacağı merkezi bir politika dosyası (`/etc/claude-code/managed-settings.json`) tanımlayabilirsiniz. Bu, özellikle güvenlik ve gizlilik kuralları için kullanılır.

### 5.3 Handoff Mimarisi: Claude ↔ Codex/DeepSeek/Haiku

Farklı modellerin farklı güçlü yanlarını kullanmak için **çift katmanlı bir üretim akışı** kurun. Bu strateji hem maliyeti düşürür hem de kör nokta riskini azaltır.

#### Konsept

```
┌─────────────────────────────────────────────────────────────┐
│  STRATEJİK KATMAN (Yüksek Zeka)                             │
│  Claude Opus 5 / Sonnet 5                                   │
│  - Mimari kararlar                                          │
│  - ADR yazma                                                │
│  - Karmaşık hata ayıklama                                   │
│  - İlk tasarım ve yapı iskeleti                             │
│  - Zor algoritmik problemler                                │
└────────────────┬────────────────────────────────────────────┘
                 │ Handoff Dokümanı (handoff.md)
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  UYGULAMA KATMANI (Yüksek Hacim, Düşük Zeka, Ucuz)          │
│  Claude Haiku / DeepSeek-Coder / Codex CLI                  │
│  - Tekrarlayan boilerplate                                  │
│  - DTO/interface üretimi                                    │
│  - CRUD endpoint'leri                                       │
│  - Toplu refactor/rename                                    │
│  - Test iskeletleri yazma                                   │
│  - Dokümantasyon                                            │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  DOĞRULAMA KATMANI (Zero-Context / Adversarial Review)      │
│  Ayrı oturum veya farklı model (GPT-5 / farklı Claude oturumu)│
│  - Testlerin geçtiğini doğrula                              │
│  - Adversarial güvenlik incelemesi                          │
│  - Spec uyumunu denetle                                     │
└─────────────────────────────────────────────────────────────┘
```

Model değiştirmeyi kolaylaştırmak için `/model` komutunu veya `--model` CLI bayrağını kullanın. Handoff sonrası düşük modele geçip işi bitirip tekrar ana modele dönmek sadece 2-3 komuttur.

#### Handoff Doküman Şablonu

Stratejik katman işini bitirip uygulama katmanına devrederken bu dosyayı oluşturur:

**`handoff.md`:**
```markdown
# HANDOFF: [Görev Adı]
**Tarih:** 2026-08-09
**Kaynak Model:** Claude Sonnet 5 (Strateji/İskelet)
**Hedef Model:** Claude Haiku / DeepSeek-Coder (Uygulama)

## Mevcut Durum
- İskelet dosyalar oluşturuldu: `src/auth/login.ts`, `src/auth/register.ts`
- Interface ve tipler tanımlandı: `src/types/auth.ts`
- Ana mimari kararlar verildi (JWT cookie-based, bcrypt ile hash)

## Tamamlananlar
- [x] Auth route yapısı kuruldu
- [x] Prisma User model şeması güncellendi
- [x] Login/Register için zod şemaları

## Kalan İş (Sen Yapacaksın)
1. `login.ts` içine gerçek fonksiyon implementasyonu
2. `register.ts` içine kayıt mantığı
3. Tüm endpoint'ler için hata yönetimi (400, 401, 500)
4. Her endpoint için unit test

## Doğrulama Kriterleri
- [ ] `pnpm test src/auth` tüm testleri geçmeli
- [ ] `pnpm typecheck` hatasız
- [ ] E2E flow çalışıyor
- [ ] Hatalı şifre denemesinde generic hata mesajı

## Kısıtlamalar
- Rate limiting ekle: aynı IP'den 5 dk'da 5 deneme
- Password'ü asla loglama
- JWT secret process.env.JWT_SECRET'dan geliyor
- Cookie secure: true production'da
```

#### Handoff Workflow (Pratik)
```bash
# 1. Ana oturumda stratejiyi ve iskeleti oluştur (Sonnet/Opus)
claude --model sonnet
# ... iskelet kur, mimari kararlar al, handoff.md yazdır ...

# 2. Strateji oturumundan çık (compact/clear veya sadece model değiştir)
/model haiku
# veya tamamen ayrı bir oturum: claude --model haiku

# 3. Ucuz modelle uygulamayı yaptır
"handoff.md'yi oku ve talimatları uygula."

# 4. Doğrulama: tekrar ana modele geç
/model sonnet
# veya sıfır bağlam yeni oturumda
# "@code-reviewer src/auth/ dosyalarını incele" subagent'ını çağır
```

### 5.4 Agent Teams ve Paralel İş Orkestrasyonu (Yerleşik Özellik)

Claude Code v2.1+ sürümünde **Agent Teams** özelliği bulunur: birden fazla ajan P2P haberleşerek aynı görev üzerinde koordine çalışabilir. Manuel tmux ve worktree yönetimine göre daha entegre bir deneyim sunar.

**Agent Team tanımı (`.claude/teams/feature-dev/config.json`):**
```json
{
  "name": "feature-dev",
  "agents": [
    {"role": "architect", "model": "opus", "skills": ["architecture"]},
    {"role": "implementor", "model": "sonnet", "skills": ["tdd", "coding"]},
    {"role": "reviewer", "model": "gpt-5*", "tools": ["Read", "Grep", "Bash:test"]}
  ],
  "workflow": "sequential-parallel",
  "isolation": "worktree"
}
```

*GPT-5 gibi üçüncü parti modeller için MCP veya API köprüsü gerekir.

Takımı çalıştırmak:
```bash
claude team run feature-dev "Kullanıcıya rol yönetimi ekle"
```

#### Alternatif: Claude Squad (Açık Kaynak Orkestratör)
Resmi özelliği kullanmak istemiyorsanız topluluk tarafından geliştirilen **Claude Squad** aracı tek başına geliştiriciler için en olgun seçenektir. TUI arayüzü ile birden fazla ajan oturumunu yönetir, otomatik worktree oluşturur, ve tmux üzerinde çalışır.

Kurulum:
```bash
brew install claude-squad
# veya
curl -fsSL https://raw.githubusercontent.com/smtg-ai/claude-squad/main/install.sh | bash
cs  # TUI'yu başlat
```

Diğer popüler orkestratörler:
- **Composio AO:** Web dashboard, mileston gate'ler, otomatik CI retry
- **Baton:** GitHub Issue odaklı poll-dispatch-reconcile döngüsü
- **Vibe Kanban:** Kanban panosu üzerinden 10+ farklı ajan desteği
- **GitHub Copilot App (resmi):** Her ajan otomatik worktree'de çalışır, masaüstü kontrol merkezi (Copilot kullanıcıları için)

### 5.5 Worktree Runtime İzolasyonu (Önemli Detay)

Git worktree'ler dosya sistemi izolasyonu sağlar ama **çalışma zamanı çakışmalarını çözmez**:

| İzolasyon          | Worktree çözer mü? | Çözüm                               |
|:-------------------|:-------------------|:------------------------------------|
| Dosya sistemi çakışması | ✅ Evet | Worktree başına ayrı dizin |
| Port çakışması (3000, 5432 vb.) | ❌ Hayır | Her worktree için ayrı port (.env.local) |
| Veritabanı çakışması | ❌ Hayır | İş başına ayrı test veritabanı |
| node_modules tekrarı | ❌ Hayır | pnpm ile store paylaşımı (workspace ile) |
| `.env` paylaşımı | ❌ Kısmen | Worktree başına `.env.local` (gitignored) |

Uygulama örneği:
```bash
# Her worktree için farklı port ve DB
cat > ../myapp-auth/.env.local << 'EOF'
PORT=3001
DATABASE_URL="postgresql://user:pass@localhost:5432/myapp_auth"
EOF

cat > ../myapp-payment/.env.local << 'EOF'
PORT=3002
DATABASE_URL="postgresql://user:pass@localhost:5432/myapp_payment"
EOF
```

Aksi halde iki ajanın biri 3000 portunda çalışırken diğeri çakışır veya aynı veritabanına yazarak birbirinin test verisini bozar.

### 5.6 CI/CD İçine Ajan Doğrulama Entegrasyonu

PR açıldığında otomatik adversarial review çalıştırmak için:

- **Önerilen:** Anthropic'in resmi `claude-code-action` GitHub Action'ı (en güncel)
- Basit olması için manuel workflow (aşağıda)

**`.github/workflows/ai-review.yml`:**
```yaml
name: AI Code Review
on:
  pull_request:
    types: [opened, synchronize, reopened, ready_for_review]

permissions:
  contents: read
  pull-requests: write

jobs:
  adversarial-review:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Claude Code (native)
        run: |
          curl -fsSL https://claude.ai/install.sh | bash
          echo "$HOME/.local/bin" >> $GITHUB_PATH

      - name: Run adversarial review (zero-context)
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          REVIEW_BASE: origin/${{ github.base_ref }}
        run: |
          DIFF=$(git diff $REVIEW_BASE...HEAD)
          claude -p --max-turns 3 --model sonnet \
            --system-prompt "Sen sıfır-bağlam adversarial kod denetçisisin.
            Sadece diff'i görüyorsun, hiçbir bağlamın yok. Şunları ara:
            bug, güvenlik açığı, race condition, test eksikliği,
            belirgin hata. Yorumları DOSYA:SATIR formatında ver.
            LGTM demeden önce iki defa düşün." \
            "PR diff incelemesi:\n$DIFF" > review-comment.txt

      - name: Post review comment
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: context.issue.number,
              body: '## 🤖 Adversarial AI Review\n\n' + fs.readFileSync('review-comment.txt', 'utf8')
            });
```

> **Not:** Reviewer için farklı bir model kullanmak (GPT-5 gibi) çapraz kontrol sağlar ve "aynı model aynı kör noktayı görür" riskini azaltır. Endüstri buna **maker-checker ayrımı** diyor.

### 5.7 Fleet Ops: 24/7 Çalışan Ajan Filosu

Uzun görevler (büyük refactor, test yazma kampanyası, dokümantasyon) için ajanlarınızı siz uyurken bile çalıştırmak amacıyla merkezi bir sunucu üzerinde sürekli çalışan tmux oturumları kurun.

#### Mimari

```
[Sizin Bilgisayar/Telefon]                    [Bulut VPS / Sunucu]
(CLI kontrolü)                                (7/24 çalışır)
     │                                              │
     ├─ SSH ─────────────────────────────────────►  │ tmux oturumları
     └─ Tailscale (zero-trust) ──────────────────►  │ Git worktree'ler
                                                    │ Claude arka plan ajanları
```

#### VPS Kurulumu (Güncel)

**1. Ucuz bir VPS (5-10$/ay):**
- Hetzner CX22 veya DigitalOcean Basic (2 CPU, 4GB RAM) yeterli
- Ubuntu 24.04 tercih edin

**2. Claude Code'u Native Olarak Kurun:**
```bash
# Node gerekmez! Native installer kullanın:
curl -fsSL https://claude.ai/install.sh | bash
# API key ile kullanacaksanız:
echo 'export ANTHROPIC_API_KEY="sk-ant-..."' >> ~/.bashrc
```

**3. Tailscale VPN:**
```bash
curl -fsSL https://tailscale.com/install.sh | sh
sudo tailscale up
# Tarayıcıda açılan URL'de yetkilendirin
```

**4. tmux ve Git:**
```bash
sudo apt install -y tmux git
```

#### Görev Yardımcı Scripti (Güncel)

`~/bin/claude-task.sh`:
```bash
#!/bin/bash
set -e
TASK_NAME=$1
BRANCH="task/$(date +%Y%m%d)-${TASK_NAME}"
WT_PATH="$HOME/tasks/${BRANCH}"

if [ -z "$TASK_NAME" ]; then
  echo "Kullanım: claude-task.sh <task-name>"
  exit 1
fi

mkdir -p "$HOME/tasks"
cd "$HOME/repo"
git fetch origin main
git worktree add -b "$BRANCH" "$WT_PATH" origin/main

cd "$WT_PATH"

# Her worktree için benzersiz port/DB ayarla (runtime izolasyon)
echo "PORT=3$((RANDOM % 900 + 100))" > .env.local
echo "DATABASE_URL=postgresql://app:app@localhost:5432/task_${TASK_NAME}" >> .env.local

if [ ! -f TASK.md ]; then
  cat > TASK.md << 'EOF'
# Görev
## Talimatlar
[Yapılacak iş]

## Kabul Kriterleri
- [ ] Testler geçiyor
- [ ] Typecheck/lint temiz
- [ ] Maksimum 6 turda çözemezsen durup raporla
EOF
  echo "✓ TASK.md oluşturuldu: $WT_PATH/TASK.md"
  echo "Düzenledikten sonra şununla başlat:"
  echo "  tmux new -s $TASK_NAME"
  echo "  cd $WT_PATH && claude"
  exit 0
fi

# Maksimum tur ve bütçe limitiyle arka planda başlat
tmux new-session -d -s "$TASK_NAME" \
  "cd '$WT_PATH' && claude -p --max-turns 10 --max-budget-usd 5 'TASK.md oku ve görevi yap' 2>&1 | tee task.log"
echo "✓ Ajan başlatıldı: $TASK_NAME"
echo "Takip: tmux attach -t $TASK_NAME"
```

```bash
chmod +x ~/bin/claude-task.sh
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
```

#### Arka Plan Ajanlar (Yerleşik Özellik)
Ayrıca v2.1.154+ sürümünde yerleşik `--bg` bayrağı ile tmux kullanmadan da arka plan ajanı çalıştırabilirsiniz:
```bash
claude --bg "Tüm dosyalardaki TODO'ları bul ve raporla"
claude agents   # Çalışan/tamamlanan ajanları listele
```

#### Mobil Müdahale
- Telefonunuza Tailscale kurun
- Termius/Blink Shell (iOS) veya JuiceSSH (Android) ile SSH bağlanın
- `tmux attach -t <isim>` ile oturuma girip onayları verebilirsiniz

### 5.8 Maliyet İzleme

- `/usage` komutu ile oturum bazında tüketim
- Anthropic Console üzerinden proje ve API key bazında raporlama
- Console'da hard limit belirleyin (örn: ayda $200)'yi geçerse dursun)
- Team/Enterprise planlarında yönetici paneli kullanıcı başına tüketimi gösterir

> **L3 Kontrol Noktası:**
> - [ ] Organizasyon için ortak plugin paketi oluşturuldu ve dağıtıldı
> - [ ] Çift katmanlı (strateji/uygulama) üretim akışı çalışıyor
> - [ ] Adversarial review CI/CD'de otomatik
> - [ ] En az bir VPS üzerinde sürekli çalışan ajan filosu mevcut
> - [ ] Tailscale ile güvenli erişim
> - [ ] Worktree'ler arası runtime izolasyon (port/.env) ayarlandı
> - [ ] Maliyet limiti ve takibi aktif
> - [ ] Takım üyeleri eğitildi ve ortak kullanım başladı
