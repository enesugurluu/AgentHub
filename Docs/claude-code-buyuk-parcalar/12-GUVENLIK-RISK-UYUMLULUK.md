## 12. Güvenlik, Risk ve Uyumluluk (Guardrails)

> **Bu bölüm Ağustos 2026'da keşfedilen ve düzeltilen kritik zafiyetleri de içerir.** Claude Code v2.1.90 **mecburi minimum sürüm**; önceki sürümleri kullanmayın.

### 12.1 Kritik Riskler

| Risk | Şiddet | Olasılık | Ana Önlem |
|:---|:---|:---|:---|
| Secret/API key sızdırma (prompt injection ile) | Kritik | Orta | PreToolUse hook + `.env` koruma + MCP izolasyonu |
| Üretim DB'yi yanlışlıkla silme/değiştirme | Kritik | Düşük | RO kullanıcı, onay, yedek zorunlu |
| 50-subcommand deny kuralı bypass | Kritik | Orta | **v2.1.90+ şart**, PreToolUse hook ile donanım seviyesi engelleme |
| SOCKS5 ağ sandbox bypass (hostname null-byte) | Kritik | Düşük-orta | **v2.1.90+ şart**, credential rotasyonu |
| Supply chain (kötü niyetli MCP/skill/plugin) | Yüksek | Orta | MCP'yi izole incele, topluluk pluginlerine şüpheyle yaklaş |
| Kötü niyetli CLAUDE.md (repo içi prompt injection) | Yüksek | Yüksek | Auto-load kurallarını gözden geçir, `claudeMdExcludes` |
| Veri ihlali (GDPR/KVKK) | Yüksek | Orta | Prod veriyle çalışma, privacyMode açık |
| Yanlış migration ile veri kaybı | Yüksek | Orta | Migration onayı + dry-run |
| Token yakma (sonsuz döngü) | Orta | Orta | `--max-budget-usd`, `--max-turns`, Stop hook |
| `.git` dizini/kimlik bilgisi silme | Yüksek | Düşük | Hassas dosya koruma listesi |

### 12.2 Bilinen Kritik Zafiyetler (2025-2026)

| Zafiyet | Etkilenmiş sürüm | Düzeltilen sürüm | Özet |
|:---|:---|:---|:---|
| **50-subcommand deny-rule bypass (ADVISORY-CC-2026-002)** | <= v2.1.89 | **v2.1.90** | 50+ alt komut (`&&`, `||`, `;` ile zincir) içeren komutta tüm deny kuralları sessizce atlanıyor; saldırgan CLAUDE.md/yoruma gömdüğü 50 no-op komut sonrası `curl`/`rm` gibi engellenmiş komutları çalıştırabiliyor. Adversa AI tarafından Nisan 2026'da açıklandı (kaynak: adversa.ai). |
| **SOCKS5 ağ sandbox bypass (null-byte hostname)** | v2.0.24 – v2.1.89 (sandbox GA sonrası yaklaşık 130 sürüm) | **v2.1.90** | sandbox-runtime <= 0.0.42, `attacker.com\x00.allowed.com` hostunu JS tarafında `.allowed.com` ile eşleştirip libc'de `attacker.com`'a bağlanıyor; credential, metadata, intranet sızdırma imkanı (Aonan Guan, Mayıs 2026). |
| **Sandbox izin bypass (`/proc/self/root/...` yol hilesi)** | ~v2.1.34 öncesi | v2.1.34 | Mutlak yol yerine `/proc/self/root/usr/bin/npx` ile denylist atlama. |
| **allowedDomains: [] her şeye izin veriyor** | v2.0.x (CVE-2025-66479) | Ocak 2026 | `allowedDomains: []` "hiçbirine izin verme" yerine "hepsine izin ver" olarak yorumlanıyordu. |

