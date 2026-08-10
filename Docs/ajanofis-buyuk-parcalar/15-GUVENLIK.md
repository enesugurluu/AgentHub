## 15. Güvenlik

AjanŞirket kullanıcı makinesinde çalışan ve harici CLI'ları (her biri kendi başına kod yürütebilen) orkestra eden bir uygulama olduğu için güvenlik en yüksek önceliğe sahiptir. Ajanların kullanıcı adına dosya silmesi, veritabanı komutu çalıştırması, dış ağa çıkması riskler doğurur. Savunma derinliği (defense in depth) ilkesiyle birden fazla katman uygulanır.

### 15.1 İzin Modeli (Tauri Capability-based)

Tauri 2'nin capability tabanlı izin modeli temel alınır:
- Frontend (webview) hiçbir zaman doğrudan işletim sistemine erişmez; sadece tanımlı Tauri invoke komutlarını çağırabilir
- Her komut için yetki denetimi yapılır
- Varsayılan prensip: **enyet (hepsi kapalı)**; ancak açıkça izin verilen kabiliyetler çalışır

### 15.2 Komut Politikası Motoru (Policy Engine)

Bölüm 7.4'de bahsedilen onay akışı bir politika motoruyla desteklenir:

- İzinler dört seviyeli: `allow`, `allow-always`, `ask`, `deny`
- Varsayılan politika:
  - `allow`: dosya okuma (worktree içi), build komutları, test çalıştırma, git status/diff/log
  - `ask`: dosya yazma, paket kurma, migration, dış ağ erişimi, ana branş işlemleri
  - `deny`: `rm -rf /`, `sudo`, `curl|sh`, `DROP TABLE`, `git push --force`, `.env` dosyalarına yazma, `mkfs`, `dd if=... of=/dev/`, bilinmeyen MCP sunucu ekleme
- Kullanıcı UI'dan politika kuralları ekleyip kaldırabilir; kalıcı kurallar `~/.agentcompany/policy.toml` içinde saklanır
- Komut çalıştırılmadan önce Rust tarafında regex/lexer tabanlı denetim; basit regex değil shell ayrıştırma (bashlex-benzeri)

### 15.3 İzin Verilenler/Dışlananlar (Allow/Deny List)

| Kategori | Allow | Ask | Deny |
|:---|:---|:---|:---|
| Dosya erişimi | worktree içi okuma, izinli path yazma | worktree dışı yazma, ana proje dizini yazma | `~/.ssh/`, `~/.aws/`, `.env*`, `*.pem`, `id_rsa*` |
| Ağ | localhost, MCP whitelist | dış internet (curl/wget/fetch) | açık proxy, bilinmeyen hostlara raw bağlantı |
| Git | status, diff, log, branch, worktree | commit, push, merge, checkout | push --force, reset --hard (onaysız) |
| Paket yöneticisi | — | npm/pip install, cargo add | kaldırma/force flag'leri |
| Veritabanı | SELECT (RO bağlantı) | INSERT/UPDATE/MIGRATE | DROP, TRUNCATE, DELETE (onaysız) |
| Sistem | echo/cat/ls/git/derleme/test |  | sudo, chmod -R 777, mkfs, dd, fork bomb |

### 15.4 Secret/API Anahtar Koruması

- Hiçbir API anahtarı düz metin dosyada tutulmaz; `keyring` crate ile OS keychain'de (Windows Credential Manager, macOS Keychain, Linux Secret Service) saklanır
- Ajanlara çevre değişkeni olarak geçirilirken **maskelenir**; log/terminal çıktısında regex ile `sk-ant-[A-Za-z0-9_-]{20,}` gibi desenler sansürlenir
- `.env` ve benzeri gizli dosyalar worktree'lere otomatik kopyalanmaz
- Kullanıcı isterse worktree başına environment injection yapabilir; ancak bu da açık onayla ve maskeli loglamayla olur

### 15.5 İzolasyon (Bölüm 10'u Tamamlar)

- Basit mod: process group, çalışma dizini, PATH kısıtı
- Orta mod (Linux için önerilen): bubblewrap ile mount namespace'i worktree ile sınırla, /home salt okunur, ağ opsiyonel olarak kapat
- Güçlü mod: Docker konteyneri; volume mount worktree ile sınırlı, user namespace, ağ opsiyonel
- Claude Code v2.1.90+ sürüm zorunluluğu; eski sürümdeki 50-subcommand deny-rule bypass ve SOCKS5 sandbox bypass açıklarından (bkz. Claude raporu Bölüm 12) korunmak için ajan CLI'larının sürüm denetimi yapılır. Bilinen güvenlik açığı olan sürümler çalıştırılmaz.

### 15.6 Denetim Kaydı (Audit Log)

Tüm ajan etkinliği değiştirilemez (append-only) JSONL olarak saklanır:
- Spawn edilen süreç (PID, komut, çalışma dizini, ajan ID)
- Çalıştırılan her shell komutu (onay dahil)
- Silinen/oluşturulan/değiştirilen dosyalar
- STDOUT/STDERR çıktısı
- Maliyet/token kullanımı
- Kullanıcı onayları ve cevapları
- Süreç bitiş kodu

Bu log ile herhangi bir zamanda "ne oldu, hangi ajan hangi dosyayı değiştirdi" sorusu cevaplanır. "Time Machine" UI ile ajan oturumları geriye sarılıp izlenebilir.

### 15.7 CSP ve Web Güvenliği

Tauri webview üzerinde Content Security Policy sıkı tutulur:
- Script kaynağı sadece kendi paketinden
- İnline script çalışmaz (hash/nounce ile kontrollü)
- `unsafe-eval` kapalı
- Dış kaynak yükleri varsayılan kapalı
- Tauri IPC komutları için origin doğrulaması

### 15.8 Auto-updater Güvenliği

- Tauri built-in auto-updater kullanılır; güncellemeler kod-imzalı (ED25519)
- İmza doğrulaması yapmadan güncelleme yüklenmez
- Güncelleme sunucusu HTTPS; sertifika sabitleme (certificate pinning) opsiyonel

### 15.9 Acil Durum (Kill Switch)

- UI'da belirgin "Tüm ajanları durdur" düğmesi
- Kısayol: `Ctrl/Cmd+Shift+X` tüm child process'leri SIGKILL ile sonlandırır
- Uygulama kapanırken de tüm PTY süreçleri temizlenir (process group kill)
- Kill switch sonrası audit log'da açıkça işlenir; kullanıcıya "N ajan durduruldu, hangi worktree'ler temizlendi?" raporu verilir

### 15.10 Hata ve Bug Bounty Zihniyeti

- İlk sürümden itibaren `security@` adresine açık raporlama kanalı
- Güvenlik açıkları için sorumlu ifşa politikası
- Kritik güncellemeler için kullanıcıya zorunlu yükseltme uyarısı (riskliyse işlem durdurulabilir)

---
