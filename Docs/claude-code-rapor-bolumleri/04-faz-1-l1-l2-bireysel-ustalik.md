## 4. FAZ 1: L1 → L2 — Bireysel Ustalık (Hafta 1-4)

Bu fazın hedefi: Claude Code'un uzantı mimarisini (Skills, MCP, Hooks, Subagents) özümseyerek bireysel verimliliği 5-10x'e çıkarmak.

### 4.1 Claude Code Komut Ustalığı

#### Oturum İçi Temel Slash Komutları

| Komut | Amaç | Kullanım Sıklığı |
|:---|:---|:---|
| `/help` | Tüm komutları göster | Gerektiğinde |
| `/clear` | Konuşma geçmişini temizle, yeni oturum | Her görev arası |
| `/compact` | Context'i özetle, temizleyerek devam et | Context 50-80k token olduğunda |
| `/usage` | Token ve maliyet raporu (/cost yerine) | Günlük sonunda |
| `/model` | Model değiştir (Opus/Sonnet/Haiku/Fable) | Görev zorluğuna göre |
| `/effort` | Zeka seviyesi: low/medium/high/xhigh/max | Her görev başında |
| `/mcp` | MCP sunucu durumu, yetkilendirme | Yeni entegrasyonda |
| `/skills` | Mevcut becerileri listele/yönet | Gerektiğinde |
| `/resume` | Önceki oturumlardan seç ve devam | /clear sonrası |
| `/recap` | Bu oturumun özetini çıkar | Ara verme öncesi |
| `/loop` | Bir prompt'u periyodik tekrarla | Tekrarlayan kontroller |
| `/login` | Hesap değiştir/yeniden kimlik doğrulama | Gerektiğinde |
| `/doctor` | Sistem sağlık kontrolü | Hata durumunda |
| `/init` | CLAUDE.md'yi otomatik oluşturma önerisi | Yeni projede |
| `# ...` | Memory'ye bir not ekle (örn: `# Auth0 kullanıyoruz`) | Anlık dersler |
| `!komut` | Doğrudan bash çalıştır (örn: `!git status`) | Hızlı kontroller |
| `@dosya` | Bir dosyayı açıkça context'e dahil et | Belirli dosya için |

#### CLI Bayrakları (Shell'den)

```bash
claude                          # Normal interaktif
claude "hatayı düzelt"          # Tek seferlik görev
claude -p "sorgu"               # Pipe/script için non-interactive
claude -c                       # Son oturuma devam et
claude -r                       # Oturum seçici
claude --worktree fix/auth      # Otomatik yeni worktree'de başlat
claude --bg "uzun görev"        # Arka planda ajan
claude --permission-mode plan   # Sadece plan yapsın, dosya değiştirmesin
claude --max-turns 10           # Maksimum tur limiti
claude --max-budget-usd 5       # Maksimum $5 harcama
claude -p --output-format json  # JSON çıktı (otomasyon için)
```

### 4.2 Etkili Prompt Kalıpları

#### KALIP 1: Görev Tanımı + Bağlam + Kabul Kriterleri
```
Görev: Kullanıcı profil sayfasına "son oturum açma tarihi" alanı ekle.

Bağlam:
- apps/web/app/profile/page.tsx dosyası mevcut
- API endpoint apps/api/src/routes/users/[id]/route.ts user objesi dönüyor
- User.lastLoginAt Prisma şemada mevcut

Kabul Kriterleri:
1. Tarih formatı: "2 Ocak 2026, 14:30" (Türkçe)
2. Sayfa yüklenirken skeleton göster
3. Hiç oturum açılmamışsa "İlk oturum" göster
4. Diğer alanlar bozulmadan çalışmalı
5. En az 1 unit test

Önce test yaz, sonra implementasyonu yap.
```

#### KALIP 2: Adversarial (Zero-Context) Self-Review
```
Az önce yazdığın kodu incele. Sıfır bağlamla (sadece koda bakarak) şu
potansiyel sorunları tara:
1. TypeError/null/undefined riski
2. Off-by-one ve sınır koşulu hataları
3. Async/await eksikleri ve yarış koşulları
4. Güvenlik açıkları (injection, XSS, auth bypass)
5. Kaynak sızıntısı (connection, dosya)
6. Performans sorunları (N+1 query, gereksiz döngü)

Bulduğun her sorun için satır numarası ile nedenini açıkla ve doğrudan düzelt.
```

