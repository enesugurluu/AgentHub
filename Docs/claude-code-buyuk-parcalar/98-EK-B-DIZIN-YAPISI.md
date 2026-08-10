## EK B: Tavsiye Edilen Dizin Yapısı

> **Son güncelleme:** 2026-08-09 — Claude Code v2.1.x için geçerli yapı.
> Kaynaklar: Anthropic resmi dokümantasyonu, topluluk best-practice, 2026 extension layer referansı.

### B.1 Proje Kökü

```
your-project/
├── CLAUDE.md                         # Takım talimatları (commit'lenir)
├── CLAUDE.local.md                   # Kişisel override'lar (.gitignore)
├── .mcp.json                         # Proje MCP sunucu konfigürasyonu
│
├── .claude/
│   ├── settings.json                 # Takım izin/ayar/hook/MCP (commit'lenir)
│   ├── settings.local.json           # Kişisel override'lar (.gitignore)
│   ├── settings.md                   # Hook/ayar dökümantasyonu (insan için)
│   ├── .gitignore                    # local/ephemeral dosyaları yok say
│   │
│   ├── rules/                        # Modüler talimat dosyaları (*.md)
│   │   ├── code-style.md             # path: frontmatter ile dosya/klasör hedefli
│   │   ├── testing.md
│   │   └── api-conventions.md
│   │
│   ├── skills/                       # Yeniden kullanılabilir prosedürler
│   │   ├── deploy/                   # SKILL.md klasörü (otomatik veya / ile tetiklenir)
│   │   │   └── SKILL.md
│   │   ├── review/
│   │   │   └── SKILL.md
│   │   └── debug-playbook/
│   │       ├── SKILL.md
│   │       └── debug-flowchart.svg
│   │
│   ├── agents/                       # Özel subagent persona'ları
│   │   ├── code-reviewer.md          # YAML frontmatter ile izin/model/memory tanımlı
│   │   ├── security-auditor.md
│   │   └── db-migrator.md
│   │
│   ├── commands/                     # Eski stil slash komutlar (artık skills'e birleşti)
│   │   ├── onboard.md                # /project:onboard olarak görünür
│   │   └── pr-review.md
│   │
│   ├── hooks/                        # Yaşam döngüsü scriptleri
│   │   ├── guard-bash.sh             # PreToolUse: tehlikeli komut engelleme
│   │   ├── block-secrets.py          # Secret/korunan dosya düzenleme engeli
│   │   ├── format-on-edit.sh         # PostToolUse: otomatik prettier/eslint
│   │   └── audit-log.sh              # SessionStart/Stop: loglama
│   │
│   ├── plugins/                      # Kurulu plugin manifest'leri
│   └── worktrees/                    # `claude --worktree` ile otomatik oluşturulan izole worktree'ler
│
├── .claudeignore                     # Claude'un taramaması gereken dosyalar
├── .github/
│   └── workflows/
│       ├── ai-review.yml             # PR'a otomatik Claude incelemesi
│       └── scheduled-quality.yml     # Periyodik kalite kontrol
└── src/ ...
```

### B.2 Global Dizin (`~/.claude/`)

```
~/.claude/
├── CLAUDE.md                         # Global talimatlar (tüm projelerde yüklenir)
├── settings.json                     # Global izinler, varsayılan model
├── settings.local.json               # (kullanılmaz; global zaten kişiseldir)
├── commands/                         # /user:cmd-name olarak görünür
├── skills/                           # Tüm projelerde erişilebilir beceriler
├── agents/                           # Kişisel subagentlar
├── hooks/                            # Global hook'lar
├── plugins/                          # Global pluginler
├── teams/                            # Agent Teams konfigürasyonları (Araştırma önizlemesi)
│   └── <team-name>/config.json
├── tasks/                            # Takım görev listeleri
└── projects/
    └── <project-hash>/
        ├── memory/
        │   └── MEMORY.md             # Otomatik hafıza notları
        └── sessions/                 # Oturum transkriptleri
```

### B.3 Yükleme (Priority) Hiyerarşisi

**Settings birleşim sırası (son kazanır):**
```
1. CLI flags                           (en yüksek — oturum anı)
2. .claude/settings.local.json         (proje kişisel — .gitignore)
3. .claude/settings.json               (proje takım — commit'lenir)
4. ~/.claude/settings.local.json       (global kişisel)
5. ~/.claude/settings.json             (global)
6. Yönetilen Politika (Managed Policy) — (en düşük; IT tarafından MDM/Enterprise, override edilemez)
```

**CLAUDE.md yükleme sırası (en alttakiler üsttekileri biriktirir, sonraki üstte gelir):**
```
1. Managed Policy (CLAUDE.md)          (en düşük)
2. ~/.claude/CLAUDE.md
3. CLAUDE.md (proje kökü veya .claude/)
4. CLAUDE.local.md (proje kökü)        (en yüksek)
```

> **Önemli:** Subagentlar kendi `memory`, `mcpServers`, `hooks` ve `skills` tanımlarına sahip olabilir. Güvenlik nedeniyle **plugin tarafından dağıtılan subagentlar**, plugin'in hook/mcpServers/permissionMode alanlarını yok sayar; bu yetenekler için subagent dosyasının `.claude/agents/` içine elle kopyalanması gerekir.

### B.4 `.claudeignore` Önerisi

```gitignore
node_modules/
.next/
dist/
build/
coverage/
__pycache__/
*.log
.env
.env.*
!.env.example
pnpm-lock.yaml
package-lock.json
yarn.lock
*.min.js
*.min.css
.turbo/
.svelte-kit/
.nuxt/
.output/
target/
.venv/
.mypy_cache/
.ruff_cache/
.pytest_cache/
```

### B.5 Ne Commit'lenir, Ne Ignore Edilir?

**Git'e commit'lenmesi gerekenler (takım paylaşımı):**
- `CLAUDE.md`
- `.claude/settings.json`
- `.claude/rules/`
- `.claude/agents/`
- `.claude/skills/`
- `.claude/commands/`
- `.claude/hooks/`
- `.mcp.json`
- `.claudeignore`

**`.gitignore`'a eklenmesi gerekenler (kişisel/geçici):**
```
.claude/settings.local.json
.claude/projects/
.claude/worktrees/
.claude/plugins/
CLAUDE.local.md
```

### B.6 Karar Verici: Hangi Uzantıyı Ne İçin Kullanmalı?

| İhtiyaç | Kullanılacak katman |
|:---|:---|
| Her turda doğru olması gereken kural | `CLAUDE.md` veya `.claude/rules/` |
| Elle tetiklenen tekrar kullanılabilir prosedür | `skills/` (veya eski `commands/`) |
| Claude'un otomatik karar vermesi gereken iş akışı | `skills/` (auto-trigger frontmatter ile) |
| Mutlaka her seferinde/o olayda çalışması gereken kod | `hooks/` (PreToolUse, PostToolUse, Stop vb.) |
| Ana context'i kirletmeden uzman delegasyon | `agents/` (subagent) |
| Birden fazla ajanın P2P haberleşmesi | Agent Teams (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`) |
| Dış araçlara (GitHub, Sentry, Postgres, Figma) erişim | `.mcp.json` içindeki MCP sunucuları |
| Tüm takımın aynı kurulumu tek komutla alması | Plugin (`.claude/plugins/` veya marketplace) |
| "Bunu asla çalıştırma" seviyesinde güvenlik kuralı | `settings.json` içindeki `permissions.deny` + PreToolUse hook |

---
