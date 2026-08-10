## 10. Paralel Agent Orkestrasyonu ve Worktree Stratejisi

### 10.1 Neden Paralelleştirme Gerekli?

Büyük bir feature'ı tek bir oturumda tek ajanla yapmak:
- Context penceresini hızla doldurur
- Birçok farklı dosya arasında dikkat dağınıklığına yol açar
- Hata zinciri riski artar
- Geri dönüşü zorlaşır

**Bunun yerine: görevi bağımsız parçalara ayırın ve her parçayı ayrı ajan/worktree'de paralel yapın.**

### 10.2 Bağımsız Alan Analizi

Paralelleştirmeden önce **bağımlılık grafiğini** çıkarmak şarttır.

**İyi paralelleşen görevler (bağımsız):**
- Frontend component'leri (her component ayrı dosya)
- Backend route/controller'lar
- Test yazma (birden fazla test dosyası)
- Dokümantasyon
- Tip tanımları / DTO'lar
- Farklı utility fonksiyonları

**Paralelleşmemesi gerekenler:**
- Veritabanı migration'ları (sıralı olmalı)
- Birbiriyle konuşan modüllerin ikisi aynı anda
- Auth ve onu kullanan korumalı route'lar (önce auth, sonra route)
- Shared konfigürasyon dosyaları
- Package.json (merge conflict garantidir)

### 10.3 Git Worktree Kurulumu

Git worktree, aynı repo'nun birden fazla kopyasını farklı dizinlerde açmanızı sağlar. Her ajana ayrı worktree = çakışma yok. Bu deseni 2026'da neredeyse tüm ajan orkestratörleri (GitHub Copilot App, Claude Squad, Composio AO) varsayılan olarak kullanır.

```bash
# Ana repo (her zaman main branch'te temiz kalsın)
cd ~/projects/myapp

# Yeni bir worktree ve branch oluştur
git worktree add ../myapp-auth -b feat/auth
git worktree add ../myapp-payment -b feat/payment
git worktree add ../myapp-docs -b docs/readme

# Worktree'leri listele
git worktree list
```

**Yerleşik Kısayol (v2.1+):**
Claude Code'un kendi `--worktree` bayrağı otomatik olarak worktree oluşturup o dizinde başlatır:
```bash
claude --worktree feat/auth     # Otomatik worktree + branch
claude --bg --worktree feat/payment "Stripe entegrasyonu yap"  # Arka planda
```

#### Runtime İzolasyonu (Önemli!)
Worktree dosya sistemi izolasyonu sağlar ama aşağıdakiler paylaşılır:
1. **Portlar:** İki ajan aynı anda `localhost:3000`'i açmaya çalışırsa çakışır.
2. **Veritabanı:** Aynı dev veritabanını kullanırlarsa test verileri birbirine girer.
3. **`.env` dosyası:** Ana dizindeki `.env` sembolik link veya paylaşımdan gelebilir.
4. **node_modules:** pnpm/npm ile genelde sembolik link ile paylaşılır (iyi, sorun değil).

Her worktree için ayrı `.env.local` (gitignored) ile port/DB ayırın:
```bash
# ../myapp-auth/.env.local
PORT=3001
DATABASE_URL="postgresql://user:pass@localhost:5432/myapp_auth"

# ../myapp-payment/.env.local
PORT=3002
DATABASE_URL="postgresql://user:pass@localhost:5432/myapp_payment"
```

Güvenli tarafta kalmak için paralel ajanları çalıştırmadan önce dev sunucuyu durdurup ajanların testleri Node.js'in internal test runner'ıyla (port açmadan) çalıştırmasını sağlayın.

**Worktree temizleme:**
```bash
git worktree remove ../myapp-auth
git worktree prune  # Merge sonrası temizlik
```

### 10.4 Sentezleyici (Synthesizer) Ajan

Paralel biten işleri birleştirmek için **ana branch'te çalışan bir sentezleyici ajan** gerekir. Bu ajan, tüm değişiklikleri okur, çelişkileri çözer ve birleştirir.

**Sentezleyici Görev Tanımı:**
```
Şu branch'lerde paralel yapılan işleri inceleyip main branch ile birleştir:
- feat/auth: Kimlik doğrulama sistemi
- feat/payment: Stripe entegrasyonu
- feat/user-profile: Kullanıcı profil sayfası

Görevlerin:
1. Her branch'teki değişiklikleri oku ve özetle
2. Çakışan veya çelişen değişiklikleri tespit et (örneğin iki branch aynı dosyayı değiştirmiş)
3. Çelişkileri çöz (gerekirse insan onayı için not et)
4. Tüm branch'leri sırayla main'e merge et (rebase et)
5. Merge sonrası tüm testlerin geçtiğini doğrula
6. Entegrasyon hatalarını düzelt
7. Nihai durumu bir CHANGELOG girişine yaz
```

### 10.5 Paralel İş Başlatma Yardımcı Scripti

