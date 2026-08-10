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

Git worktree aynı repo'nun birden fazla kopyasını farklı dizinlerde açar. Her ajana ayrı worktree = dosya çakışması yok.

```bash
# Ana repo (her zaman main branch'te temiz kalsın)
cd ~/projects/myapp

# YÖNTEM 1: Elle git worktree
git worktree add ../myapp-auth -b feat/auth
git worktree add ../myapp-payment -b feat/payment
git worktree add ../myapp-docs -b docs/readme

# YÖNTEM 2: Claude Code native --worktree flag (önerilen)
# Otomatik .claude/worktrees/ altında worktree ve branch oluşturur
claude --worktree feat/auth      # main'den yeni worktree açar
claude -w feat/payment -n pay    # isimlendirilmiş worktree
claude agents                    # çalışan worktree ajanlarını izle

# Worktree'leri listele
git worktree list

# Her worktree'de ayrı terminal/ajan:
# Terminal 1:
cd ~/myapp-auth && claude
# (Auth üzerinde çalışacak ajan)

# Terminal 2:
cd ~/myapp-payment && claude
# (Payment üzerinde çalışacak ajan)

# Terminal 3:
cd ~/myapp-docs && claude
# (Dokümantasyon yazacak ajan)
```

**İş bittiğinde worktree temizleme:**
```bash
# Bir worktree'yi kaldır
git worktree remove ../myapp-auth

# Veya merge sonrası otomatik temizlik
git worktree prune
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

### 10.6 Runtime İzolasyonu (Worktree Yeterli Değil!)

Git worktree sadece **dosya sistemini** ayırır. Şunlar paylaşılır:

- localhost portları (iki ajan aynı 3000 portuna `pnpm dev` kalkışırsa çakışır)
- `.env.local` ve global environment variable'lar
- Ortak Docker daemon / yerel postgres-redis instance
- `~/.npm`, `~/.pnpm-store` gibi global önbellekler
- Tarayıcı oturumları ve auth cookie'leri

**Çözüm:** Her ajan için:
- Port atama tablosu tutun (3001, 3002, …) veya `PORT=0` (rastgele) kullandırın
- `.env.local` yerine worktree'ye özel `.env.<branch>` dosyaları
- Gerekirse Docker network'ü / ayrı container kullanın
- Hassas credential'ları ortak process'lerde bırakmayın

`.claude/agents/*.md` içinde `isolation: worktree` frontmatter alanı subagentlar için otomatik worktree yaratır.

### 10.7 Mevcut Orkestrasyon Araçları (2026 Manzara)

Sıfırdan script yazmak yerine olgun araçları değerlendirin:

| Araç | Tip | Öne çıkan özellik |
|:---|:---|:---|
| **Claude Squad** (`cs`) | TUI (Homebrew/script) | tmux + worktree yönetir, Claude/Codex/Aider/Gemini destekli, hafif, terminal severler için en iyi başlangıç |
| **Vibe Kanban** | Web UI | Kanban board, her kart kendi worktree + branch, çoklu ajan desteği, diff inceleme |
| **Composio AO (Agent Orchestrator)** | Web dashboard + CLI | GitHub Issues → ajan → PR pipeline; CI kırılırsa ajan düzeltir, multi-model |
| **Conductor (Melty Labs)** | macOS native app | Görsel dashboard, Claude Code + Codex, diff-first review, Mac için en cilalı |
| **Parallel Code** | Electron/desktop | 2-4 canlı ajanı aktif yönlendirme |
| **Bernstein** | CLI + web | Deterministik scheduler, pre-merge verification |
| **Emdash** | Electron | ~22 CLI provider, paralel dispatch, port yönetir |
| **Baton** | CLI/Desktop | GitHub Issues tabanlı poll-dispatch |
| **Nimbalyst (Crystal'ın devamı)** | Desktop | Kod + döküman/diyagram/mockup/CSV karışık işler |
| **Agent Teams (yerleşik)** | Claude Code native | Lead + teammate'ler P2P, env var ile açılır |
| **OMC (oh-my-claudecode)** | Plugin | 19+ ajan, 40+ skill, Claude+Codex+Gemini routing |

**Karar çerçevesi:**
- Terminal/CLI ağırlıklı, sade → **Claude Squad**
- Görsel board, kod dışı artifact da var → **Vibe Kanban** veya **Nimbalyst**
- GitHub Issues güdümlü otonom pipeline → **Composio AO** / **Baton**
- Mac'te güzel UI + hızlı multi-ajan → **Conductor**
- Sadece Claude'un kendi özelliği ile → **Agent Teams** (basit ama 3-5 teammate ile sınırlı)

### 10.8 Senkronizasyon Kontrol Noktaları

Paralel çalışan ajanlar arasında belirli aralıklarla senkronizasyon yapmak çakışma riskini azaltır:

**İyi uygulama:**
- Her 1-2 saatte bir ana branch'i işleyen ajanların worktree'sine rebase/fetch ettirin
- Paylaşılan tipler veya arayüzler değiştiğinde, ilgili diğer ajanları bilgilendirin
- Gün sonunda tüm worktree'leri commit'leyin ve sentezleyici ile birleştirin
- Entegre edilemeyen büyük değişiklikleri bir sonraki güne bırakın, acele etmeyin

---

