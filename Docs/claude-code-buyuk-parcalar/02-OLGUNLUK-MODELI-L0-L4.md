## 2. Olgunluk Modeli: L0'dan L4'e Yolculuk

Kuruluma geçmeden önce **nerede olduğunuzu** ve **nereye gittiğinizi** netleştirmek için 5 seviyeli bir olgunluk modeli. Claude Code'un 2026 uzantı katmanları (CLAUDE.md, Skills, MCP, Hooks, Subagents/Agent Teams/Plugins) seviye ilerledikçe devreye girer.

### SEVİYE 0 (L0): Vibe Coder
- Claude Code'u rastgele prompt'larla kullanır
- Proje bağlamını her seferinde sıfırdan açıklar
- Yazılan kodu tamamen manuel okur ve test eder
- Hiç otomasyon yok
- **Belirti:** "Bugün Claude çok iyi/kötü yazıyor" ruh haline bağımlılık

### SEVİYE 1 (L1): Bilinçli Kullanıcı
- Native installer ile kurulum yapmış (`curl -fsSL https://claude.ai/install.sh | bash`)
- CLAUDE.md ile proje bağlamını kalıcılaştırır
- Temel komutları bilir (`/clear`, `/compact`, `/usage`, `/effort`, `/model`)
- Basit görevleri (refactor, test yazdırma) yüksek başarıyla yaptırır
- Manuel ama sistematik doğrulama yapar
- `.claudeignore` ve temel güvenlik kuralları tanımlı
- **Gelinen nokta:** Bireysel verimlilik ~2-3x

### SEVİYE 2 (L2): Sistem Operatörü
- **İki veya daha fazla uzantı katmanını** kullanır: CLAUDE.md + Skills + bir MCP sunucusu + bir PostToolUse hook'u
- Generator + zero-context reviewer ayrımı (subagent veya ayrı terminal)
- Memory: otomatik hafıza + proje içi notlar (daha sonra graf'a evrilir)
- `claude --worktree` ile paralel iş akışı; `--bg` ile arka plan ajanı denemiş
- Checkpoint, `/compact`, `/clear` disiplini
- IDE + tmux + shell helpers entegrasyonu
- `/usage` ile bilinçli maliyet takibi
- **Gelinen nokta:** Bireysel verimlilik ~5-10x

### SEVİYE 3 (L3): Takım Mimarı / Tech Lead
- Organizasyon genelinde CLAUDE.md şablonları ve ortak `.claude/` klasörü
- Takıma plugin/kurulum paketi dağıtır, standartları belirler
- Handoff mimarisi: yüksek zeka (Opus/Fable) → yüksek hacim (Sonnet/Haiku/Codex) → zero-context review
- MCP portföyü: GitHub, Sentry, Linear, Postgres, Figma, Playwright
- Deterministik güvenlik: PreToolUse hook'ları (veya dcg), `permissions.deny`, v2.1.90+ sürüm takibi
- CI/CD içinde otomatik ajan review (native installer ile)
- Fleet ops: VPS/tmux/Tailscale ile 24/7 ajanlar veya Claude Squad/Vibe Kanban gibi yerel orkestratör
- Agent Teams'i denemiş (env var ile açılır, 3-5 teammate)
- Maliyet monitoring ve Console hard limit
- **Gelinen nokta:** Takım verimliliği ~10-20x

### SEVİYE 4 (L4): Senior Lead / Otonom Fabrika Operatörü
- Knowledge Graph tabanlı kalıcı hafıza (SQLite+FTS5 veya Neo4j), Obsidian-tarzı bidirectional link
- Sentry/Linear/GitHub ile otonom bug triage → worktree → PR pipeline
- Closed-loop self-improvement: beceri (skill) öğrenme döngüsü, periyodik eval
- Çoklu model ve çoklu CLI orkestrasyonu (Claude Code + Codex + Gemini CLI + Aider + OpenCode), port/DB izolasyonu
- Güvenlik politikaları PreToolUse hook'lar ile enforced; plugin/MCP denetimi yapılıyor
- Zamanlanmış görevler (`/schedule`), batch API ile %50 indirimli asenkron işler
- Ölçüm, eval ve A/B test altyapısı; haftalık skorlar
- AAA araçlar (örn. AjanŞirket) ile görsel ofis/kanban/graf yönetimi
- **Gelinen nokta:** Organizasyonel çarpan ~30-50x

> **Bu raporun hedefi:** Sizi L0'dan L4'e çıkarmak için somut, adım adım, uygulanabilir bir plan sunmak. Her faz sonunda o seviyenin kontrol listesini bulacaksınız.

---
