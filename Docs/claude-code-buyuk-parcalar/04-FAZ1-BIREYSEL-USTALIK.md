## 4. FAZ 1: L1 → L2 — Bireysel Ustalık (Hafta 1-4)

Bu fazın hedefi: Claude Code'u **dizginsiz ve güvenilir** şekilde çalıştırmak, uzantı katmanlarını (MCP, Skills, Hooks, Subagents) tanımak, bireysel verimliliği 5-10x'e çıkarmak.

### 4.1 Claude Code Komut Ustalığı (2026)

#### Temel Slash Komutları (güncel)

| Komut | Amaç | Sıklık |
|:---|:---|:---|
| `/help` | Tüm komutları göster | Gerektiğinde |
| `/clear` | Yeni oturum (context sıfırla) | Her görev arası |
| `/compact` | Context'i özetle | Context şiştiğinde |
| `/usage` | Bu oturumun token/maliyet özeti (eski `/cost`) | Her oturum sonu |
| `/model` | Model değiştir | Gerekli oldukça |
| `/effort` | Zeka seviyesi: low/medium/high/xhigh/max | Her görevin başında |
| `/mcp` | MCP sunucu durumu / auth | Entegrasyon |
| `/skills` | Mevcut becerileri listele/yönet | Haftada bir |
| `/login` | Hesap değiştir/yeniden giriş | Seyrek |
| `/loop` | Bir prompt'u periyodik tekrarla | Otomasyon |
| `/recap` | Context özeti (uzak dönüşte) | Ara verdiğinizde |
| `/schedule` | Bulut zamanlanmış görevler | Asenkron iş |
| `/tasks` | Görev listesi (Agent Teams) | Takım çalışmasında |
| `/init` | CLAUDE.md sihirbazı | Yeni projede |
| `/config` | Interaktif ayar düzenleyici | Kurulumda |

CLI flags (oturum açmadan):
```bash
claude -c / --continue        # son oturumu devam ettir
claude -w <branch>            # yeni worktree'de aç
claude --bg "görev"           # arka plan ajanı
claude agents                 # çalışan/tamam ajanları listele
claude --effort high          # zeka seviyesi sabitle
claude --max-budget-usd 5     # harcama limiti
claude --max-turns 50         # tur limiti
claude -p -o json             # JSON çıktı (script)
```

#### Etkili Prompt Kalıpları

**KALIP 1: Görev + Bağlam + Kabul Kriterleri**

```
Görev: Kullanıcı profili sayfasına "son oturum açma tarihi" alanı ekle.

Bağlam:
- apps/web/app/profile/page.tsx mevcut
- API endpoint apps/api/src/routes/users/[id]/route.ts user objesi dönüyor
- Prisma User.lastLoginAt alanı mevcut

Kabul Kriterleri:
1. Tarih formatı "2 Ocak 2026, 14:30" Türkçe
2. Sayfa açılırken skeleton
3. Hiç oturum yoksa "İlk oturum"
4. Diğer alanlar bozulmasın
5. En az 1 unit test

Önce test dosyasını oluştur, sonra implementasyon.
```

**KALIP 2: Zero-Context İnceleme**
```
Az önce yazdığın kodu SIFIR BAĞLAMLA incele. Sadece kod ve hata
mesajlarına bakarak şu hataları ara:
1. TypeError null/undefined riski
2. Off-by-one
3. Async/await eksikleri
4. Güvenlik açıkları (SQLi, XSS, IDOR)
5. Eksik edge case

Bulduğun her sorunu doğrudan düzelt.
```

**KALIP 3: Plan Modu (Büyük Görevler için)**
```
/claude --permission-mode plan
"Önce bu kod tabanında X özelliği ile ilgili mevcut yapıyı incele:
1. İlgili dosyaları listele
2. Veri akışını açıkla
3. Değiştirilecek noktaları planla
4. Risk ve yan etkileri belirt
KOD YAZMA. Planı onaylat."
Plan modu Shift+Tab ile de açılıp kapatılabilir.
```

### 4.2 Context Yönetimi Disiplini

Context ~80-100k token'a yaklaştığında **dumb zone** belirir:

- Daha önce düzelttiği hatayı tekrar yapıyor
- Testleri yorum satırına çeviriyor veya kolaylaştırıyor
- Basit import'ları yanlış yazıyor
- "Bunu zaten yapmıştım" ama yapmamış
- Gereksiz tekrar döngüsüne giriyor
- Önceki talimatları görmezden geliyor

