## 5. FAZ 2: L2 → L3 — Takım Çapında Benimseme (Ay 2-3)

Bu faz, kurulumları **sizden takım arkadaşlarınıza** ölçeklendirmeyi ve lead olarak başkalarının da bu sistemi kullanabilmesi için altyapı kurmayı kapsar.

### 5.1 Organizasyonel CLAUDE.md Standartları

Takım olarak her proje için standart bir CLAUDE.md şablonu oluşturun ve bunu monorepo'nuzda veya organizasyon repo'nuzda paylaşın:

**`.github/CLAUDE_TEMPLATE.md` (organizasyon şablonu):**
```markdown
# [Proje Adı]

## Takım İletişimi
- Takım kanalı: #proj-[isim] (Slack/Discord)
- Tech Lead: [İsim]
- Onay gereken durumlar: Migration, yeni bağımlılık, mimari değişiklik

## Tech Stack
- [Burayı doldurun]

## Geliştirme Ortamı Kurulumu
```bash
[Kurulum komutları]
```

[... yukarıdaki şablonu devam ettirin]

## Takım Kuralları
- PR'lar en az 1 kişiden review almalı (Claude review'dan sonra!)
- Branch'ler 3 günden fazla açık kalmamalı
- WIP PR'lar "Draft" olarak açılır
- Her yeni feature için feature flag kullan
```

### 5.2 Handoff Mimarisi: Claude ↔ Codex/DeepSeek

Farklı modellerin farklı güçlü yanlarını kullanmak için **çift katmanlı bir üretim akışı** kurun.

#### Konsept

```
┌─────────────────────────────────────────────────────────────┐
│  STRATEJİK KATMAN (Yüksek Zeka)                             │
│  Claude Code (Opus/Sonnet)                                  │
│  - Mimari kararlar                                          │
│  - ADR yazma                                                │
│  - Karmaşık hata ayıklama                                   │
│  - İlk tasarım ve yapı iskeleti                             │
│  - Zor algoritmik problemler                                │
└────────────────┬────────────────────────────────────────────┘
                 │ Handoff Dokümanı (handoff.md)
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  UYGULAMA KATMANI (Yüksek Hacim, Düşük Zeka)                │
│  Codex / DeepSeek-Coder / Claude Haiku                      │
│  - Tekrarlayan boilerplate                                  │
│  - DTO/interface üretimi                                    │
│  - CRUD endpoint'leri                                       │
│  - Toplu refactor/rename                                    │
│  - Test iskeletleri yazma                                   │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│  DOĞRULAMA KATMANI (Zero-Context)                           │
│  Ayrı bir Claude/GPT oturumu                                │
│  - Testlerin geçtiğini doğrula                              │
│  - Hata ve güvenlik incelemesi                              │
└─────────────────────────────────────────────────────────────┘
```

#### Handoff Doküman Şablonu

Stratejik katman işini bitirip uygulama katmanına devrederken şu dosyayı oluşturur:

**`handoff.md`:**
```markdown
# HANDOFF: [Görev Adı]
**Tarih:** 2026-08-09
**Kaynak Model:** Claude Code (Generator)
**Hedef Model:** DeepSeek-Coder (Executor)

## Mevcut Durum
- İskelet dosyalar oluşturuldu: `src/auth/login.ts`, `src/auth/register.ts`
- Interface ve tipler tanımlandı: `src/types/auth.ts`
- Ana mimari kararlar verildi (JWT cookie-based, bcrypt ile hash)

## Tamamlananlar
- [x] Auth route yapısı kuruldu
- [x] Prisma User model şeması güncellendi
- [x] Login/Register için zod şemaları

## Kalan İş (Sen Yapacaksın)
1. `login.ts` içine gerçek fonksiyon implementasyonu:
   - email/şifre doğrulama
   - bcrypt.compare
   - JWT sign ve cookie set
2. `register.ts` içine:
   - bcrypt.hash (saltRounds: 12)
   - Kullanıcı oluşturma
   - Otomatik login (JWT set)
3. Tüm endpoint'ler için hata yönetimi (400, 401, 500)
4. Her endpoint için unit test

## Doğrulama Kriterleri
- [ ] `pnpm test src/auth` tüm testleri geçmeli
- [ ] `pnpm typecheck` hatasız
- [ ] E2E flow çalışıyor: kayıt ol → giriş yap → protected route eriş
- [ ] Hatalı şifre denemesinde generic hata mesajı (kullanıcı var mı yok mu belli olmasın)

## Kısıtlamalar ve Dikkat Edilecekler
- Rate limiting ekle: aynı IP'den 5 dk'da 5 deneme
- Password'ü asla loglama
- JWT secret process.env.JWT_SECRET'dan geliyor
- Cookie secure: true olmalı production'da
```