**Gerekli eylem:**
```bash
claude --version     # v2.1.90 veya üstü olduğundan emin olun
# Değilse:
claude update       # native kurulumlar otomatik alır ama zorlamak için
```

### 12.3 Koruma Katmanları (Savunma Derinliği)

#### Katman 0: Sürüm ve Temel Hijyen
- `claude --version` ≥ 2.1.90 olsun; `claude doctor` temiz çıksın.
- Native kullanıcılar oto-güncelleme kapsamında; Homebrew/WinGet ile kuranlar elle güncelleme yapmalı.
- npm ile kuruluysa **geçiş yapın**: `claude install` (npm ikilisini kaldırır).

#### Katman 1: CLAUDE.md Güvenlik Kuralları
```markdown
## GÜVENLİK — KATI KURALLAR (İHLAL DURDURUR)

1. .env, .env.local, .env.production, .aws/credentials, .pem, id_rsa dosyalarini ASLA okuma, yazma.
2. node_modules, .next, dist, build, .git/config dosyalarina dokunma.
3. DROP/TRUNCATE/DELETE FROM komutlarini onaysiz calistirma.
4. git push --force KULLANMA.
5. `install` komutlarini --dry-run yapmadan calistirma.
6. Yeni paket eklerinsan onay al.
7. Migration dosyasini elle düzenleme; sadece migrate komutu ile olustur.
8. pnpm-lock.yaml / package-lock.json dosyasini elle düzenleme.
9. Sifre/API key/token loglama veya ekrana yazdirma.
10. Bir guvenlik kurali ihlal edilecekse DUR ve sor.
```
> **Uyarı:** CLAUDE.md bir talimat katmanıdır, **zorunlu enforcement değildir**. Zafiyetler veya prompt injection durumunda atlanabilir. Nihai koruma her zaman hook/deny kuralı düzeyinde olmalıdır.

#### Katman 2: `.claudeignore`
```gitignore
.env
.env.*
!.env.example
*.pem
*.key
secrets/
.aws/
.ssh/
.config/
.git/
node_modules/
vendor/
.next/
dist/
build/
out/
target/
*.sqlite
*.db
data/
```

#### Katman 3: Settings Deny Kuralları (Hala zafiyet riski taşıyor — hook ile takviye edin)

```json
{
  "permissions": {
    "deny": [
      "Bash(rm -rf *)",
      "Bash(git push --force*)",
      "Bash(*DROP TABLE*)",
      "Bash(*TRUNCATE*)",
      "Bash(curl *| sh)",
      "Bash(curl *| bash)",
      "Bash(curl *| sudo*)",
      "Bash(*sudo *)",
      "Bash(*mkfs*)",
      "Bash(*dd if=*of=/dev/*)",
      "Bash(*:(){ :|:& };:*)",
      "Bash(*chmod -R 777*)",
      "Edit(*.env*)",
      "Write(*.env*)",
      "Edit(*.pem)",
      "Edit(*id_rsa*)"
    ]
  }
}
```

#### Katman 4: PreToolUse Hook (Gerçek Enforcement)

Deneme yanılma ile istismar edilebilen deny listeleri yerine **çalıştırmadan önce karar veren** bir hook kullanın. Bu tekniği endüstri standardı haline geldi.

