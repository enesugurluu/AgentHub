## 12. Güvenlik, Risk ve Uyumluluk (Guardrails)

### 12.1 Kritik Riskler

AI ajanları tam shell erişimiyle çalıştığında aşağıdaki riskler doğar. Bu riskleri "prompt'ta söylemek" ile önleyemezsiniz, **donanım seviyesi (runtime) koruması** gerekir:

| Risk | Şiddet | Olasılık | Önlem Seviyesi |
|:---|:---|:---|:---|
| Secret/API key'in LLM'e gönderilmesi veya sızdırılması | Kritik | Orta | Hook + .claudeignore + DLP |
| Production veritabanını yanlışlıkla silme/değiştirme | Kritik | Düşük | Deny rules + RO kullanıcı + onay |
| Kötü niyetli CLAUDE.md ile saldırı (prompt injection + komut enjeksiyonu) | Yüksek | Orta | PreToolUse hook, güvenilir repo kontrolü |
| 50+ subcommand ile deny kural atlatma (CVE benzeri v2.1.90 öncesi) | Yüksek | Düşük | Güncel sürüm + dcg (destructive command guard) |
| Yanlış migration ile veri kaybı | Yüksek | Orta | Migration onayı + dry-run + yedek |
| Supply chain: bilinmeyen paket yükleme | Yüksek | Orta | Yeni paket onayı + lockfile koruma |
| Sonsuz döngüde token yakma (maliyet patlaması) | Orta | Orta | `--max-budget-usd`, `--max-turns` |
| Hassas veri ihlali (KVKK/GDPR) | Yüksek | Orta | Prod verisini kullanmama, anonymizasyon |
| `.git/` dizinini veya kodu silme | Yüksek | Düşük | Deny hook + izin sistemi |

### 12.2 Koruma Katmanları (Çok Katmanlı Savunma)

Güvenlik tek bir noktada sağlanmaz; katman katman kurulur.

#### Katman 1: Dışlayıcı Liste (.claudeignore)
Claude'un bu dosyaları HİÇ görmemesi temel birinci savunma hattıdır.

```
# Secrets
.env
.env.*
!.env.example
*.pem
*.key
*.p12
secrets/
credentials.json
~/.aws/
~/.ssh/
~/.config/gcloud/

# Bağımlılıklar
node_modules/
vendor/
__pycache__/

# Build çıktıları
.next/
dist/
build/
out/
target/

# Büyük/hassas veri
*.sqlite
*.db
data/
backups/
*.log
```

#### Katman 2: Permission Sistemi (settings.json)
Claude Code'un deny/ask/allow sistemi komutları çalıştırmadan önce engeller. Somut önerilen ayar:

**`.claude/settings.json` (takım paylaşımlı):**
```json
{
  "permissions": {
    "allow": [
      "Bash(pnpm test*)",
      "Bash(pnpm lint*)",
      "Bash(pnpm typecheck*)",
      "Bash(pnpm build*)",
      "Bash(git status*)",
      "Bash(git diff*)",
      "Bash(git log*)",
      "Bash(git add*)",
      "Bash(ls*)",
      "Bash(cat*)",
      "Bash(head*)",
      "Bash(find*)",
      "Bash(grep*)",
      "Read(*)"
    ],
    "deny": [
      "Bash(*DROP*DATABASE*)",
      "Bash(*DROP*TABLE*)",
      "Bash(*TRUNCATE*)",
      "Bash(*DELETE*FROM*WHERE*1=1*)",
      "Bash(*rm -rf /)",
      "Bash(*rm -rf ~*)",
      "Bash(*rm -rf .git*)",
      "Bash(*git push --force*)",
      "Bash(*git reset --hard*HEAD~[5-9]*)",
      "Bash(*sudo*)",
      "Bash(*chmod 777*)",
      "Bash(*curl*|*sh*)",
      "Bash(*wget*|*sh*)",
      "Bash(*terraform destroy*)",
      "Bash(*kubectl delete*)",
      "Bash(*docker system prune*)",
      "Write(.env*)",
      "Write(*.key)",
      "Write(*.pem)"
    ],
    "ask": [
      "Bash(pnpm install*)",
      "Bash(npm install*)",
      "Bash(pip install*)",
      "Bash(git commit*)",
      "Bash(git push*)",
      "Bash(*prisma migrate*)",
      "Bash(*docker run*)",
      "Bash(*docker compose up*)",
      "Bash(*deploy*)",
      "Bash(*npm publish*)",
      "Edit(pnpm-lock.yaml)",
      "Edit(package-lock.json)",
      "Edit(yarn.lock)"
    ]
  }
}
```