#### Handoff CLI Workflow (Basit Manuel Yöntem)
```bash
# 1. Claude ana oturumunda stratejiyi konuşlandır
claude
# ... iskeleti kur, mimariyi belirle ...
# Claude'a handoff.md yazdır

# 2. Derin düşünme oturumunu kapat (kompakt/cost kontrol)
# Claude son çıktıyı özetledikten sonra

# 3. Codex/DeepSeek oturumunu aç
# Codex CLI veya ayrı bir Claude (düşük model) ile el ile:
claude --model haiku
# "handoff.md dosyasını oku ve talimatları uygula"

# 4. Doğrulama oturumu (sıfır bağlam, ana model)
claude
# "src/auth/ dosyalarındaki yeni kodları incele ve doğrula"
```

### 5.3 CI/CD İçine Ajan Doğrulama Entegresyonu

PR açıldığında otomatik olarak Claude'un review yapmasını sağlayabilirsiniz.

#### GitHub Actions ile Otomatik AI Review

**`.github/workflows/ai-review.yml`:**
```yaml
name: AI Code Review
on:
  pull_request:
    types: [opened, synchronize]

jobs:
  ai-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Setup Claude Code Reviewer
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          # Native installer (npm v2.1.15+ itibarıyla deprecated)
          curl -fsSL https://claude.ai/install.sh | bash
          export PATH="$HOME/.local/bin:$PATH"
          # PR diff'ini al ve Claude'a inceleterek comment olarak ekle
          claude --print --max-budget-usd 2 "
          PR #${{ github.event.pull_request.number }} için kod incelemesi yap.
          
          Diff:
          $(git diff origin/${{ github.base_ref }}...HEAD)
          
          İnceleme kuralları:
          1. Hatalar ve potansiyel bug'lar
          2. Güvenlik açıkları
          3. Performans sorunları
          4. Test eksiklikleri
          5. Kod standardı ihlalleri
          
          Yorumları yapıcı olsun, her sorun için DOSYA:SATIR şeklinde referans ver.
          Eğer kritik bir hata yoksa kısaca 'LGTM' de ve neden iyi olduğunu özetle.
          " > review-comment.txt

      - name: Post Review Comment
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const review = fs.readFileSync('review-comment.txt', 'utf8');
            github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: context.issue.number,
              body: '## 🤖 AI Code Review\n\n' + review
            });
```

> **Not:** Bu basit bir örnektir. Üretimde Anthropic'un resmi PR review entegrasyonlarını veya `claude-code-action` gibi araçları kullanabilirsiniz.

### 5.4 Fleet Ops: 24/7 Çalışan Ajan Filosu

Uzun görevler (büyük refactor, test yazma kampanyası, dokümantasyon) için ajanlarınızı siz uyurken bile çalıştırmak amacıyla merkezi bir sunucu üzerinde sürekli çalışan tmux oturumları kurun.

#### Mimari

```
[Sizin Bilgisayar]                          [Bulut Sunucu / VPS]
(CLI etkileşimi)                            (7/24 çalışan)
       │                                           │
       ├─── SSH ─────────────────────────────────► │  tmux session 1: feat/auth
       │                                           │  tmux session 2: refactor/api
       └─ Tailscale ─────────────────────────────► │  tmux session 3: docs-generator
                                                   │
                                                   └─ Git worktree'ler ile izole
```

#### Kurulum Adımları

**1. Ucuz bir VPS kiralayın (5-10$/ay):**
- Hetzner CX22 veya DigitalOcean Basic Droplet (2 CPU, 4GB RAM) yeterli
- Ubuntu 24.04 kurulu olsun

**2. Tailscale ile güvenli erişim:**
```bash
# VPS üzerinde
curl -fsSL https://tailscale.com/install.sh | sh
sudo tailscale up
# Size bir URL verecek, tarayıcıda açın ve yetkilendirin

# Tailscale IP'sini öğrenin
tailscale ip -4
# Örn: 100.xx.xx.xx

# Artık kendi bilgisayarınızda Tailscale açıkken:
ssh root@100.xx.xx.xx
# Veya Tailscale MagicDNS ile:
ssh root@ajansunucu
```

**3. VPS üzerinde Claude Code kurulumu:**
```bash
# Native installer (Node.js gerektirmez, önerilen)
curl -fsSL https://claude.ai/install.sh | bash
export PATH="$HOME/.local/bin:$PATH"

# API Key'i kalıcı olarak ayarla
echo 'export ANTHROPIC_API_KEY="sk-ant-..."' >> ~/.bashrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# tmux ve gerekli araçlar
sudo apt install -y tmux git

# Kurulum doğrulama
claude --version     # en az v2.1.90 (güvenlik düzeltmeleri)
claude doctor
```

> Not: `npm install -g @anthropic-ai/claude-code` v2.1.15+ itibarıyla deprecated. Eski VPS imajlarında npm kurulumu görürseniz `claude install` ile native binary'e geçin.

**4. Paralel İşler İçin Worktree Yardımcı Scripti:**