#### KALIP 3: Plan Önce, Kod Sonra (Büyük Görevler)
```
Önce bu kod tabanında [özellik] ile ilgili mevcut yapıyı incele:
1. İlgili dosyaları listele (değiştirilecekler ve etkilenebilecekler)
2. Mevcut veri akışını açıkla
3. Değişiklik planını adım adım sun
4. Riskleri ve yan etkileri belirt

KOD YAZMA. Planı onaylat.
```

#### KALIP 4: Subagent Kullanımı (Araştırma/Review İçin)
"/" komutundan sonra bir subagent'ı doğrudan çağırabilirsiniz. Alt ajan izole bir context'te çalışır, sadece nihai yanıt/özeti ana oturuma döner; ana context şişmez.

Örneğin özel bir `code-reviewer` subagent'ınız varsa:
```
@code-reviewer Son değişiklikleri incele ve sorunları listele.
```

### 4.3 Context Yönetimi Disiplini

#### Dumb Zone Tespiti
Her modelin güvenilir çalıştığı bir context aralığı vardır. ~80-100k token üstünde belirtiler görülürse **hemen müdahale edin**:

**Dumb Zone Belirtileri:**
- Aynı hatayı düzeltmek yerine testleri yoruma alıyor veya kolaylaştırıyor
- Daha önce defalarca düzelttiği bir detayı tekrar yanlış yapıyor
- "Bunu zaten yapmıştım" diyor ama yapmamış
- Import'lar kayboluyor/yanlış geliyor
- Önceden belirtilen kuralları görmezden geliyor
- Aynı kodu döngüde tekrar tekrar değiştiriyor

**Müdahale Protokolü:**
1. Önce `/compact` ile context'i özetlet
2. Hala devam ederse durum özetini yazdır:
   ```
   Dur. Şu ana kadar yapılanları, hangi yaklaşımların denendiğini,
   neyin işe yarayıp neyin yaramadığını PROGRESS.md olarak özetle.
   Sonraki net adımı da ekle.
   ```
3. `/clear` ile temiz oturum
4. Yeni oturumda: `PROGRESS.md'yi oku ve kaldığı yerden devam et`

#### Checkpoint Stratejisi
Her mantıksal ara hedefte commit attırın:
```bash
# İyi commit noktaları (bunları Claude kendine alışkanlık etmeli):
- Her test grubu geçtiğinde
- Her ana bileşen tamamlandığında
- Refactor öncesi ve sonrası
- Her büyük feature adımında

# Claude'a doğrudan söyletmek:
"Şu anki durumu anlamlı bir Conventional Commits mesajı ile commit et."
```

**Geri dönüşler:**
```bash
git reset HEAD~1           # Son commit'i geri al (değişiklikler korunur)
git reset --hard HEAD~1    # DİKKAT: Son commit'i tamamen siler
git checkout .             # Tüm kaydedilmemiş değişiklikleri geri al
```

### 4.4 CLAUDE.md Ustası Olmak

#### CLAUDE.md En İyi Uzunluk
Anthropic'in kendi tavsiyesi ve topluluk tecrübesi: CLAUDE.md **100-300 satır** arasında olmalı. Daha uzun olanlar hem cache verimini düşürür hem de ortadaki kuralların Claude tarafından kaçırılma olasılığını artırır.

Uzun içerikler için:
- Kod standartları → `.claude/rules/code-style.md`
- Test kuralları → `.claude/rules/testing.md`
- API desenleri → `.claude/rules/api-conventions.md`
- Karmaşık prosedürler → `.claude/skills/<isim>/SKILL.md`
- Mimari dokümantasyon → `.claude/docs/architecture.md`

#### Etkili Kural Yazımı
Kuralları "yapma" yerine "yap" şeklinde, açık ve test edilebilir yazın:

❌ **Kötü:** "Güvenliğe dikkat et."
✅ **İyi:** "Yeni bir endpoint yazdığında: (1) auth kontrolü, (2) zod ile input validation, (3) try-catch ile hata yönetimi sırasıyla ekle."

❌ **Kötü:** "İyi kod yaz."
✅ **İyi:** "Her yeni fonksiyondan sonra: dönüş tipi tanımla, hatayı fırlat (yutma), 1 satır JSDoc ekle."