**Müdahale Protokolü:**
1. `/compact` dene.
2. Hala devam ediyorsa PROGRESS.md yazdır:
   ```
   /clear öncesi şu ana kadar yapılanları PROGRESS.md'ye özetle:
   - değiştirilen dosyalar
   - test durumu
   - bilinen sorunlar ve sıradaki adımlar
   ```
3. `/clear` ile yeni oturum.
4. Yeni oturumda: "PROGRESS.md'yi oku ve kaldığı yerden devam et."

#### Checkpoint Stratejisi

Her mantıksal ara hedefte **git commit at**:
```bash
# İyi commit noktaları: test geçtiğinde, ana bileşen bittiğinde, refactor öncesi/sonrası
"Şu ana kadarki değişiklikler için anlamlı bir commit mesajı yaz ve commit et."
```

Geri dönüş:
```bash
git reset HEAD~1          # commit'i geri al, değişiklikler korunur
git reset --hard HEAD~1   # tamamen son commit'e dön (dikkat!)
git checkout .            # çalışma dizinini temizle
```

### 4.3 İlk Doğrulama Kurulumu (Zero-Context Review)

Aynı oturumda yazdığı kodu inceletmek **güvenilmezdir** (kendi kör noktasını görmez).

**Yöntem 1: İki terminal (aynı model):**
```bash
# Terminal 1: Üretici (Generator)
claude
# Kod yazdır, geliştir

# Terminal 2: Denetleyici (Reviewer) — SIFIR BAĞLAM
claude
"Sadece şu dosyaları oku: src/auth/login.ts, src/auth/register.ts.
Sıfır bağlamla incele. Hata, güvenlik açığı, eksik test ara.
Kodu yazanın varsayımlarını görmüyorsun, sadece koda bak."
```

**Yöntem 2: Subagent (aynı oturum içinde, temiz context):**
`.claude/agents/reviewer.md` oluştur:
```markdown
---
name: reviewer
description: Zero-context code reviewer. Uses sonnet, no project memory.
model: claude-sonnet-5
tools: Read, Grep, Glob, Bash
isolation: worktree
permissionMode: plan
---
Sen bir sıfır bağlam kod denetçisisin. Verilen dosyaları sadece
kendilerine bakarak incele; bağlam, mimari, önceki konuşmalar yok.
Her hata için DOSYA:SATIR referansı ver.
```
Sonra oturum içinde bu ajanı görevlendir.

**Yöntem 3: Çapraz model (en güçlü):** Claude yazsın, GPT-5/Codex/Gemini incelesin (veya tersi).

### 4.4 CLAUDE.md Derinlemesi (Gelişmiş Şablon)

L2 seviyesinde CLAUDE.md dosyanız zenginleşir ama **200 satırı geçerse** bölmeye başlayın:

```markdown
# [Proje Adı] — Çalışma Rehberi

## Mimari Genel Bakış
[1-paragraf açıklama + akış diyagramı metni]

### Veri Akışı
1. İstek → Next.js middleware → App Router
2. Server Action → Prisma → PostgreSQL
3. Asenkron: BullMQ → Redis worker

## Dizin Yapısı
(apps/web, apps/api, packages/... standart şablon)

## Önemli Kararlar (ADR)
- ADR-001: PostgreSQL (MongoDB yerine)
- ADR-002: App Router (Pages yerine)
- ADR-003: BullMQ (Kafka yerine)

## Sık Kullanılan Komutlar
(pnpm install/dev/build/test/lint/typecheck/db:migrate)

## Çalışma Kuralları (Katı)
1. Migration dosyasını elle düzenleme, prisma migrate dev kullan
2. pnpm-lock.yaml elle dokunma
3. Yeni paket eklerken neden bu ve neden alternatif değil açıkla
4. Önce props interface
5. Her endpoint: auth + zod validation + error handling
6. Test dosyası [isim].test.ts şeklinde aynı dizinde
7. Büyük değişikliklerden önce mevcut testler geçmeli

## Bilinen Tuzaklar
- Server component'te useState kullanma → "use client"
- Prisma Decimal toString() ile kullanılmalı
- Redis connection yeniden başlatmada 5 sn gecikme olabilir

## İletişim
- Her önemli adımda 1 cümle özet
- Hata alırsan tam hata mesajı
- 100+ satırlık değişiklikten önce haber ver
```

> 200 satırı geçiyorsa talimatları `.claude/rules/` altına dosya başına bölün (ör: `rules/code-style.md`, `rules/testing.md`). Claude ana talimatını kısa tutar; gerektiğinde alt dosyaları çeker.

### 4.5 Uzantı Katmanları: İlk Skills, Hooks ve MCP

