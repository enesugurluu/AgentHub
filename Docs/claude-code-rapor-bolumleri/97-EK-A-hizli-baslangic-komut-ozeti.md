## EK A: Hızlı Başlangıç Komut Özeti

### Kurulum (Ağustos 2026 — Native Installer Önerilir)

```bash
# macOS / Linux / WSL2 (ÖNERİLEN — Node.js gerektirmez, otomatik güncellenir)
curl -fsSL https://claude.ai/install.sh | bash

# Homebrew (macOS/Linux)
brew install --cask claude-code

# Windows PowerShell
irm https://claude.ai/install.ps1 | iex

# Windows WinGet
winget install Anthropic.ClaudeCode

# Kurulumu doğrula
claude --version
claude doctor

# Eski npm kurulumundan geçiş (npm ile kurduysanız)
claude install   # Native binary'ye geçiş yapar
```

### CLI Bayrakları

```bash
claude                           # İnteraktif oturum (varsayılan)
claude "görev açıklaması"        # Tek seferlik görev
claude -p "sorgu"                # Non-interactive (script/pipe için)
claude -c                        # Son konuşmaya devam et
claude -r                        # Önceki oturumlardan seç
claude --worktree <branch>       # Yeni bir git worktree'de başlat
claude --bg "uzun görev"         # Arka planda ajan başlat
claude agents                    # Tüm çalışan/tamamlanan ajanları listele
claude --model sonnet            # Belirli modelle başla
claude --permission-mode plan    # Plan modu (sadece öneri, değişiklik yok)
claude -p --max-turns 8          # Maksimum 8 tur
claude -p --max-budget-usd 3     # Maksimum $3 harcama
claude --effort high             # Zeka seviyesi (low/medium/high/xhigh/max)
claude mcp list                  # MCP sunucularını listele
claude project purge             # Proje state'ini temizle
claude update                    # Claude Code'u güncelle
```

### Oturum İçi Slash Komutlar

| Komut | İşlev |
|:---|:---|
| `/help` | Tüm komutları göster |
| `/clear` | Konuşmayı sıfırla, temiz oturum |
| `/compact` | Context'i özetle ve devam et |
| `/usage` | Token ve maliyet raporu |
| `/model` | Model değiştir (opus/sonnet/haiku/fable) |
| `/effort` | Zeka seviyesi ayarla |
| `/mcp` | MCP durumunu kontrol |
| `/skills` | Mevcut becerileri göster/yönet |
| `/resume` | Önceki oturumdan devam et |
| `/recap` | Oturum özeti çıkar |
| `/loop 10m` | Her 10 dakikada tekrarla |
| `/login` | Hesap değiştir/doğrula |
| `/doctor` | Sistem sağlık kontrolü |
| `/init` | Proje için otomatik CLAUDE.md önerisi |

### Kısayollar ve Önekler

Giriş kutusundayken:

| Önek | İşlev | Örnek |
|:---|:---|:---|
| `#` | Notu kalıcı hafızaya ekle | `# TypeScript strict mode kullan` |
| `/` | Slash komut çalıştır | `/test` |
| `!` | Doğrudan bash komutu çalıştır | `!git status` |
| `@` | Dosya/agent referansı | `@src/auth.ts`, `@code-reviewer` |
| `&` | Bulut görevi olarak gönder | `& Bügünkü hataları tara` |

### Git Worktree

```bash
# Manuel worktree oluşturma
git worktree add ../proje-ozellik -b feat/ozellik

# Worktree'leri listele
git worktree list

# Worktree kaldır
git worktree remove ../proje-ozellik

# Otomatik worktree ile Claude başlat (yerleşik)
claude --worktree feat/ozellik
```

### tmux (Kalıcı Oturumlar)

```bash
tmux new -s <isim>        # Yeni oturum
tmux attach -t <isim>     # Oturuma bağlan
tmux ls                   # Oturumları listele
tmux kill-session -t <isim> # Oturumu kapat
# İçeride: Ctrl+a sonra d → arkada bırak (detach)
```

### Proje Başlatma (10 Dakikalık Minimum Kurulum)

```bash
# 1. Proje klasörüne gir
cd my-project

# 2. Claude'u ilk kez çalıştır (kimlik doğrulamayı yap)
claude

# 3. Claude içinde otomatik başlangıç
/init
# Birkaç soruya cevap ver, CLAUDE.md iskeleti oluştursun

# 4. Temel güvenlik izinleri (.claude/settings.json)
# (raporun 12. bölümündeki örneği kullan)

# 5. .claudeignore dosyası oluştur
cat > .claudeignore << 'EOF'
.env
.env.*
!.env.example
node_modules/
.next/
dist/
build/
*.log
*.sqlite
EOF

# 6. İlk işi ver
"Projeyi oku ve genel bir mimari özet ver, nerede ne var anlat."
```

### Güvenlik Kontrolü

```bash
# Sürüm (güvenlik açığı için ≥2.1.90 olmalı)
claude --version

# İzinleri görüntüle
claude config list

# Destructive Command Guard önerilen ek güvenlik:
# https://github.com/Dicklesworthstone/destructive_command_guard
```

### Acil Durum Komutları

```bash
# Tüm ajanları durdur
claude agents | grep -E '^\s+[0-9a-f]' | awk '{print $1}' | xargs -I {} claude agents stop {}

# Çalışma dizinini son commite geri al (DİKKAT: değişiklikleri siler)
git checkout .

# Son commit'i geri al (değişiklikleri korur)
git reset HEAD~1

# Tüm tmux oturumlarını öldür
tmux kill-server
```