### 4.5 Uzantıları Tanımak: Skills, MCP, Hooks, Subagents

#### Skills (Beceriler)
Tekrarlayan iş akışlarınızı bir kez öğretin, Claude her ihtiyaç duyduğunda otomatik olarak kullansın. Manuel tetikleme için de `/skill-adi` ile çağırabilirsiniz.

**Yapı:**
```
.claude/skills/
└── guvenli-test/
    └── SKILL.md
```

**Örnek Skill (`.claude/skills/tdd/SKILL.md`):**
```markdown
---
name: tdd
description: Test-Driven Development ile kod yaz. Önce test, sonra kod, sonra refactor.
disable-model-invocation: false  # Otomatik tetiklenebilir
---

# TDD (Red-Green-Refactor) Prosedürü

1. Bir özellik istendiğinde ÖNCE o özellik için test yaz.
2. Testi çalıştır ve kırmızı olduğunu doğrula.
3. Testi geçirecek en kısa kodu yaz.
4. Testin geçtiğini doğrula.
5. Kodu temizle/refactor et, testlerin hala geçtiğini doğrula.

Kurallar:
- Testleri asla yorum satırına alma veya sil.
- Kod yazarken testleri geçmekten başka bir amaç gütme.
- Birden fazla test aynı anda kırmızıysa, önce sadece birini yeşile döndür.
```

`disable-model-invocation: true` yaparsanız Claude bu skill'i otomatik kullanmaz, sadece siz `/tdd` diye çağırdığınızda kullanır. Bu, manuel tetikleme istediğiniz işler (deploy vb.) için kullanılır.

#### MCP (Model Context Protocol)
Claude'a dış araçlara doğrudan erişim verir. Popüler kullanım alanları:
- GitHub MCP: Issue/PR oluşturma, kod arama
- Postgres/SQLite MCP: Sorgu çalıştırma, şema inceleme
- Dosya sistemi: Hassas dizinlere kontrollü erişim
- Puppeteer/Browser: Tarayıcı otomasyonu, E2E test
- Sentry: Hata kayıtlarını çekme
- Slack: Bildirim gönderme
- Özel internal API'ler

Basit başlangıç MCP'si (`.mcp.json`):
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {"GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"}
    }
  }
}
```

> Güvenlik uyarısı: Sadece güvendiğiniz MCP sunucularını ekleyin; kötü niyetli sunucular prompt injection saldırısı yapabilir.

#### Hooks (Olay Güdümlü Scriptler)
Belirli yaşam döngü olaylarında otomatik scriptler çalıştırmak için kullanılır. Güvenlik ve otomasyon için çok değerlidir.

Desteklenen hook olayları:
- `SessionStart` — Oturum açıldığında
- `PreToolUse` — Her araç çağrısı öncesi (Bash, Edit, Write vb.)
- `PostToolUse` — Araç çağrısı sonrası
- `PreCompact` — Context sıkıştırılmadan önce
- `PostCompact` — Context sıkıştırıldıktan sonra
- `UserPromptSubmit` — Kullanıcı prompt gönderdiğinde
- `Stop` — Claude yanıtı bitirdiğinde

**Örnek Hook: Düzenleme sonrası otomatik format**
`.claude/hooks/auto-format.sh` (çalıştırma izni alınmış):
```bash
#!/bin/bash
INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')
if [[ $FILE == *.ts || $FILE == *.tsx ]]; then
  npx prettier --write "$FILE" 2>/dev/null
fi
echo '{}'
```

`.claude/settings.json`'a ekleyin:
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [{"type": "command", "command": ".claude/hooks/auto-format.sh"}]
      }
    ]
  }
}
```

Böylece Claude her dosya düzenlediğinde otomatik Prettier çalışır.

Güvenlik hook'u örneği için Güvenlik bölümüne bakın (PreToolUse ile komut engelleme ve `dcg`).

#### Subagents (Alt Ajanlar)
Alt ajanlar izole context'te çalışan, ana oturumu şişirmeyen uzman kişiliklerdir. Özellikle **paralel araştırma** ve **karmaşık multi-konular için kullanılır.

