## 14. 30/60/90 Günlük Uygulama Yol Haritası

### İlk 30 Gün: Temel Kurulum ve Bireysel Ustalık (L0 → L2)

**Hafta 1: Native Kurulum ve İlk Alışkanlıklar**
- [ ] Native installer ile Claude Code'u kur (`curl -fsSL https://claude.ai/install.sh | bash`)
- [ ] `claude --version` ve `claude doctor` ile kurulumu doğrula
- [ ] tmux ve temel terminal araçlarını ayarla
- [ ] İlk proje için `CLAUDE.md` yaz (100-200 satır)
- [ ] Global `~/.claude/CLAUDE.md` oluştur
- [ ] Basit `.claude/settings.json` ile allow/deny/ask izinleri
- [ ] `.claudeignore` yaz
- [ ] Her gün 1-2 saat Claude ile çalış, alışkanlık kazan
- [ ] Her akşam `/usage` ile tüketimi kontrol et

**Hafta 2: Context Yönetimi ve İlk Uzantılar**
- [ ] Context yönetimi: `/compact`, `/clear`, checkpoint disiplini
- [ ] `#` prefix'i ile notlar almak (memory)
- [ ] `.claude/memories.md` tutmaya başla
- [ ] Dumb zone'u tanımayı öğren
- [ ] İlk kural dosyası: `.claude/rules/code-style.md`
- [ ] İlk 2-3 slash komut/skill (review, tdd, refactor)
- [ ] Basit hataları kendisine düzelttir

**Hafta 3: Doğrulama ve Subagent'lar**
- [ ] Zero-context adversarial review pratiği (subagent veya ayrı oturum)
- [ ] İlk subagent tanımı: `.claude/agents/code-reviewer.md`
- [ ] Test → Kod → Review akışını benimse
- [ ] Testleri ayrı bir düşük model (Haiku) ile yazdırma deneyleri
- [ ] İlk güvenlik hook'u (PreToolUse ile tehlikeli komut engeli)
- [ ] İlk deterministik döngüyü (test→kod→test) çalıştır
- [ ] İlk 2 beceri dosyasını (.claude/skills/) oluştur

**Hafta 4: MCP, Worktree ve Paralellik**
- [ ] İlk MCP sunucusunu bağla (GitHub MCP önerilir)
- [ ] İlk paralel worktree denemesi (2 basit görev)
- [ ] `claude --worktree` ile otomatik worktree denemesi
- [ ] Runtime izolasyonu (port/.env.local) unutma
- [ ] `/effort` ayarlarını aktif kullanmaya başla
- [ ] Uzun bir gece görevini arka planda (`--bg`) çalıştırmayı dene
- [ ] L2 seviyesine ulaş: bireysel verimlilik ~5x

**30 Gün Sonu Hedef:** Claude Code'u rahat ve güvenli kullanabiliyorsunuz, CLAUDE.md/skills/hooks/MCP uzantılarını iş akışınıza dahil ettiniz, temel paralel iş mantığını kavradınız.

### Gün 30-60: Takım ve Otomasyon (L2 → L3)

- [ ] Organizasyon için ortak Claude Plugin'i/şablonu oluştur
- [ ] Takım arkadaşlarına workshop/eğitim ver
- [ ] Handoff mimarisini kur: strateji (Sonnet) → uygulama (Haiku/DeepSeek) → doğrulama (GPT/adversarial)
- [ ] Tiered model routing (işe göre model) alışkanlığı
- [ ] Adversarial AI review'ı CI/CD'ye ekle (GitHub Actions)
- [ ] Resmi `claude-code-action` kullanımını değerlendir
- [ ] Hooks'u takım genelinde standartlaştırma (dcg guard veya kendi hook'unuz)
- [ ] VPS üzerinde tmux ile 24/7 ajan filosu kur
- [ ] Tailscale mesh ağ kur
- [ ] Paralel işler için Claude Squad (veya benzeri orkestratör) dene
- [ ] Agent Teams özelliğini ilk kez kullan
- [ ] Maliyet takibi ve Anthropic Console hard limiti
- [ ] Prompt caching verimini optimize et
- [ ] Haftalık AI retro rutinini başlat
- [ ] Eval setinin ilk versiyonunu yaz ve temel skoru al
- [ ] L3 seviyesine ulaş

### Gün 60-90: Otonom Yazılım Fabrikası (L3 → L4)

- [ ] Sentry + Linear (veya GitHub Issues) entegrasyonunu kur
- [ ] Triage servisini ayağa kaldır (yerleşik Scheduled Tasks veya cron)
- [ ] İzole worktree'de otonom bug-fix akışını test et
- [ ] İlk otonom PR'ın production'a merge sevincini yaşa
- [ ] Knowledge Graph'i kurmaya başla (SQLite veya Neo4j)
- [ ] Entity/edge evanterini oluştur (ilk 20 entity)
- [ ] 7 aşamalı Graph Yaşam Döngüsü'nü işletmeye başla
- [ ] Memory Hazard'ları (duplicate, stale, contradictory) kontrol prosedürleri
- [ ] Kapalı öğrenme döngüsü: her görevden sonra otomatik skill türetme
- [ ] Beceri kütüphanesini 15+ dosyaya çıkar
- [ ] Çoklu model orkestrasyonu ve stratejiyi dökümante et
- [ ] Batch API ile toplu işleri (test yazma, refactor) %50 indirimle yap
- [ ] Periyodik eval ölçümlerini takvime bağla
- [ ] Güvenlik politikalarını yazılı hale getir ve ekiple paylaş
- [ ] Kurumsal plugin paketini yayınla ve sürüm yönetimine başla
- [ ] L4 seviyesine ulaş: sen uyurken iş üreten bir fabrika

### 90 Gün Sonrası: Sürekli İyileştirme

- Ayda bir yeni bir modeli A/B testiyle mevcut sistemle karşılaştır
- Beceri kütüphanesini sürekli büyüt ve iyileştir
- Graph hafızasını günlük besle, haftalık tutarlılık kontrolü yap
- Yeni takım üyelerine workflow'u öğret
- Organizasyon genelinde yaygınlaştır
- Kendi özel MCP sunucularını/internal araçları geliştirmeye başla
- Gerekirse özel plugin'ler ile takım çapında standartları dağıt
- Sektördeki yeni gelişmeleri (yeni MCP sunucuları, özellikler) takip et

### Haftalık Rutin (Alışkanlık Haline Getirin)

| Gün | Aktivite | Süre |
|:---|:---|:---:|
| Her sabah | `git status`, Claude oturumlarını kontrol, günlük hedef | 5 dk |
| Her gün sonu | `/usage` kontrol, gerekirse checkpoint commit, kapanış özeti | 5 dk |
| Her Cuma | Haftalık retro: notlar, yeni skill, iyileştirme alanları | 30 dk |
| Her Pazartesi | Eval setinden 3 soru test et, doğruluk oranını kaydet | 15 dk |
| Ay sonu | Metrikler ve maliyet raporu, hedef güncellemesi | 1 saat |