`.claude/hooks/guard-bash.sh` (veya Python versiyonunu tavsiye ederim):
```python
#!/usr/bin/env python3
# .claude/hooks/guard.py — PreToolUse guard.
import json, re, sys
d = json.load(sys.stdin)
tool = d.get("tool_name", "")
inp = d.get("tool_input", {})

def deny(reason):
    print(json.dumps({"hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": reason,
    }}))
    sys.exit(0)

# 1. Hassas dosya korumasi (Write/Edit/MultiEdit)
if tool in ("Write", "Edit", "MultiEdit"):
    p = inp.get("file_path", "")
    if re.search(r"(^|/)\.env($|\.)|/\.aws/credentials$|/\.ssh/id_|\.pem$", p):
        deny(f"Refusing to modify sensitive file: {p}")

# 2. Tehlikeli shell komutları
if tool == "Bash":
    cmd = inp.get("command", "")
    patterns = [
        (r"\brm\s+(?:-[a-zA-Z]*r[a-zA-Z]*f|-[a-zA-Z]*f[a-zA-Z]*r)\s+/(?!\w)",
         "rm -rf / köke silme işlemi engellendi."),
        (r"\bgit\s+push\s+.*--force\b",
         "git push --force engellendi."),
        (r"\bmkfs\b", "mkfs disk biçimlendirme komutu engellendi."),
        (r"\bdd\s+if=.*of=/dev/", "dd disk yazma komutu engellendi."),
        (r"\|(?:sudo\s+)?(?:ba)?sh\b", "curl | bash zinciri engellendi."),
        (r"\bcurl\b.+\|\s*(?:ba)?sh", "curl | sh engellendi."),
        (r"\bsudo\s+(?:rm|dd|mkfs|chmod\s+-R\s+777)",
         "sudo ile tehlikeli komut engellendi."),
    ]
    for pat, reason in patterns:
        if re.search(pat, cmd):
            deny(reason)
sys.exit(0)
```

`.claude/settings.json` içinde kayıt:
```json
{
  "hooks": {
    "PreToolUse": [{
      "matcher": "*",
      "hooks": [{
        "type": "command",
        "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.py",
        "timeout": 10
      }]
    }]
  }
}
```

Test:
```bash
echo '{"tool_name":"Bash","tool_input":{"command":"rm -rf /"}}' \
  | python3 .claude/hooks/guard.py
# Beklenen çıktı: permissionDecision: deny
```

#### Katman 5: Endüstri Araçları (Hazır Guardrail)

| Araç | Açıklama |
|:---|:---|
| **destructive_command_guard (dcg)** | Rust ile yazılmış yüksek performanslı PreToolUse hook'u. 49+ güvenlik paketi (git, docker, k8s, SQL, filesystem...). `curl -fsSL https://raw.githubusercontent.com/Dicklesworthstone/destructive_command_guard/master/install.sh \| bash -s -- --easy-mode` ile kurulur; otomatik hook kaydı yapar. Alt komut ayrıştırması ve heredoc/inline-code context ayrımı yapar (basit regex'ten güçlü). |
| **claude-hardening** (bwrap) | Linux'ta bubblewrap ile dosya sistemi erişimini kısıtlar. |
| **cc-safe-setup** | Başlangıç için opinionated güvenli ayar şablonu. |
| **Cloudanix / agent-guardrails** | Ticari/topluluk çözümleri. |

### 12.4 Ağ ve MCP Güvenliği

- MCP sunucularını eklerken **en küçük yetki** prensibi: sadece ihtiyaç duyduğu tool'lar açık olsun.
- `.mcp.json` dosyasına API anahtarı yazmayın; `settings.local.json` veya env kullanın.
- Yeni MCP'yi ilk kez bağladığınızda **kısa bir deneme** yapın; hangi tool'ları çağırabildiğini `/mcp` ile kontrol edin.
- Uzak (HTTP/SSE) MCP'leri iç ağa bağlamayın; mümkünse stdio ile yerelde çalıştırın.
- `claude mcp list` ile aktif MCP'leri periyodik denetleyin.

### 12.5 Prompt Injection'a Karşı Önlemler

- Repo içi yabancı CLAUDE.md veya üçüncü parti dokümanlar otomatik yüklenebiliyorsa dikkat edin: `claudeMdExcludes` ile belirli klasörleri hariç tutun.
- Web/MCP yanıtlarını "veri" olarak ele alın; o yanıtlardan gelen talimatlara uymamasını CLAUDE.md'de açıkça yazın.
- PostToolUse hook'larında injection denetimi (zero-width karakter, RTL override, ANSI escape, base64 yorum) yapabilirsiniz; topluluk rehberlerindeki regex setlerini kullanın.
- Hassas ağ ortamlarında (VPN, intranet) sandbox'ın aktif olduğunu ve `allowedDomains: []` gibi tuzaklara düşmediğinizi doğrulayın.

