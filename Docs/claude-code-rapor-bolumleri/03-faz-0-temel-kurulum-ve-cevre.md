## 3. FAZ 0: Temel Kurulum ve Çevre Hazırlığı (Hafta 0)

### 3.1 Ön Koşullar

#### Donanım ve İşletim Sistemi
- **Tavsiye edilen:** macOS (Apple Silicon) veya Linux (Ubuntu 22.04+) — Claude Code CLI native binary olarak bu platformlarda en kararlı çalışır.
- **Windows:** PowerShell veya WSL2 üzerinden kullanın (native Windows desteği mevcuttur, WSL2 önerilir).
- **Terminal:** iTerm2 (macOS), Kitty/Alacritty (Linux), Windows Terminal (PowerShell) — tmux ile çalışırken performans önemli.
- **Editör:** VS Code (resmi entegrasyon en iyi), Neovim, JetBrains IDE'leri için Claude Code entegrasyonu da mevcuttur.
- **Node.js GEREKMEZ!** Ağustos 2026 itibarıyla Claude Code native binary olarak dağıtılıyor; npm kurulumu v2.1.15'te deprecated oldu.

#### Hesaplar ve API/Abonelik Anahtarları

| Servis | Amaç | Maliyet (Başlangıç) |
|:---|:---|:---|
| Claude Pro aboneliği | Ana kullanım (CLI + web) | $20/ay |
| Claude Max (opsiyonel) | Yoğun kullanım, yüksek limit | $100-200/ay |
| Anthropic Console API | Otomasyon/scriptler/arka plan ajanları | Kullanım bazlı, ~$20-50/ay |
| (Opsiyonel) OpenAI API | Zero-Context Reviewer (GPT) | ~$20-50/ay |
| (Opsiyonel) DeepSeek API | Ucuz boilerplate kod üretimi | ~$5-20/ay |
| (Opsiyonel) Tailscale | Fleet ops mesh ağ | Ücretsiz (100 cihaza kadar) |
| Git ve GitHub/GitLab | Versiyon kontrolü ve PR | Zaten vardır |

### 3.2 Claude Code Kurulumu (Güncel, 2026 v2.1.x+)

#### Resmi Kurulum Yöntemi (Önerilen — Native Installer)
Kurulum için Node.js veya başka bir ön şart gerekmez. Native binary otomatik olarak `~/.local/bin/claude` konumuna yüklenir ve arka planda kendini günceller.

**macOS / Linux / WSL2:**
```bash
curl -fsSL https://claude.ai/install.sh | bash
```

**Windows PowerShell:**
```powershell
irm https://claude.ai/install.ps1 | iex
```

**Homebrew (macOS / Linux):**
```bash
# Stable kanal (yaklaşık 1 hafta geriden, daha güvenli)
brew install --cask claude-code

# Latest kanal (en yeni özellikler)
brew install --cask claude-code@latest
```

**WinGet (Windows):**
```powershell
winget install Anthropic.ClaudeCode
```

**Linux paket yöneticileri (Ubuntu/Debian):**
```bash
sudo install -d -m 0755 /etc/apt/keyrings
sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
  -o /etc/apt/keyrings/claude-code.asc
echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
  | sudo tee /etc/apt/sources.list.d/claude-code.list
sudo apt update && sudo apt install claude-code
```

> **Not:** Eski npm kurulumu (`npm install -g @anthropic-ai/claude-code`) artık deprecation uyarısı veriyor. Native yükleyiciye geçmek için: `claude install` veya `curl` ile yeniden yükleme yapın.

Kurulumu doğrulama:
```bash
claude --version
claude doctor  # Sistem sağlık kontrolü
```

İlk çalıştırmada tarayıcınızda otomatik bir yetkilendirme sayfası açılır. Claude Pro/Max/Team hesabınızla giriş yapın, veya sağ alttan API key ile kullanımı seçin.

