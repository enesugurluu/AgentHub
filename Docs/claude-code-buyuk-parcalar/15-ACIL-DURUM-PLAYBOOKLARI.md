## 15. Acil Durum Playbook'ları ve Kontrol Listeleri

### Playbook 1: Ajan Döngüye Girdi (Debeleniyor)

**Belirtiler:**
- Aynı hatayı 3+ defa düzeltmeye çalışıyor ama düzeltmiyor
- Sürekli aynı dosyayı değiştirip duruyor
- Testleri yoruma çeviriyor veya basitleştiriyor
- Gittikçe daha fazla hata çıkıyor
- Maliyet hızla artıyor

**Müdahale:**
1. `Ctrl+C` ile hemen durdur
2. `git status` ile durumu gör
3. Son iyi commit'i tespit et (`git log --oneline -10`)
4. Hata logunu bir dosyaya kaydet: `claude` çıktısını kopyala → `ERROR_LOG.md`
5. Değişiklikleri geri al: `git checkout .` (veya son iyi commit'e `git reset --hard <hash>`)
6. `/clear` ile temiz oturum aç
7. ERROR_LOG.md'yi vererek **farklı bir yaklaşım** dene
8. Hala olmuyorsa, farklı model kullan veya kendin çöz

### Playbook 2: Context Zehirlenmesi

**Belirtiler:**
- Daha önce düzelttiğin bir şeyi ısrarla yanlış yapıyor
- Sen "X yanlış" diyorsun o "X doğru" diye ısrar ediyor
- Kodda geriye doğru gidiş (önceden çalışan şey bozuluyor)

**Müdahale:**
1. Hemen durumu özetlet:
   "Dur. Şu ana kadar yaptıklarımızı, hangi yaklaşımları denediğimizi,
    hangi hataları aldığımızı bir PROGRESS.md dosyasına tarafsızca yaz.
    Yanlış olduğunu kabul ettiğin şeyleri de dahil et."
2. Hatalı hipotezi açıkça reddet:
   "X yaklaşımı tamamen yanlış, bunu bundan sonra hiç kullanmayacağız."
3. Gerekirse `/clear`
4. Yeni oturumda PROGRESS.md ile başla

### Playbook 3: Büyük Bir Değişiklik Kodu Batırdı

**Belirtiler:**
- 30+ dosyada değişiklik var
- Testlerin yarısı kırık
- Nereye dokunacağınızı bilemiyorsunuz

**Müdahale:**
1. Panik yapmayın.
2. Önce mevcut branch'te bir "batık" işaret commit'i at:
   `git commit -am "wip: batık durum, geri dönüş noktası"`
3. Son yeşil commit'i bul:
   `git log --oneline` ile tüm testlerin geçtiği son commit'i bulun
4. Yeni bir temiz branch açın:
   `git checkout -b fix/clean-start <iyi-commit-hash>`
5. Değişiklikleri parçalara ayırın:
   - Hangi dosyalar gerçekten gerekli?
   - Hangi değişiklikler yanlıştı?
6. Parça parça, her parçada testleri çalıştırarak ilerleyin
7. Gerekirse bazı değişiklikleri tamamen yeniden yazdırın (daha temiz olur)

### Playbook 4: Maliyet Kontrolden Çıktı

**Belirtiler:**
- `/usage` ile bakıyorsunuz 10$+ bir oturumda harcanmış (eski `/cost` yerine `/usage`)
- `claude agents` çok sayıda çalışan gösteriyor
- `claude --bg` ile tetiklenen görevler beklenenden fazla tur dönmüş

**Müdahale:**
1. Hemen tmux oturumlarını kontrol et: `tmux ls` çalışıyor mu
2. Gerekirse döngüde olan ajanları durdur: `tmux kill-session -t <name>`
3. Anthropic Console'dan kullanımı kontrol et
4. Ayar bir hard limit (henüz yoksa)
5. Maliyeti düşürmek için:
   - Uzun oturumları kapat, temiz oturumlar aç
   - Opus kullanıyosan Sonnet/Haiku'ya dön
   - Daha az paralel iş çalıştır
6. Kök nedeni analiz et ve not al (genelde döngüdeki bir ajandır)

### Playbook 5: Deny Kural Atlamasından Şüpheleniyorsunuz (50-Subcommand Bypass)

**Belirtiler:**
- Ayarlarınızda açıkça deny ettiğiniz bir komut (curl, rm -rf) çalışıyor
- Claude onay ekranında "51 subcommands, too many to safety-check individually" benzer bir mesaj çıkıyor
- CLAUDE.md'ye yeni eklediğiniz bir kuralın aniden işe yaramadığını görüyorsunuz

**Neden (v2.1.90 öncesi):** bkz. Bölüm 12. 50+ alt komut içeren zincirlerde deny kural analizi atlanıyor.

**Müdahale:**
1. Hemen `Ctrl+C` ile ajanı durdurun.
2. `claude --version` kontrol edin — v2.1.89 veya altıysa **hemen** `claude update`.
3. PreToolUse hook'unuzun aktif olduğunu doğrulayın: `.claude/settings.json` ve `guard.py`. Deny kuralları prompt tabanlıdır; hook'lar enforcement katmanıdır.
4. Komutun ne yapmaya çalıştığını kaydedin (terminal log, screenshot). Şüpheliyse ağ bağlantısını kesin.
5. Çalışmış olabilecek ters etkiyi inceleyin: dış bağlantı, dosya değişikliği, credential erişimi.
6. Gerekirse API anahtarlarını döndürün; ağ erişim loglarını kontrol edin.

### Playbook 6: Arka Plandaki Ajanı Durdurma

```bash
claude agents                       # çalışan/tamamlanmış listesi
claude -r <session-id>              # konsolu attach et
# İçeride Ctrl+C ile durdurabilirsiniz.

# Alternatif: tmux kullandıysanız
tmux ls
tmux kill-session -t <name>
```

`--max-budget-usd` ve `--max-turns` limitlerini daima `--bg` ile birlikte kullanın; kontrolü kaçıran ajan limitte otomatik durur.

### Playbook 7: MCP/Plugin Şüpheli Davranıyor

**Belirtiler:** yeni yüklediğiniz MCP/pluginden sonra anormal ağ trafiği, yoklama, beklenmedik tool çağrıları.

**Müdahale:**
1. `/mcp` ile aktif sunucuları listeleyin.
2. Şüpheli sunucuyu `claude mcp remove <name>` ile kaldırın.
3. `.mcp.json` ve settings dosyalarını gözden geçirin.
4. Yeni MCP eklerken daima en küçük yetki ile başlayın; stdino yerel çalıştırmayı tercih edin.

---

### Günlük Başlangıç Kontrol Listesi

```markdown
# Claude Code Günlük Başlangıç

- [ ] `git status` ile dünden kalan değişiklikleri kontrol et
- [ ] Ana branch'i güncelle: `git pull origin main`
- [ ] Bugünkü hedefi belirle ve 1 cümle ile yaz
- [ ] Claude'u temiz oturumda başlat
- [ ] CLAUDE.md'yi hızlıca gözden geçir (güncel mi?)
- [ ] Uzun görevler tmux'ta çalışmaya devam ediyorsa kontrol et
- [ ] Dünkü maliyet raporuna bak (ne kadar harcandı?)
```

### Kod Yazdırma Öncesi Kontrol Listesi

```markdown
# Görev Vermeden Önce

- [ ] Görev net tanımlı mı? (ne yapacağı belli)
- [ ] Kabul kriterleri yazılı mı? (bittiğini nasıl anlayacağız)
- [ ] Kısıtlar belirli mi? (neler yapılamaz)
- [ ] Hangi dosyalarla ilgileneceği gösterildi mi?
- [ ] İlgili bağlam verildi mi? (CLAUDE.md'de varsa sorun yok)
- [ ] Bu görev hangi model için uygun? (Opus/Sonnet/Haiku?)
- [ ] Önce test mi, önce kod mu?
- [ ] İnsan onayı gereken bir durum var mı? (migration, yeni paket vb.)
```

### PR Göndermeden Önce Kontrol Listesi

```markdown
# Claude'dan PR Alırken

- [ ] Tüm testler gerçekten geçiyor mu? (Claude "geçti" diyebilir, kendin çalıştır)
- [ ] Typecheck/lint temiz mi?
- [ ] Build alabiliyor mu? (pnpm build)
- [ ] Değişiklikler mantıklı mı? (diff'i oku)
- [ ] Yeni bağımlılıkler var mı? Neden eklendi?
- [ ] Secret veya hassas veri karışmış mı? (git diff ile gözle tara)
- [ ] Migration varsa incelediniz mi?
- [ ] Testler gerçekten kodu mu test ediyor yoksa kolayca mı yazılmış?
- [ ] Zero-context review yaptınız mı?
- [ ] Gereksiz console.log, yorum, debug kodu kalmış mı?
```

---