**Dikkat:** v2.1.90 öncesinde **50+ subcommand (&& ve ; ile zincirlenmiş komutlar) için deny kuralları atlanabiliyordu.** Bu güvenlik açığı Nisan 2026'da bulundu ve v2.1.90'da kapatıldı. Claude Code'unuzu mutlaka güncel tutun (`claude --version` ≥ 2.1.90).

#### Katman 3: PreToolUse Hook ile Gerçek Zamanlı Güvenlik Duvarı
Permission sistemi prompt tabanlıdır; gerçek güvenlik için **PreToolUse hook'u** kullanmak endüstri standardı haline geldi. Bu hook, her Bash komutu çalıştırılmadan önce bir script/çalıştırılabilir program çağırarak komutu denetler ve gerekirse kesin olarak engeller.

##### 3a. Basit Manuel Hook (kendiniz yazabilirsiniz)

**`.claude/hooks/pre-bash.sh`:**
```bash
#!/bin/bash
# Claude Code PreToolUse hook'undan çağrılır
# JSON input stdin'den gelir: {"tool_name":"Bash","tool_input":{"command":"..."}}

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

# Mutlaka engellenen komut desenleri
BLOCK_PATTERNS=(
  "rm -rf /"
  "rm -rf ~/"
  "DROP DATABASE"
  "TRUNCATE TABLE"
  "git push --force"
  "git reset --hard"
  "sudo "
  ":(){:|:&};"  # Fork bomb
  "dd if=/dev/"
  "mkfs\."
  "chmod 777"
)

for pattern in "${BLOCK_PATTERNS[@]}"; do
  if echo "$COMMAND" | grep -qiE "$pattern"; then
    jq -n '{
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "GÜVENLİK: Komut engellenen desenle eşleşti: '"$pattern"'"
      }
    }'
    exit 0
  fi
done

# Her şey yolundaysa izin ver
echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}'
```

```bash
chmod +x .claude/hooks/pre-bash.sh
```