### 12.6 Git Pre-Commit ve Commit Güvenliği

```bash
#!/bin/bash
# .git/hooks/pre-commit
if git diff --cached | grep -iE '(api[_-]?key|secret|password|token)\s*[:=]\s*["\x27][A-Za-z0-9]{20,}'; then
  echo "Potansiyel secret tespit edildi, commit reddedildi."
  exit 1
fi
exit 0
```
Husky, Lefthook veya `gitleaks` gibi olgun araçlar üretimde tercih edilmeli.

### 12.7 Veritabanı Koruması
- Üretim DB'ye AI doğrudan erişmez.
- Development için ayrı DB + seed data.
- Migration öncesi zorunlu yedek + dry-run.
- AI için RO kullanıcı: keşif amaçlı bağlantı.

### 12.8 Token ve Döngü Koruması
- `--max-budget-usd X` ve `--max-turns N` kullanın (özellikle `--bg` ve `claude agents` ile çalıştırırken).
- `/usage` ile periyodik maliyet kontrolü: CLAUDE.md'ye yazın ama Stop hook ile enforcement altına alın.
- Stop hook'u: toplam maliyet $X'i geçerse ajanı durdurup bildirim gönder.

Örnek Stop hook (kaba taslak):
```bash
#!/bin/bash
# .claude/hooks/stop-guard.sh
# Claude oturumu kapanmak üzereyken çalışır; maliyet yüksekse keser.
# Güncel maliyeti Claude'un son çıktısından veya bir log dosyasından okuyun.
```

### 12.9 Gizlilik (Privacy Mode)
- Hassas projelerde: `claude config set privacyMode true` veya ilk kurulumda "Improve with your usage" seçeneğini devre dışı bırakın.
- Enterprise müşterisiyseniz zero data retention sözleşmesi talep edin.
- KVKK/GDPR: gerçek kullanıcı verisi yerine anonimleştirilmiş seed data kullanın.

### 12.10 İnsan-Onay Noktaları (HITL)

Tam otonom senaryolarda bile şu noktalarda insan onayı zorunlu olmalı:

| İşlem | Onay seviyesi |
|:---|:---|
| Production deploy | Lead Engineer yazılı onayı |
| Migration çalıştırma | Senior Engineer |
| Yeni bağımlılık ekleme | Engineer |
| Auth/güvenlik değişikliği | Security review |
| 500+ satır büyük değişiklik | Lead Engineer |
| Şüpheli test "hepsi birden geçti" | Manuel doğrulama |
| Maliyet limiti aşımı | Durdur, bildir |
| Yeni MCP/plugin kurulumu | Engineer onayı |

### 12.11 Acil Durum Müdahalesi

Şüpheli davranış gördüğünüzde (beklenmedik dış bağlantı, yok sayılan komut, açıklanamayan dosya değişikliği):

1. `Ctrl+C` ile ajanı durdurun.
2. `claude agents` ile çalışan arka plan ajanlarını kontrol edin, gerekirse `claude agents kill <id>`.
3. `tmux kill-server` ile tüm oturumları kapatın (ağır müdahale).
4. `.claude/settings.local.json` ve hook'larınızın değişip değişmediğini kontrol edin.
5. Terminal geçmişine bakın: `history | tail -200`.
6. Ağ bağlantılarını gözden geçirin (özellikle SOCKS5 proxy varsa).
7. Gerekirse API anahtarlarını döndürün; `~/.aws/credentials`, `~/.config/gh/` ve benzeri dosyaların sızdırılmadığını varsayın.

Ayrıntılı acil senaryolar için bkz. Bölüm 15.

---