**`scripts/parallel-spawn.sh`:**
```bash
#!/bin/bash
set -e

TASK_FILE=$1
if [ -z "$TASK_FILE" ] || [ ! -f "$TASK_FILE" ]; then
  echo "Kullanım: $0 tasks.json"
  echo "tasks.json formatı:"
  cat << 'EOF'
[
  {"name": "auth", "branch": "feat/auth", "task": "JWT auth sistemi kur"},
  {"name": "payment", "branch": "feat/payment", "task": "Stripe ödeme entegrasyonu"}
]
EOF
  exit 1
fi

# Ana dizinde olmalıyız
if [ ! -d ".git" ]; then
  echo "Bu script bir git reposunun kök dizininden çalıştırılmalı."
  exit 1
fi

BASEDIR=$(pwd)
mkdir -p .claude/parallel-logs

cat "$TASK_FILE" | jq -c '.[]' | while read -r task; do
  NAME=$(echo "$task" | jq -r '.name')
  BRANCH=$(echo "$task" | jq -r '.branch')
  TASK_DESC=$(echo "$task" | jq -r '.task')

  WT_PATH="../parallel-$(basename "$BASEDIR")-$NAME"

  # Worktree oluştur
  if [ ! -d "$WT_PATH" ]; then
    git worktree add -b "$BRANCH" "$WT_PATH" origin/main
  fi

  cd "$WT_PATH"

  # Görev dosyasını hazırla
  cat > AGENT_TASK.md << MD
# Paralel Görev: $NAME
**Branch:** $BRANCH

## Görev
$TASK_DESC

## Kısıtlar
- Bu worktree içinde çalış, ana dizine dokunma
- Sadece kendi görevinle ilgili dosyaları değiştir
- package.json veya paylaşılan config dosyalarını DEĞİŞTİRME, gerekirse not et
- Tamamladığında BENI_BITIRDIM.md dosyası oluştur ve değiştirdiğin dosyaları listele

## Kabul Kriterleri
- Kendi testlerin geçiyor olmalı
- Typecheck ve lint temiz
- Diğer branch'lere dokunma
MD

  # Tmux oturumunda ajanı başlat
  if ! tmux has-session -t "agent-$NAME" 2>/dev/null; then
    tmux new-session -d -s "agent-$NAME" \
      "cd '$WT_PATH' && claude --print 'AGENT_TASK.md dosyasını oku ve görevi tamamla' > '$BASEDIR/.claude/parallel-logs/$NAME.log' 2>&1"
    echo "✅ Ajan başlatıldı: $NAME (branch: $BRANCH, oturum: agent-$NAME)"
  else
    echo "ℹ️  Ajan zaten çalışıyor: $NAME"
  fi

  cd "$BASEDIR"
done

echo ""
echo "Tüm ajanlar başlatıldı. Durum takibi için:"
echo "  tmux ls  — oturumları listele"
echo "  tail -f .claude/parallel-logs/*.log  — logları izle"
echo ""
echo "Hepsi bittiğinde sentezleme için:"
echo "  cd $BASEDIR"
echo "  # Sentezleyici ajanı çalıştır"
```

### 10.6 Mevcut Orkestratör Araçları (2026)

Kendi script'lerinizi yazmak yerine olgun araçları kullanmak zaman kazandırır:

| Araç | Tip | Özellikleri | Uygun Olduğu |
|:---|:---|:---|:---|
| **Claude Code Agent Teams** (yerleşik) | Resmi özellik | P2P haberleşme, otomatik iş bölümü, worktree izolasyonu | Claude Code ile herkes |
| **Claude Squad** (`cs`) | Açık kaynak TUI | tmux + worktree, tek tuşla paralel oturum, birden fazla ajan desteği (Codex, Gemini) | Solo geliştiriciler |
| **GitHub Copilot App** | Resmi masaüstü | Her ajan otomatik worktree'de, görsel kontrol paneli | Copilot ekosistemi kullananlar |
| **Composio AO** | Web dashboard | Milestone gate'ler, otomatik CI retry, çoklu ajan | Takımlar |
| **Baton** | CLI | GitHub Issue odaklı poll-dispatch-reconcile | Issue-tabanlı iş akışı |
| **Vibe Kanban** | Web UI | Kanban panosu üzerinden 10+ ajan tipi | Görsel yönetim tercih edenler |
| **Emdash** | Electron desktop | Paralel dispatch, insan denetimi, 22+ CLI provider | Çoklu araç kullananlar |

**Başlangıç için tavsiye:** Claude Squad'ı kurun. Kurulumu 1 dakika sürer ve işletim seviyesinde karmaşa yaratmaz:
```bash
# macOS
brew install claude-squad
# Veya script
curl -fsSL https://raw.githubusercontent.com/smtg-ai/claude-squad/main/install.sh | bash

cs  # TUI'yu başlat
```

### 10.7 Senkronizasyon Kontrol Noktaları

Paralel çalışan ajanlar arasında belirli aralıklarla senkronizasyon yapmak çakışma riskini azaltır:

**İyi uygulama:**
- Her 1-2 saatte bir ana branch'i ajanların worktree'lerine fetch/rebase edin
- Paylaşılan tipler veya arayüzler değiştiğinde ilgili diğer ajanları bilgilendirin (ancak paylaşımlı dosyaları değiştirmekten kaçının; ortak bir sözleşme/interface ajanıyla halledin)
- Gün sonunda tüm worktree'leri commit'leyin ve sentezleyici ajan ile birleştirin
- Entegre edilemeyen büyük değişiklikleri acele etmeyin, bir sonraki güne bırakın
- Çakışma çıktığında ajana çözdürmeyin; kendiniz çözün veya sentezleyiciye açık bir talimat olarak verin

---