#### Sürüm Sabitleme ve Güncellemeler
- Native kurulum **varsayılan olarak arka planda otomatik güncellenir.**
- Belirli bir sürümü sabitlemek için:
  ```bash
  curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89  # Belirli sürüm
  curl -fsSL https://claude.ai/install.sh | bash -s stable  # Stable kanala kilitle
  ```
- Manuel güncelleme: `claude update`

#### CLI Flag'ler (Temel)
```bash
claude                           # İnteraktif REPL modu (normal kullanım)
claude "görev açıklaması"        # Tek seferlik görev çalıştır ve çık
claude -p "sorgu"                # Non-interactive (pipe/script için)
claude -c                        # Bu dizindeki en son konuşmaya devam et
claude -r                        # Önceki oturumlardan seçip devam et
claude --worktree <branch>       # Yeni bir git worktree oluşturup orada başlat
claude --bg "görev"              # Arka planda ajan başlat
claude agents                    # Tüm çalışan/bloke/tamamlanmış oturumları listele
claude -p --max-turns 5          # Maksimum 5 tur
claude -p --max-budget-usd 2     # Maksimum $2 harcama limiti
claude --permission-mode plan    # Plan modu (öneride bulunur, değişiklik yapmaz)
claude mcp list                  # MCP sunucularını listele
claude project purge             # Bu projeye ait tüm Claude state'ini sil
```

#### IDE Entegrasyonu

**VS Code (Resmi):**
- Claude Code extension'ı Market Place'ten yükleyin.
- Komut Paleti → "Claude Code: Open" ile açın.
- Terminaldeki ile aynı oturumu paylaşır, arayüzden dosya seçimi yapabilirsiniz.

**JetBrains IDE'leri (IntelliJ, WebStorm, vb.):**
- Resmi Claude Code eklentisi mevcut, benzer mantıkta çalışır.

**Vim/Neovim:**
- Harici bir eklenti gerekmez. Terminal split'inde `claude` çalıştırma alışkanlığı edinin.
- `:terminal claude` ile entegre edebilirsiniz.

### 3.3 Terminal Ortamı Yapılandırması

#### tmux Kurulumu (Vakit Kaybetmeden Kurun!)
Claude Code oturumları saatlerce sürebilir. Bilgisayarınız uyuduğunda veya SSH bağlantınız koptuğunda iş kaybetmemek için **tmux şarttır**.

```bash
# macOS
brew install tmux

# Ubuntu/Debian
sudo apt install tmux
```

**Temel tmux config (`~/.tmux.conf`):**
```bash
cat > ~/.tmux.conf << 'EOF'
set -g mouse on
set -g history-limit 50000
set -g default-terminal "screen-256color"
set -g prefix C-a
bind r source-file ~/.tmux.conf \; display "Reloaded!"
# Pane splitting
bind | split-window -h
bind - split-window -v
# Vim benzeri pane navigation
bind h select-pane -L
bind j select-pane -D
bind k select-pane -U
bind l select-pane -R
# Yeniden boyutlandırma
bind H resize-pane -L 5
bind J resize-pane -D 5
bind K resize-pane -U 5
bind L resize-pane -R 5
EOF
```

**Tmux Temel Komutları:**
```bash
tmux new -s claude-main      # "claude-main" isimli yeni oturum
tmux attach -t claude-main   # Var olan oturuma bağlan
tmux ls                      # Tüm oturumları listele
tmux kill-session -t <isim>  # Oturumu kapat
# İçeride: Ctrl+a sonra d → detach (çıkmadan arka planda bırak)
```

#### Shell Helpers (Kısayollar)
```bash
# ~/.bashrc veya ~/.zshrc içine ekleyin
alias c="claude"
alias cc="claude -c"           # Son oturuma devam
alias cb="claude --bg"         # Arka plan ajanı
alias cl="claude agents"       # Aktif ajanlar
alias ct='tmux new-session -A -s claude'
alias cs='claude squad'        # Paralel ajanlar (Claude Squad kuruluysa)
```

