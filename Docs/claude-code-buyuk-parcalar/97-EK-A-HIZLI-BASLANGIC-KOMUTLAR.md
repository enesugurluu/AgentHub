## EK A: Hızlı Başlangıç Komut Özeti

> **Son güncelleme:** 2026-08-09 — Claude Code v2.1.x içindir. npm kurulumu v2.1.15+ itibarıyla deprecated; native installer kullanın.

### A.1 Kurulum (3 platform)

```bash
# macOS / Linux / WSL (önerilen — native, Node gerektirmez, oto-güncellemeli)
curl -fsSL https://claude.ai/install.sh | bash

# Windows PowerShell (yönetici değil)
irm https://claude.ai/install.ps1 | iex

# Windows WinGet (manuel güncelleme)
winget install Anthropic.ClaudeCode

# macOS Homebrew (manuel güncelleme)
brew install --cask claude-code

# npm (DEPRECATED — sadece eski Node 22+ ile çalışır, yeni kurulumda kullanmayın)
# npm install -g @anthropic-ai/claude-code

# npm'den native'e geçiş (ghost npm ikili dosyasını temizler)
claude install

# Kurulum doğrulama
claude --version
claude doctor           # sistem sağlık kontrolü + hangi ayarların yüklendiğini gösterir
```

### A.2 Yetkilendirme

```bash
claude                  # ilk açılışta tarayıcı otomatik açılır, hesapla giriş yap
# API key ile kullanmak isterseniz:
export ANTHROPIC_API_KEY="sk-ant-..."
claude --console        # Console (API) hesabıyla giriş
```

### A.3 CLI Flags

| Flag | Açıklama |
|:---|:---|
| `claude` | Etkileşimli mod |
| `claude "soru"` | Tek seferlik görev çalıştır, shell'e dön |
| `claude -p "soru"` | Print/pipe modu: cevabı stdout'a bas, çık (scriptler için) |
| `claude -c / --continue` | Bu dizindeki en son oturumu devam ettir |
| `claude -r / --resume` | Oturum seçici aç |
| `claude -w <branch>` / `--worktree <branch>` | Yeni git worktree + branch'te oturum aç |
| `claude --bg "görev"` | Arka plan ajanı başlat |
| `claude agents` | Çalışan/bloke/tamamlanmış oturumları listele |
| `claude --effort <low\|medium\|high\|xhigh\|max>` | Zeka seviyesi (overthinking'i önler) |
| `claude --max-turns N` | Maksimum tur limiti |
| `claude --max-budget-usd X` | Harcama limiti (script modunda) |
| `claude --permission-mode plan` | Plan modu: sadece oku ve öner, değişiklik yok |
| `claude --dangerously-skip-permissions` | Onayları kapat (yalnız güvenli ortamlarda) |
| `claude -p --output-format json` | JSON çıktı (script entegrasyonu) |
| `claude mcp list` | MCP sunucularını listele |
| `claude mcp add --transport http <name> <url>` | Uzak MCP ekle |
| `claude project purge` | Projeye ait tüm Claude state'ini sil |
| `claude update` | Native kurulumu elle güncelle |
| `claude uninstall` | Kaldır |
| `claude --safe-mode` | Özelleştirmeleri devre dışı bırak (sorun giderme) |

### A.4 Oturum İçi Slash Komutları

| Komut | Açıklama |
|:---|:---|
| `/help` | Tüm komutlar |
| `/clear` | Konuşmayı sıfırla (yeni oturum) |
| `/compact` | Context'i özetle (token tasarrufu) |
| `/resume` | Oturum seçici |
| `/model` | Model değiştir |
| `/effort` | Zeka seviyesi: low/medium/high/xhigh/max |
| `/mcp` | MCP sunucu durumu / auth |
| `/skills` | Mevcut becerileri listele/yönet |
| `/usage` | Bu oturumun token ve maliyet özeti (eski `/cost`) |
| `/login` | Hesap değiştir |
| `/loop` | Bir prompt'u periyodik tekrarla |
| `/recap` | Context özeti (uzak dönüşte) |
| `/schedule` | Bulut zamanlanmış görev yönetimi |
| `/tasks` | Görev listesi (Agent Teams için) |
| `/status` | Anlık durum |
| `/init` | CLAUDE.md oluşturma sihirbazı |
| `/config` | Ayarları interaktif düzenle |
| `Shift+Tab` | Plan modunu aç/kapat |
| `Ctrl+D` / `exit` | Çık |

### A.5 Cron İşleri ve Arka Plan

```bash
# Arka plan ajanı (terminal kapansa da devam eder)
claude --bg "Tüm testleri çalıştır, kırılanları düzelt, sonucu özetle"

# Çalışan ajanları gör
claude agents

# Belirli bir ajan oturumunu attach et
claude -r            # son çalışana devam
# veya oturum ID'si ile

# Bulutta zamanlanmış görev (Scheduled Tasks)
claude --schedule "0 9 * * 1-5" "Günlük dependency audit raporu hazırla"
# Veya oturum içinde /schedule komutu
```

### A.6 Git Worktree ile Paralel Görev

```bash
# Her paralel görev için ayrı worktree + branch
claude --worktree feat/auth        # .claude/worktrees/ altında yeni worktree
claude --worktree fix/login-bug -n bugfix

# Subagent tanımında worktree izolasyonu (.claude/agents/reviewer.md):
#   isolation: worktree
#   background: true
```

### A.7 Teşhis ve Sorun Giderme

```bash
claude doctor                       # sistem tanısı
claude --debug                      # hangi ayar dosyalarının yüklendiğini loglar
which -a claude                     # PATH'te birden fazla claude (npm ghost) mu var?
rm -rf ~/.local/bin/claude && curl -fsSL https://claude.ai/install.sh | bash   # temiz yeniden kurulum
```

---