**`~/bin/claude-task.sh`:**
```bash
#!/bin/bash
set -e

TASK_NAME=$1
BRANCH="task/$(date +%Y%m%d)-${TASK_NAME}"
WORKTREE_PATH="$HOME/tasks/${BRANCH}"

if [ -z "$TASK_NAME" ]; then
  echo "Kullanım: claude-task.sh <task-name>"
  echo "Örnek: claude-task.sh add-user-roles"
  exit 1
fi

# Repo'yu klonlamadıysan
if [ ! -d "$HOME/repo" ]; then
  echo "Önce $HOME/repo içine ana projeyi klonlayın."
  exit 1
fi

mkdir -p "$HOME/tasks"
cd "$HOME/repo"
git fetch origin main
git worktree add -b "$BRANCH" "$WORKTREE_PATH" origin/main

cd "$WORKTREE_PATH"

# Talimat dosyası oluşturma yardımı
if [ ! -f "TASK.md" ]; then
  cat > TASK.md << EOF
# Görev: ${TASK_NAME}

## Talimatlar
[Buraya yapilacak isi detayli yazin]

## Kabul Kriterleri
- [ ] Testler geçiyor: pnpm test
- [ ] Typecheck: pnpm typecheck
- [ ] Lint: pnpm lint

## Notlar
- Claude, bu görevi bitirdiğinde beni bekle.
- Her 30 dakikada bir progress kaydet.
EOF
  echo "TASK.md oluşturuldu. Lütfen düzenleyin: $WORKTREE_PATH/TASK.md"
  echo "Sonra şu komutla çalıştırın:"
  echo "  tmux new -s $TASK_NAME"
  echo "  cd $WORKTREE_PATH && claude"
  echo "  # Claude içinde: TASK.md'yi oku ve görevi yap"
  exit 0
fi

# Doğrudan tmux'ta başlat
tmux new-session -d -s "$TASK_NAME" "cd $WORKTREE_PATH && claude --print 'TASK.md oku ve görevi tamamla' 2>&1 | tee task.log"
echo "Ajan $TASK_NAME tmux oturumunda başlatıldı."
echo "Takip için: tmux attach -t $TASK_NAME"
```

```bash
chmod +x ~/bin/claude-task.sh
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
```

**Kullanım:**
```bash
# Yeni görev oluştur
claude-task.sh payment-refactor

# TASK.md'yi editörle düzenle, görevi tanımla
nano ~/tasks/task-20260809-payment-refactor/TASK.md

# Oturumu attach et ve çalıştır
tmux attach -t payment-refactor
```

**Mobil Müdahale İçin:**
- Telefonunuza Tailscale kurun (Android/iOS app'i var)
- Termius veya Blink Shell (iOS) veya JuiceSSH (Android) ile SSH bağlanın
- `tmux attach -t payment-refactor` ile oturuma girebilir, bekleyen onayları verebilirsiniz

### 5.5 Takım Geneli Dağıtım: Plugins ve Agent Teams

Takım olarak aynı kurulumu paylaşmanın en temiz yolu **Plugin** sistemi ve/veya organize `.claude/` klasörünü repoya commit etmektir.

#### Agent Teams (Araştırma önizlemesi — 2026)

Tek bir lead ajanın birden fazla "teammate" ajanı koordine etmesi. Subagent'tan farkı: teammate'ler birbirleriyle P2P haberleşir, ortak görev listesi ve mailbox paylaşır.

Açmak için:
```json
// .claude/settings.json (takım) veya ~/.claude/settings.json (kişi)
{ "env": { "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1" } }
```
Kontroller: Shift+Down (teammate'ler arası geçiş), Shift+Tab (delegate mode — lead sadece koordine eder), Ctrl+T (paylaşımlı görev listesi). Konfigürasyon `~/.claude/teams/<team-name>/config.json` altında tutulur.

3-5 teammate ile kullanılması önerilir; daha fazlası için yerel orkestratör (Bkz. Bölüm 10) tercih edin.

#### Plugin olarak paketleme

Takımınız standardı olgunlaştığında `.claude/` altındaki kural/skill/agent/hook/MCP'yi tek bir plugin arşivi olarak paketleyebilirsiniz. Tüm takım üyeleri tek komutla yükler:
```
/plugin marketplace add <repo-url>
/plugin install <plugin-adı>
```

> **Güvenlik notu:** Plugin'den gelen subagentlar hook, mcpServers ve permissionMode frontmatter alanlarını güvenlik nedeniyle yok sayar. Hassas hook/MCP'leri elle `.claude/agents/` içine kopyalayarak kullanın.

### 5.6 Maliyet İzleme Dashboard'u (Basit)

Ajan filonuzun tüketimini takip etmek için basit bir günlük tüketim log'u tutun:

**`~/bin/cost-log.sh`:**
```bash
#!/bin/bash
LOG="$HOME/costs/cost-$(date +%Y-%m).csv"
mkdir -p "$HOME/costs"
if [ ! -f "$LOG" ]; then
  echo "tarih,session,input_tokens,output_tokens,cost_usd,gorev" > "$LOG"
fi
echo "$@" >> "$LOG"
echo "Kaydedildi: $*"
```

> **L3 Kontrol Noktası:**
> - [ ] Organizasyon genelinde CLAUDE.md şablonu
> - [ ] Çift katmanlı (strateji/uygulama) üretim akışı
> - [ ] CI/CD içinde otomatik AI review
> - [ ] En az bir VPS üzerinde tmux ile 24/7 ajan
> - [ ] Tailscale mesh ağ kurulu
> - [ ] Maliyet takibi yapılıyor

---