### 3.4 CLAUDE.md Dosya Hiyerarşisi

Claude Code'un en önemli özelliği, birden fazla konumdan otomatik olarak kural dosyalarını yüklemesidir. 2026'da kesin hiyerarşi (en yüksek öncelik en üstte):

```
Öncelik (en yüksek → en düşük)
┌─────────────────────────────────────────────┐
│  CLAUDE.local.md (proje kökü)               │ Sadece size özel, gitignore'da
│  ↑ üzerine biner                            │
├─────────────────────────────────────────────┤
│  CLAUDE.md (proje kökü veya .claude/)       │ Takım talimatları, git'e push'lanır
│  ↑ üzerine biner                            │
├─────────────────────────────────────────────┤
│  ~/.claude/CLAUDE.md                        │ Kişisel global ayarlar
│  ↑ üzerine biner                            │
├─────────────────────────────────────────────┤
│  Enterprise Managed Policy                  │ Kurumsal BT tarafından kilitli
└─────────────────────────────────────────────┘
```

Ek olarak `.claude/rules/` dizinine `.md` dosyaları koyarak modüler kurallar tanımlayabilirsiniz. Claude, `.claude/rules/` altındaki tüm `.md` dosyalarını otomatik yükler; alt dizinler yol bazlı koşullu yükleme yapar (örn: `.claude/rules/frontend/react-patterns.md` sadece `frontend/` dizininde çalışırken aktif).

#### İlk Proje CLAUDE.md (Minimum Başlangıç)
Proje köküne yerleştirin:

```markdown
# Proje Adı: [Proje İsmi]

## Kim Bu Proje?
- **Tech Stack:** Next.js 15, TypeScript, PostgreSQL (Prisma), Redis
- **Mimari:** Monorepo (pnpm workspaces), `apps/web`, `apps/api`, `packages/shared`
- **Dil:** Türkçe yorumlar, İngilizce kod ve commit mesajları

## Kod Standartları
- TypeScript strict mode aktif
- React'ta Server Component önceliği, client component'ler `"use client"` ile işaretlenmeli
- Test: Vitest + React Testing Library, yeni kod için zorunlu
- Stil: Tailwind CSS, özel CSS yok
- Lint/Format: ESLint + Prettier
- Migration'lar `pnpm prisma migrate dev` ile oluşturulur, elle düzenlenmez

## Çalışma Komutları
- `pnpm dev` — Tüm uygulamaları development modunda çalıştır
- `pnpm build` — Production build
- `pnpm test` — Tüm testler
- `pnpm lint` — ESLint
- `pnpm typecheck` — TypeScript tip kontrolü
- `pnpm db:migrate` — Prisma migration uygula

## Güvenlik Kuralları
- `.env` dosyalarını ASLA okuma/yazma
- `node_modules`, `.next`, `dist` dizinlerine dokunma
- `DROP`, `TRUNCATE`, `DELETE FROM` onaysız çalıştırma
- Yeni npm paketi eklediğinde önce bildir
- Git'e secret/API key commit'leme
- `git push --force` kullanma

## Çalışma Prensipleri
- Önce test yaz, sonra kod (TDD tercih edilir)
- Kodu değiştirdiğinde etkilenen tüm yerleri güncelle
- Emin olmadığın şeyleri sor, varsayım yapma
- 3 başarısız düzeltmede dur ve özetle
- Context şiştiğinde `/compact` öner
```

#### Global Kişisel CLAUDE.md (`~/.claude/CLAUDE.md`)
Tüm projelerde geçerli kişisel tercihleriniz:

