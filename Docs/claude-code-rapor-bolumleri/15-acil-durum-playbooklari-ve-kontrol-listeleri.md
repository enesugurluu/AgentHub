## 15. Acil Durum Playbook'ları ve Kontrol Listeleri

### Playbook 1: Ajan Döngüye Girdi (Debeleniyor)

**Belirtiler:**
- Aynı hatayı 3+ defa düzeltmeye çalışıyor ama düzeltmiyor
- Testleri yoruma çeviriyor veya kolaylaştırıyor
- Gittikçe daha fazla dosya değiştirip daha çok hata çıkıyor
- `/usage` hızla artıyor, çıktı yok
- Dosyaları ileri geri değiştiriyor

**Müdahale:**
1. `Ctrl+C` ile hemen durdur (veya `claude agents` ile arka plan ajanını öldür: `tmux kill-session`)
2. `git status` ve `git diff --stat` ile durumu gör
3. Son iyi commit'i tespit et: `git log --oneline -10`
4. Hata logunu dosyaya kaydet: çıktıyı `ERROR_LOG.md` olarak kopyala
5. Değişiklikleri geri al (son iyi commit'e):
   ```bash
   git checkout .  # çalışma dizinini temizler
   # veya son commit'e dön (daha sert):
   # git reset --hard <iyi-commit-hash>
   ```
6. `/clear` ile temiz oturum aç veya tamamen yeni bir claude başlat
7. ERROR_LOG.md'yi vererek **farklı bir yaklaşım** dene
8. Hala olmuyorsa model değiştir (Opus → Sonnet, veya tam tersi), kendin çöz

### Playbook 2: Context Zehirlenmesi

**Belirtiler:**
- Daha önce düzelttiğini söylediği bir şeyi ısrarla yanlış yapıyor
- Sen "X yanlış" diyorsun o "X doğru" diye ısrar ediyor
- Kodda geriye doğru gidiş (önceden çalışan şey bozuluyor)
- Kuralları çiğnemeye başladı, daha önce duyduğu talimatları unutuyor

**Müdahale:**
1. Hemen durumu özetlet:
   ```
   DUR. Şu ana kadar yaptıklarımızı, hangi yaklaşımları denediğimizi,
   hangi hataları aldığımızı, hangi varsayımların REDDEDİLDİĞİNİ
   PROGRESS.md dosyasına tarafsızca yaz.
   ```
2. Hatalı hipotezi açıkça reddet:
   "X yaklaşımı tamamen yanlış, bundan sonra HİÇ kullanmayacağız. Y yaklaşımını deneyeceğiz."
3. `/compact` özetlemeyi dene
4. Hala devam ediyorsa:
   - PROGRESS.md'yi yazdır
   - `/clear` ile yeni oturum
   - Yeni oturumda PROGRESS.md ile başla ve yanlış yaklaşımı açıkça bildir

### Playbook 3: Büyük Değişiklik Kodu Batırdı

**Belirtiler:**
- 30+ dosyada değişiklik var
- Testlerin yarısı kırık
- Nereye dokunacağınızı bilemiyorsunuz
- Claude da durumu kurtaramıyor

**Müdahale:**
1. Panik yapmayın. Her şey git'te.
2. Bir "batık" işaret commit'i at ki geri dönebilesin:
   ```bash
   git add -A
   git commit -m "wip: batık durum, dönüş noktası"
   ```
3. Son yeşil commit'i bul:
   ```bash
   git log --oneline  # Son temiz commit'i bulun
   ```
4. Yeni bir temiz branch aç:
   ```bash
   git checkout -b fix/clean-start <iyi-commit-hash>
   ```
5. Değişiklikleri mantıksal parçalara ayırın:
   - Hangi özellik parçaları gerçekten gerekli?
   - Hangi değişiklikler yanlıştı?
6. Parça parça, her parçada testleri çalıştırarak ilerleyin
7. Gerekirse bazı değişiklikleri tamamen yeniden yazdırın (temiz başlangıç daha hızlıdır)

### Playbook 4: Maliyet Kontrolden Çıktı

**Belirtiler:**
- `/usage` ile bakıyorsunuz bir oturumda $10+ harcanmış
- Paralel görevler açık ve hepsi token yakıyor
- Ajan döngüde ve tekrar tekrar çağrı yapıyor

**Müdahale:**
1. Hemen çalışan ajanları kontrol et:
   ```bash
   claude agents              # Yerel ajanlar
   tmux ls                    # Tmux oturumları
   ```
2. Döngüdeki ajanları durdur:
   ```bash
   # Çalışan tmux oturumlarını öldür
   tmux kill-session -t <isim>
   # Arka plan ajanlarını durdur ( claude agents çıktısından ID ile)
   ```
3. Anthropic Console'da kullanımı kontrol et, hard limit koy (henüz yoksa)
4. Kök nedeni analiz et: genellikle bir test döngüsü veya yanlış task tanımıdır
5. Kalan görevi `--max-budget-usd` limiti ile yeniden başlat
6. Not al ve bir daha olmaması için hook/limit ekle

### Playbook 5: Şüpheli Komut/Güvenlik İhlali

**Belirtiler:**
- Claude `curl ... | sh` gibi bir komut öneriyor
- `.env` dosyasını okumaya çalışıyor
- Bilmediğiniz bir URL'ye bağlanıyor
- 50+ komutu birden `&&` ile zincirlemeye çalışıyor (v2.1.90 öncesi deny bypass riski)
- `sudo` veya `rm -rf` ile bir şeyler silmeye çalışıyor

**Müdahale:**
1. Hemen `Ctrl+C` ile komutu çalıştırmasını ENGELLE. İzin penceresinde "Deny" de.
2. Claude'u Durdur:
   ```
   DUR. Bu komut neden çalıştırılacak? Nereden geldi? Hangi amaca hizmet ediyor?
   ```
3. CLAUDE.md veya dış bir kaynaktan prompt injection olup olmadığını kontrol et (yeni klonladığınız repo'daki CLAUDE.md olabilir).
4. Eğer yeni bir repo ve şüpheli komut geldiğinde:
   - Önce CLAUDE.md dosyasını kendiniz okuyun
   - `.mcp.json` ve hook dosyalarını denetleyin
   - İlk çalıştırmada `--permission-mode plan` ile sadece öneri alıp çalıştırmayın
5. PreToolUse hook/dcg guard yoksa hemen kurun
6. Claude Code sürümünüzü kontrol edin: `claude --version` ≥ 2.1.90 olmalı (50 subcommand bypass fix'i içeren sürüm)

### Playbook 6: Yanlış Branch/Worktree'de Çalıştınız

**Belirtiler:**
- Yanlış branch'te değişiklik yapmışsınız
- Commit'ler yanlış branch'e gitti

**Müdahale:**
```bash
# Değişiklikleri kaybetmeden doğru worktree'ye taşı:
git stash
git worktree add ../dogru-worktree -b dogru-branch
cd ../dogru-worktree
git stash pop

# Yanlış branch'teki son commit'i taşıma:
git cherry-pick <hash>
```

### Playbook 7: Ajan Testleri Hileli Geçiriyor

**Belirtiler:**
- Testleri yazıyor ama test gerçek kodu çağırmıyor
- Veriyi hardcode ediyor
- Asenkron hataları yok sayıyor
- `try { } catch { }` ile tüm hataları yutuyor
- Test yazdığı fonksiyonu çağırmıyor bile

**Müdahale:**
1. Hemen durdur. Bu ciddi bir sorundur.
2. Yanlış testleri geri al:
   ```bash
   git checkout -- tests/  # Testleri sıfırla (eğer yanlışsa)
   ```
3. Ayrı bir subagent/oturum ile sadece test yazma görevi ver (test mühendisi ajanı)
4. Daha da iyisi: kendiniz 1-2 test yazın ve ajana "buna benzer devam et" deyin
5. Test kalitesini zero-context reviewer ile tekrar inceletin
6. Kuralı net koy: "Testleri hiçbir şekilde yutma, kodu değiştir, testleri kolaylaştırma. Test başarısızsa kodu düzelt."

---

### Kontrol Listeleri

#### Günlük Başlangıç Kontrolü
- [ ] `git status` ile dünden kalan değişiklikler kontrol
- [ ] `git pull` ile ana branch'i güncelle
- [ ] `claude --version` ve `claude doctor` (haftada bir)
- [ ] Çalışan tmux/arka plan ajanları kontrol: `tmux ls`, `claude agents`
- [ ] Bugünkü hedefi 1 cümle ile belirle
- [ ] Claude'u temiz oturumda başlat
- [ ] CLAUDE.md'nin hâlâ geçerli olduğunu teyit et

#### Görev Vermeden Önce Kontrol Listesi
- [ ] Görev net tanımlı mı?
- [ ] Kabul kriterleri yazılı mı?
- [ ] Kısıtlar (neler yapılamaz) açık mı?
- [ ] Hangi dosyalarla ilgili olduğu belli mi?
- [ ] Hangi model kullanılmalı? (basit iş → Haiku, normal → Sonnet, zor → Opus)
- [ ] Effort seviyesi belirlendi mi? (basit için low)
- [ ] Onay gerektiren durumlar var mı? (migration, yeni paket vb.)
- [ ] Mevcut context temiz mi? Çok uzunsa `/compact` veya `/clear`

#### PR Göndermeden Önce Kontrol Listesi
- [ ] Tüm testleri kendiniz çalıştırın (Claude'un "geçti" demesi yetmez)
- [ ] Typecheck/lint temiz mi?
- [ ] Build alabiliyor mu? (`pnpm build`)
- [ ] `git diff` ile değişiklikleri kendiniz okuyun
- [ ] Yeni bağımlılık var mı? Neden gerekli?
- [ ] Secret, API key veya hassas veri karışmış mı? (grep ile kontrol)
- [ ] Migration varsa incelediniz ve yedek aldınız mı?
- [ ] Testler gerçekten kodu mu test ediyor yoksa kolay mı yazılmış?
- [ ] Sıfır-bağlam bir review yaptınız mı? (kendiniz veya subagent ile)
- [ ] Gereksiz console.log, debug kodu, yorum kalmış mı?
- [ ] Değişiklikler ne kadar büyük? >500 satır ise bölmek gerek mi?

#### Haftalık Kontrol
- [ ] `/usage` raporlarını gözden geçir, haftalık toplam maliyet ne?
- [ ] `claude skills` ile hangi yeni beceriler eklenmiş, güncel mi?
- [ ] Haftalık retro yapıldı, dersler kaydedildi mi?
- [ ] Eval setinden 3-5 soru test edildi mi?
- [ ] Yeni hook/skill/MCP eklenecek mi?
- [ ] Claude Code güncellemesi var mı: `claude update`
- [ ] Takım plugin'inin güncel sürümü kullanılıyor mu?
