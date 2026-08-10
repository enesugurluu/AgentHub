## 3. FAZ 0: Temel Kurulum ve Çevre Hazırlığı (Hafta 0)

> **Bu bölüm Ağustos 2026 için günceldir.** Claude Code v2.1.x itibarıyla npm kurulumu **deprecated**; resmi olarak tavsiye edilen yöntem **native binary installer**. Node.js sadece npm ile kurulum yapacaksanız gerekli (o da artık Node 22+ gerektiriyor) — native kurulum Node'suz çalışır.

### 3.1 Ön Koşullar

#### Donanım ve İşletim Sistemi
- **Tavsiye edilen:** macOS (Apple Silicon) veya Linux (Ubuntu 22.04+) — native installer bu platformlarda en kararlı.
- **Windows:** PowerShell ile native kurulum (`irm ... | iex`); eski sürümlerde WSL2 önerilirdi ama artık native Windows desteği tam.
- **Terminal:** iTerm2 (macOS), Kitty/Alacritty/Hyper (Linux), Windows Terminal.
- **Editör:** VS Code (resmi Claude Code extension) veya Neovim (terminal yanında).
- **Git (zorunlu):** Claude Code worktree, branch ve diff özellikleri için git şart. `git --version` 2.30+ olmalı.

#### Hesaplar ve API Anahtarları

| Servis | Amaç | Maliyet (Başlangıç) |
|:---|:---|:---|
| Claude.ai hesabı (Pro/Max) | Claude Code ana giriş yöntemi | Pro: $20/ay (başlangıç için yeterli) |
| Anthropic Console API key | CLI/CI/CD ve arka plan ajanları için | Kullanım bazlı, aylık hard limit koyun |
| (Opsiyonel) OpenAI API | Zero-context review / çapraz model doğrulama | Kullanım bazlı ~$20-50/ay |
| (Opsiyonel) Gemini/DeepSeek API | Ucuz boilerplate / çoklu orkestrasyon | Çok ucuz, ~$5-20/ay |
| (Opsiyonel) Tailscale | Fleet/uzak sunucu mesh ağı | Ücretsiz (100 cihaza kadar) |
| Git + GitHub/GitLab | Versiyon ve PR | Zaten vardır |

### 3.2 Claude Code CLI Kurulumu (Resmi Yöntem)

#### Adım 1: Native Installer (ÖNERİLEN)
```bash
# macOS / Linux / WSL — tek komut
curl -fsSL https://claude.ai/install.sh | bash

# Windows PowerShell (yönetici gerekmez)
# irm https://claude.ai/install.ps1 | iex

# macOS Homebrew alternatifi (manuel güncellenir)
# brew install --cask claude-code

# Windows WinGet
# winget install Anthropic.ClaudeCode
```

- **Node.js gerekmez.** Native binary (~/.local/bin/claude) arka planda otomatik güncellenir.
- Apple'da notarized; Linux/macOS'ta imzalı.
- Kurulumdan sonra yeni bir terminal açın (PATH değişikliği için).

> **npm kullananlar için geçiş:** Önceden npm ile yüklediyseniz `claude install` komutu otomatik olarak native binary'e geçer ve ayarlarınızı (`~/.claude/`) korur. `which -a claude` ile eski npm ikilisinin PATH'te üstte olup olmadığını kontrol edin; üstteyse `npm uninstall -g @anthropic-ai/claude-code` ile kaldırın.

#### Adım 1b: npm (DEPRECATED — yeni kurulumda kullanmayın)
```bash
# Vazgeçilmezseniz Node.js 22+ gerekir
# nvm install 22 && nvm use 22
# npm install -g @anthropic-ai/claude-code
# Artık oto-güncelleme yok; elle npm update yapmanız gerekir.
```

#### Adım 2: Kurulum Doğrulama
```bash
claude --version       # v2.1.x veya üstü olmalı (güvenlik için v2.1.90+ şart)
claude doctor          # Sistem tanısı + hangi ayar dosyalarının yüklendiğini gösterir
```

#### Adım 3: İlk Yetkilendirme
```bash
claude                 # Komut satırından ilk çalıştırma
```
- Tarayıcınız otomatik açılır, Claude.ai hesabınızla giriş yaparsınız.
- `claude login` eski bir komuttur; artık otomatik browser auth akışı kullanılıyor.
- API key ile kullanmak isterseniz: `export ANTHROPIC_API_KEY="sk-ant-..."` ya da `claude --console`.

#### Adım 4: IDE Entegrasyonu
```bash
# VS Code: resmi "Claude Code" extension'ı yükleyin
# Command Palette → "Claude Code: Open" ile açın
# Terminal ile aynı oturumu paylaşır.
```
Vim/Neovim kullanıcıları terminal split'inde `claude` çalıştırmaya devam edebilir; TPope/vim-fugitive ve Telescope ile git/dosya erişimi en pratik akıştır.

### 3.3 Terminal Ortamı Yapılandırması

#### tmux Kurulumu (Vakit Kaybetmeden Kurun!)
Claude Code oturumları saatlerce sürebilir. Bağlantı koptuğunda/uykuda iş kaybetmemek için **tmux şarttır**.