```markdown
# Global CLAUDE.md — Tüm Projelerde Geçerli

## İletişim Tarzı
- Açık, net, doğrudan iletişim. Gereksiz nezaket cümleleri kullanma.
- Hata varsa doğrudan söyle.
- Kod bloğundan önce ne yapıldığını 1 cümle ile özetle.
- Türkçe yanıt ver; teknik terimleri İngilizce bırak.

## Mühendislik Prensipleri
- Önce çalışır kod, sonra güzel kod.
- Erken hata ver (fail fast).
- Yeni fonksiyonlar için mutlaka test.
- Kendini incele: kodunu bitirdiğinde sıfır bağlamla incele.
- Büyük değişikliklerden önce mevcut testlerin geçtiğini doğrula.
- 2'den fazla düzeltme denemesi başarısız olursa, yaklaşımı değiştir.

## Asla Yapmaman Gerekenler
- .env dosyalarına erişme
- Kilit dosyalarını elle düzenleme (pnpm-lock.yaml, package-lock.json)
- İnsana sormadan production'a yönelik komutlar çalıştırma
```

### 3.5 İlk `.claude/` Klasör Yapısı

Kurulum tamamlandığında tavsiye edilen başlangıç klasör yapısı:

```
project-root/
├── CLAUDE.md                   # Takım kuralları (committed)
├── CLAUDE.local.md             # Kişisel (gitignored)
├── .mcp.json                   # MCP sunucu ayarları (sonraki bölümlerde)
├── .claude/
│   ├── settings.json           # İzinler ve ayarlar (committed)
│   ├── settings.local.json     # Kişisel izinler (gitignored)
│   ├── rules/                  # Modüler kurallar
│   │   ├── code-style.md
│   │   ├── testing.md
│   │   └── api-conventions.md
│   ├── skills/                 # Yeniden kullanılabilir iş akışları
│   │   └── <skill-name>/SKILL.md
│   ├── agents/                 # Özel subagent persona'ları (sonraki bölümlerde)
│   └── hooks/                  # Yaşam döngüsü scriptleri (güvenlik için)
└── .claudeignore               # Claude'un hiç görmeyeceği dosyalar
```

#### `.claudeignore` (İlk Versiyon)
`.gitignore` benzeri, Claude'un taramayacağı dosyaları listeler:

```
# Secrets
.env
.env.*
!.env.example
*.pem
*.key
secrets/

# Bağımlılıklar
node_modules/
vendor/

# Build çıktıları
.next/
dist/
build/
out/
target/

# Büyük veri
*.sqlite
*.db
data/
backups/

# Hassas
.git/
.ssh/
.aws/
```

#### Minimum `settings.json` (Başlangıç)
`.claude/settings.json` içine takımın paylaşmalı temel izinleri:

```json
{
  "permissions": {
    "allow": [
      "Bash(pnpm test*)",
      "Bash(pnpm lint*)",
      "Bash(pnpm typecheck*)",
      "Bash(git status*)",
      "Bash(git diff*)",
      "Bash(git log*)",
      "Read(*)"
    ],
    "deny": [
      "Bash(*DROP*)",
      "Bash(*TRUNCATE*)",
      "Bash(*rm -rf*)",
      "Bash(*git push --force*)",
      "Bash(*sudo*)",
      "Bash(*chmod 777*)",
      "Write(.env*)"
    ],
    "ask": [
      "Bash(pnpm install*)",
      "Bash(git commit*)",
      "Bash(*prisma migrate*)",
      "Bash(*docker run*)"
    ]
  }
}
```

Bu ayar ile:
- Test/lint/typecheck gibi güvenli komutlar otomatik çalışır
- Tehlikeli komutlar (`rm -rf`, `DROP`, `force push`) doğrudan engellenir
- Bağımlılık yükleme, commit, migration gibi önemli komutlar için sizden onay ister.

> **L0 → L1 Geçiş Kontrol Noktası:** Bu kurulumu bitirdiğinizde, Claude Code ile güvenli ve üretken bir başlangıç yapabilecek donanıma sahipsiniz demektir. Bir sonraki bölümde alışkanlıkları ve ileri komutları pekiştireceğiz.