Claude Code'un gücü beş katmandan gelir. Bu fazda hepsinden **birer tane** kurun:

#### Skill (yeniden kullanılabilir prosedür)

Özel komutlar `/project:name` olarak görünen markdown dosyalarıdır. Yeni sistemde `commands/` yerine `skills/<name>/SKILL.md` kullanılması önerilir (eskisi `commands/` otomatik çalışmaya devam eder).

**`.claude/skills/deploy/SKILL.md`:**
```markdown
---
name: deploy
description: Staging deploy checklist'ini çalıştırır.
---
Deploy checklist:
1. `pnpm test && pnpm lint && pnpm typecheck` çalıştır
2. `pnpm build` başarılı mı?
3. Migration varsa veritabanı yedeğini al
4. Version/tag oluştur: `pnpm version patch`
5. Push et ve CI'yi bekle
6. Slack kanalına 1 satırlık özet at
```

Çağırma: `/deploy` veya ilgili gördüğünde Claude otomatik tetikler.

#### Hook (deterministik otomasyon ve güvenlik)

En kullanışlı beş hook olayı: `PreToolUse`, `PostToolUse`, `UserPromptSubmit`, `SessionStart`, `Stop`.

**İlk hook'unuz: dosya kaydedildikten sonra otomatik format**
`.claude/settings.json` ekleyin:
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write|MultiEdit",
        "hooks": [{
          "type": "command",
          "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/format-on-edit.sh"
        }]
      }
    ]
  }
}
```
`.claude/hooks/format-on-edit.sh`:
```bash
#!/bin/bash
# Sadece Edit/Write sonrası ilgili dosyayı formatla
file=$(jq -r '.tool_input.file_path // empty')
[ -n "$file" ] || exit 0
case "$file" in
  *.ts|*.tsx|*.js|*.jsx) npx --no-install prettier --write "$file" 2>/dev/null ;;
  *.rs) rustfmt "$file" 2>/dev/null ;;
esac
exit 0
```

#### MCP (Dış Araç Bağlantısı)

İlk hafta sadece **1-2 MCP** bağlayın:
```bash
# GitHub resmi MCP
claude mcp add github --transport http https://api.github.com --scope user
# Playwright MCP (browser otomasyonu)
claude mcp add playwright --transport stdio npx @playwright/mcp@latest
```
Artık Claude commit oluşturabilir, PR açabilir, browser'da test edebilir.

### 4.6 Hafıza Sistemi (L2)

Otomatik hafıza: Claude Code kendine `~/.claude/projects/<hash>/memory/MEMORY.md` dosyasında not tutar.

Proje içi not defteri:
```
.claude/
├── memories.md
└── sessions/
    ├── 2026-08-01-auth-refactor.md
    └── 2026-08-05-payment-bug.md
```

`memories.md` örneği:
```markdown
# Proje Hafıza — Kalıcı Dersler

## Çözülen Tuzaklar
- Prisma $transaction 5 sn timeout yapıyor → connection_limit≥10
- Next.js 15 cache manuel revalidate etmezsen stale
- Redis Cluster tek node'a bağlanmıyor, bütün seed'ler lazım

## ADR Özeti
- ADR-004: BullMQ (priority + retry iyi)
- ADR-005: next-intl (App Router uyumlu)

## Claude ile Yaşananlar
- Büyük migration dosyalarında yanlış yapıyor → küçük ve izole
- Şema değişikliği sonrası prisma generate hatırlatması ver
```

### 4.7 `/effort` Kullanımı

| Seviye | Model önerisi | En uygun görev |
|:---|:---|:---|
| low | Haiku/Sonnet | Arama, kısa cevap, dosya okuma, basit düzeltme |
| medium | Sonnet 5 | Çoğu geliştirme görevi (varsayılan) |
| high | Opus 5 | Karmaşık refactor, bug kovalama |
| xhigh | Opus 5 | Mimari karar, çoklu dosya değişikliği |
| max | Fable 5 | Sınır durumları, en zor problemler |

> `--effort` veya `/effort` kullanmak gereksiz overthinking'i ve token'ı keser.

> **L2 Kontrol Noktası:**
> - [ ] tmux'ta kalıcı oturum disiplini
> - [ ] CLAUDE.md + en az birer skill/hook/MCP
> - [ ] Context yönetimi (compact/clear) alışkanlık
> - [ ] Zero-context review kullanılıyor
> - [ ] `/effort` ve `/usage` ile bilinçli model seçimi
> - [ ] Bireysel verim artışı hissediliyor (3-5x)

---