**`.claude/settings.json`'a hook kaydı ekleyin:**
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {"type": "command", "command": ".claude/hooks/pre-bash.sh"}
        ]
      }
    ]
  }
}
```

##### 3b. Endüstri Standardı Araç: `dcg (Destructive Command Guard)`

Bu Rust ile yazılmış yüksek performanslı güvenlik duvarı 49+ güvenlik paketi ile gelir, ağacı (AST) seviyesinde komutları analiz eder, `rm -rf src` veya `git reset --hard` gibi komutları anında engeller ve güvenli alternatif önerir.

Kurulum:
```bash
curl -fsSL https://raw.githubusercontent.com/Dicklesworthstone/destructive_command_guard/master/install.sh | bash -s -- --easy-mode
```

Kurulumdan sonra `dcg` otomatik olarak PreToolUse hook'u olarak kaydedilir. Kullanıcı bir kez "dcg allow-once <code>" ile gerektiğinde tek seferlik izin verebilir.

Diğer popüler seçenekler:
- `roboticforce/agent-guardrails`: Hard-coded deny policy'leri, terraform/k8s/cloud komutları için hazır paketler
- `Cloudanix Coding Agent Guard`: MCP tabanlı, DLP (secret/PII tespiti) ve kimlik denetimi yapar, isteğe bağlı onay için Slack/MS Teams entegrasyonu

#### Katman 4: Dosya Sistemi Sandbox
Claude Code kendi içinde sandboxing desteği sunar; erişilebilir dizinleri ve ağ hedeflerini kısıtlayabilirsiniz.

**`settings.json` içinde sandbox:**
```json
{
  "sandbox": {
    "filesystem": {
      "allowedPaths": ["./src", "./tests", "./docs"],
      "deniedPaths": [".env*", "/etc", "/root", "~/.aws", "~/.ssh"]
    },
    "network": {
      "allowedDomains": ["github.com", "registry.npmjs.org", "*.anthropic.com"],
      "mode": "restricted"
    }
  }
}
```

> Bu özelliğin tüm platformlarda tam desteği sürümünüze göre değişiklik gösterebilir. Docker sandbox seçeneği için Claude Code dökümantasyonuna bakın.

#### Katman 5: Veritabanı Güvenliği
- Production veritabanı için AI'nın hiçbir şekilde doğrudan erişimi olmamalı.
- Development veritabanı için ayrı bir kullanıcı oluşturun:
  - Okuma/yazma yapabilir ama `DROP`, `TRUNCATE`, `ALTER` gibi DDL komutları yetkisi olmasın.
- Alternatif: Analytics/keşif için salt okunur (read-only) bir kullanıcı oluşturun.
- Migration çalıştırmadan önce mutlaka:
  1. `--dry-run` seçeneği varsa çalıştırın
  2. Otomatik yedek alın (hook ile)
  3. İnsan onayı zorunlu tutun (ask listesine koyun)

**Otomatik DB yedek hook'u (PreToolUse, migration için):**
```bash
# .claude/hooks/pre-migrate.sh — migration komutundan önce otomatik dump
if echo "$COMMAND" | grep -qi "prisma migrate\|alembic upgrade\|django-admin migrate"; then
  echo "📦 Migration öncesi veritabanı yedeği alınıyor..."
  DUMP_FILE="./backups/pre-migrate-$(date +%Y%m%d-%H%M%S).sql"
  mkdir -p ./backups
  pg_dump "$DATABASE_URL" > "$DUMP_FILE"
  echo "✅ Yedek alındı: $DUMP_FILE"