```bash
# macOS
brew install tmux
# Ubuntu/Debian
sudo apt install tmux

cat > ~/.tmux.conf << 'EOF'
set -g mouse on
set -g history-limit 50000
set -g default-terminal "screen-256color"
set -g prefix C-a
bind r source-file ~/.tmux.conf \; display "Reloaded!"
bind | split-window -h
bind - split-window -v
bind h select-pane -L
bind j select-pane -D
bind k select-pane -U
bind l select-pane -R
EOF
```

**Tmux Temelleri:**
```bash
tmux new -s claude-main      # Yeni oturum
tmux attach -t claude-main   # Var olan oturuma bağlan
tmux ls                      # Oturumları listele
# İçeride: Ctrl+a sonra d → detach (çıkış yapmadan bırak)
```

#### Shell Helpers (Ağustos 2026 uyumlu)
```bash
# ~/.bashrc veya ~/.zshrc
alias c="claude"
alias cc="claude --continue"         # son oturumu devam ettir
alias cw="claude --worktree"         # hızlı worktree
alias cb="claude --bg"               # arka plan ajanı
alias ca="claude agents"             # çalışan ajanları listele
alias cu="claude update"             # native güncelleme (npm alias'ı değildi)
alias ct='tmux new-session -A -s claude'
```

### 3.4 İlk CLAUDE.md Dosyanız

Claude Code'un kalbi **CLAUDE.md** dosyasıdır. Claude her oturum başında bunu otomatik okur. `~/.claude/CLAUDE.md` (global, tüm projeler) ve proje kökündeki `CLAUDE.md` (projeye özel) birleşir; proje dosyası üstün gelir.

> **Önemli:** CLAUDE.md her request'in prefix'ine girdiği için **kısa ve kesin** tutun. Uzun prosedürleri `skills/`, otomasyonu `hooks/`, uzman delegasyonu `agents/` içine taşıyın (sonraki bölümlerde).

**Proje kökü için minimum `CLAUDE.md`:**
```markdown
# Proje Adı: [Proje İsmi]

## Kim Bu Proje?
- **Tech Stack:** Next.js 15, TypeScript, PostgreSQL (Prisma), Redis
- **Mimari:** Monorepo (pnpm), apps/web, apps/api, packages/shared
- **Dil:** Türkçe yorumlar, İngilizce kod ve commit

## Kod Standartları
- TypeScript strict mode açık
- React Server Component öncelikli; client component "use client"
- Test: Vitest + RTL, yeni kod için zorunlu
- Tailwind CSS kullan, inline CSS yok
- Lint/Format: ESLint + Prettier, commit öncesi otomatik

## Git Workflow
- Branch: feat/, fix/, refactor/, chore/
- Conventional Commits
- PR açmadan önce pnpm test && pnpm lint

## Çalışma Prensipleri
- TDD tercih et
- Değişikliklerin etki alanını güncelle
- Emin olmadığın şeyleri sor, varsayım yapma
- Asla secret/API key yazma; .env.example kullan
- Migration'lar geri döndürülebilir olmalı
```

**Global (`~/.claude/CLAUDE.md`) — kişisel tercihler:**
```markdown
# Global CLAUDE.md — Tüm Projelerde Geçerli

## İletişim Tarzı
- Açık, net, doğrudan. Gereksiz nezaket cümlesi yok.
- Hata varsa doğrudan söyle.
- Kod bloğundan önce 1 cümle özet, sonra dikkat noktası.
- Türkçe yanıt; teknik terimler İngilizce kalabilir.

## Mühendislik Prensipleri
- Önce çalışır kod, sonra güzel kod.
- Erken hata ver (fail fast).
- Her değişiklik için en az 1 test.
- Zero-context review uygula.
- Context 80k token'a yaklaşınca /compact öner.
- 3 başarısız düzeltme sonrası durumu özetle ve insana devret.

## Güvenlik
- .env dosyalarını ASLA okuma/yazma.
- node_modules/build çıktılarını tarama.
- Veritabanı drop/reset için onay iste.
```

### 3.5 İlk Ayarlar: `.claude/settings.json`

Proje kökünde `.claude/` klasörü oluşturun ve takım paylaşımlı temel ayarları koyun:

```json
{
  "model": "claude-sonnet-5",
  "permissions": {
    "allow": ["Read", "Edit", "Write", "Grep", "Glob", "Bash(pnpm test:*)", "Bash(pnpm lint)"],
    "deny": ["Bash(rm -rf *)", "Bash(git push --force *)", "Bash(*DROP TABLE*)", "Bash(curl *| sh)"]
  },
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

`settings.local.json` (gitignore'a alın) içine API key'ler gibi kişisel değerleri koyun.

### 3.6 İlk `claude doctor` Çalıştırması
```bash
mkdir -p my-project && cd my-project
git init
claude doctor
# "No project CLAUDE.md found" uyarısı alacaksınız → /init ile sihirbazı çalıştırın:
claude
#  /init       # CLAUDE.md oluşturma sihirbazı
```

> **L0 Kontrol Noktası:** `claude --version` ve `claude doctor` temiz çıkıyor; CLAUDE.md oluşturuldu; tmux'ta oturum açılabiliyor → L1'e hazırsınız.

---