**Basit subagent tanımı (`.claude/agents/code-reviewer.md`):**
```markdown
---
name: code-reviewer
description: Kodları sıfır bağlamla şüpheci gözle inceleyen güvenlik uzmanı
tools: Read, Grep, Glob, Bash  # Sadece okuma yetkisi
model: sonnet
---

Sen bir kod inceleme uzmanısın. Sana verilen kodları BAĞIMSIZ olarak, yani
bu kodun kim tarafından ne amaçla yazıldığını bilmeden incele.

Sadece şu kategorilerde hata tara:
1. Çalışma zamanı hataları (null, type, async vb.)
2. Güvenlik açıkları
3. Test eksikliği ve yetersizliği
4. Performans sorunları
5. Açık kaynaklı bileşenlerin yanlış kullanımı

Yapıcı ol, her sorun için dosya:satır referansı ver, düzeltme önerisi sun.
Eğer kritik bir hata yoksa kısaca "LGTM" de ve neden güvendiğini belirt.
```

Kullanım: Ana Claude oturumunda şöyle dersiniz:
```
@code-reviewer src/auth/login.ts dosyasını incele.
```

Alt ajan kendi context'inde çalışır, araçlar kullanır ve sadece inceleme sonucunu ana oturuma döndürür.

### 4.6 Hafıza Sistemi (L2 Seviye)

#### Dahili Memory (/# prefix)
Konuşma içinde `#` ile başlayan notlar Claude'un otomatik hafızasına eklenir:
```
# Bu projede tarih formatı olarak date-fns kullanıyoruz, moment yok.
# Auth middleware her zaman apps/api/src/middleware/auth.ts'den gelir.
```

Claude sonraki oturumlarda da bu notları hatırlar. Notları `/memory` komutu ile yönetebilirsiniz.

#### Harici Hafıza: Markdown + Git
Daha ciddi kalıcı hafıza için:

```
.claude/
├── memories.md          # Ana hafıza: kalıcı dersler ve kararlar
└── sessions/            # Önemli oturumların özetleri
    ├── 2026-08-01-auth-refactor.md
    └── 2026-08-07-payment-bug.md
```

**`memories.md` şablonu:**
```markdown
# Proje Hafızası

## Çözülen Tuzaklar (Gotchas)
- [2026-08-01] Prisma $transaction timeout 5s, bağlantı havuzu 5+ olmalı
- [2026-08-03] Next.js 15 cache revalidate edilmezse stale data gösteriyor
- [2026-08-07] Redis Cluster için tüm seed node'ları vermek gerekli

## Mimari Kararlar (ADR Özetleri)
- ADR-004: Background job için BullMQ (öncelik ve retry iyi)
- ADR-005: i18n için next-intl (App Router uyumlu)

## Claude İle Deneyimler
- Büyük migrationlarda tek seferde yazmaya çalışırsa yanlış yapıyor, küçük parçalara böl
- Prisma değişikliği sonrası `prisma generate` çalıştırmayı unutuyor, hatırlat
- React Server Component'te useState kullanmaya çalışırsa "use client" uyar
```

Bu dosya çok büyüdükçe konularına göre dosyalara bölün (`hooks/`, `auth.md`, `testing.md`).

### 4.7 Özel Slash Komutlar (Skills olarak)
Eski `.claude/commands/` sistemi artık Skill'ler ile birleşti. Yine de `.claude/commands/` altına koyduğunuz `.md` dosyaları `/project:<isim>` şeklinde çalışmaya devam eder.

Becerileri `disable-model-invocation: true` ile işaretleyerek eski komutlar gibi davranmalarını sağlayabilirsiniz.

> **L2 Kontrol Noktası:**
> - [ ] Native kurulum tamam, `claude --version` ve `claude doctor` temiz
> - [ ] CLAUDE.md kısa (100-300 satır), kurallar net
> - [ ] `.claude/rules/` ile modüler kurallar tanımlı
> - [ ] tmux ile kalıcı oturum disiplini
> - [ ] Context yönetimi (/compact, /clear, checkpoint) alışkanlığı
> - [ ] Zero-context review oturumu ve subagent'lar aktif kullanılıyor
> - [ ] İlk 2-3 skill yazıldı (TDD, review vb.)
> - [ ] En az bir MCP sunucusu (GitHub vb.) bağlı
> - [ ] Bir güvenlik hook'u aktif (örn: tehlikeli komut engelleyici)
> - [ ] Bireysel verimlilik 3-5x hissedilir seviyede