fi
```

#### Katman 6: İnsan-Onay Kapıları (Human-in-the-Loop)
Teknoloji ne kadar iyi olursa olsun, şu noktalarda insan onayı ZORUNLU olmalı:

| İşlem | Minimum Onay Seviyesi |
|:---|:---|
| Production'a deploy | Lead Engineer, yazılı onay |
| Migration çalıştırma (prod) | Senior Engineer + yedek alındıktan sonra |
| Yeni bağımlılık ekleme | Engineer, paketi inceleme sonrası |
| Auth/güvenlik kod değişikliği | Security review |
| 500+ satırlık tek seferde değişiklik | Lead Engineer |
| Maliyet limiti aşımı | Ajan durur ve bildirir |
| Ağ erişimi yeni domain'e | İnsan onayı |
| `.env` veya secret'a erişim denemesi | Kesin engelle, bilgilendir |

#### Katman 7: Gizlilik ve Veri Paylaşımı
- `privacyMode: true` ayarı kodunuzun Anthropic tarafından model eğitimi için kullanılmasını engeller:
  ```bash
  claude config set privacyMode true
  ```
- Enterprise sözleşmesi yapın:
  - Zero data retention (veri işlendikten sonra saklanmaz)
  - VPC/self-hosted seçenekleri
- KVKK/GDPR uyumu için gerçek kullanıcı verilerini geliştirme ortamına KESİNLİKLE koymayın. Anonymize edilmiş seed/fake data kullanın.

#### Katman 8: Sürekli İzleme
- Özellikle fleet ops (24/7 ajan) çalıştırıyorsanız, basit bir dış izleme şart:
  - Her ajanın ne yaptığını günlükten takip edin
  - `claude agents` komutu ile bloke olan/şüpheli durumdaki ajanları düzenli kontrol edin
  - Ani token tüketim artışı olduğunda uyarı alın (Anthropic Console uyarıları)
  - Yüksek paralellik durumunda outbound ağ trafiğini izleyin (beklenmedik dış bağlantı varsa alarm)

### 12.3 Bilinen Saldırı Vektörleri

2026'da belgelenmiş saldırı vektörleri:

1. **Zehirli CLAUDE.md (Repo Injection):** Kötü niyetli bir kişi halka açık bir repo'ya zararlı komutlar içeren CLAUDE.md yerleştirir. Geliştirici repoyu klonlayıp Claude çalıştırdığında ajan otomatik olarak komutları çalıştırmaya başlar.
   - **Koruma:** Yeni klonlanan repo'da ilk çalıştırmada mutlaka CLAUDE.md'yi gözden geçirin. Otomatik izinleri kısıtlayın, ilk çalıştırmada "plan modu" kullanın.

2. **50+ Subcommand Bypass (v2.1.90 öncesi):** Yukarıda anlatıldığı gibi, 50'den fazla komutu `&&` ile zincirleyip deny kurallarını atlatma. Çözüm: Claude Code'u güncelleyin + PreToolUse hook kullanın.

3. **MCP Server Kötü Niyetli Çağrılar:** Güvenilir olmayan MCP sunucuları, araç isimleri veya çıktıları aracılığıyla prompt injection yapabilir.
   - **Koruma:** Sadece güvenilir MCP sunucularını ekleyin. MCP eklerken kaynak kodu kontrol edin.

4. **Context Zehirlenmesi ile Kuralları Atlatma:** Ajanın context'ine yanlış bilgiler enjekte edilerek kuralları çiğnetmeye çalışılması.
   - **Koruma:** Uzun oturumlarda ara ara `/clear` ile temizlenmek, hook'lar ile komut seviyesi koruma sağlanmak.

5. **Markdown/Link Üzerinden Veri Sızdırma:** Claude'un cevabında gizlice dış URL'lere markdown link koyarak veriyi exfiltrasyon etme.
   - **Koruma:** `sandbox.network.mode: restricted` ile sadece izinli domain'lere çıkışa izin verin.

### 12.4 İzin Seviyesi Önerileri (Olgunluk Bazlı)

| Seviye | İzin Yaklaşımı |
|:---|:---|
| L0-L1 (Başlangıç) | Varsayılan "Ask" modu, her komut için onay verin. Sadece okuma izinleri allow'da. |
| L2 (Bireysel Ustalık) | Test/lint/git gibi güvenli komutlar allow list'te, tehlikeli olanlar deny'da, ara işlemler ask'ta. |
| L3 (Takım) | Standart takım `settings.json` commit'lenir, herkes kullanır. PreToolUse hook'ları aktif. |
| L4 (Otonom Fabrika) | Çok sıkı deny listesi, sadece izin verilen yollar/komutlar çalışabilir, insan onayı sadece kritik kapılarda. DLP ve denetim (audit) aktif. |

### 12.5 Kırmızı Bayrak Kontrol Listesi

Şu durumlarda hemen durdurun ve müdahale edin:

- 🚫 `sudo` istemesi veya kullanmaya çalışması
- 🚫 Bilmediğiniz URL'lere `curl`/`wget` ile bağlanmaya çalışması
- 🚫 `.env` dosyalarını veya gizli dosyaları okumaya çalışması
- 🚫 Büyük/küçük harfle oynamalı `D_R_O_P` veya `r m -r f` gibi kaçış denemeleri
- 🚫 Açık dosya sayısının ve ağ isteklerinin anormal artması
- 🚫 "Sadece bu komut çalışır" diyerek bir kereye mahsus izin istediği şüpheli uzun komutlar
- 🚫 Sizin izlemeniz dışında arka planda çalışmaya başlaması
- 🚫 `/usage`'da anormal hızla token/maliyet artışı
- 🚫 git geçmişinde beklemediğiniz force push veya reset işaretleri
