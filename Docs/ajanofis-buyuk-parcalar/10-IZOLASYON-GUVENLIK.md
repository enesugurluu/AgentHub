## 10. İzolasyon ve Güvenlik

### 10.1 Her Ajan İçin Ayrı Git Worktree
Her ajan için kendi iş dizini, branch'i ve worktree'si otomatik oluşturulur:
```
<project>/.agentcompany/worktrees/
├── ceo/                 # CEO worktree (ana branch)
├── cto/
├── backend-1/
├── frontend-2/
└── qa-1/
```

Worktree oluşturma iş akışı:
1. Görev atanınca Rust `git worktree add` çağrısı yapar
2. `.env.local` dosyası otomatik oluşturulur (port çakışmasını önlemek için her ajan için farklı PORT atanır)
3. node_modules sembolik link ile paylaşılır (disk tasarrufu)
4. Ajan process'i o dizinde spawn edilir
5. Görev bitiminde/ajan işten çıkartıldığında worktree ya silinir ya inceleme için tutulur

### 10.2 Süreç İzolasyonu
- Her ajan kendi OS sürecinde çalışır
- PTY (portable-pty) üzerinden izole edilir; stdin/stdout doğrudan child process'e gider, webview hiçbir zaman kabuk erişimi almaz
- **Üç kademeli sandbox seçeneği:**
  1. **Basit (varsayılan):** process-group izolasyonu, worktree ile dosya sistemi sınırı, izin listesi tabanlı komut filtreleme
  2. **Orta (bwrap):** Linux'ta `bubblewrap` ile dosya sistemi/network namespace'i (macOS'ta `sandbox-exec` benzeri); Claude Code topluluğunda benimsenen `claude-hardening` deseni
  3. **Güçlü (Docker/K8s):** Ajan bir konteyner içinde çalışır; beyaz listeyle izin verilen dizinler bağlanır, ağ erişimi kapatılabilir (opsiyonel)
- Ana proje dizini hiçbir ajan tarafından doğrudan değiştirilemez, sadece kendi worktree'sinde değişiklik yapar.
- Tüm ajan çıktıları logda maskelenir: API key, token, `.env` benzeri desenler regex ile sansürlenir (15.3).

### 10.3 Runtime İzolasyonu (Port/DB Çakışma)
Port numaraları atanırken ajan ID'sine göre offset kullanılır:
```
PORT = 3000 + (agent_id * 10)
REDIS_DB = agent_id
TEST_DB = `test_${agent_id}`
```
.env.local otomatik oluşturulur.

### 10.4 Ana Branch Koruma
Hiçbir ajan direkt ana branch'e push/commit atamaz. Tüm değişiklikler kendi branch'inde kalır. Merge işlemi:
1. QA incelemesinden geçer
2. CEO tarafından onaylanır
3. İnsana sunulur (opsiyonel otomatik merge sadece onayla)

---

