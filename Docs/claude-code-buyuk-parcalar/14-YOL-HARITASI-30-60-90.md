## 14. 30/60/90 Günlük Uygulama Yol Haritası

> Komut ve özellik isimleri Ağustos 2026'ya göre günceldir (native installer, `/usage`, `/effort`, `/skills`, `--bg`, `--worktree`, `claude agents`, Agent Teams, Plugins, MCP).

### İlk 30 Gün: Temel ve Bireysel Ustalık (L0 → L2)

**Hafta 1: Kurulum ve İlk Alışkanlıklar**
- [ ] Native installer ile Claude Code kur: `curl -fsSL https://claude.ai/install.sh | bash` (npm kullanma)
- [ ] `claude --version` ve `claude doctor` temiz çıksın (en az v2.1.90)
- [ ] tmux kur, temel config'i yerleştir
- [ ] İlk proje için kısa bir `CLAUDE.md` yaz (<200 satır)
- [ ] Global `~/.claude/CLAUDE.md` oluştur (iletişim + evrensel prensipler)
- [ ] `.claudeignore` ve temel güvenlik `permissions.deny` kurallarını ayarla
- [ ] Her gün 1-2 saat Claude ile çalış, alışkanlık kazan
- [ ] Her akşam `/usage` ile maliyet kontrolü (eski `/cost`)

**Hafta 2: Context, Memory, Komutlar**
- [ ] `/compact` ve `/clear` disiplini
- [ ] `/effort` kullanımı: göreve göre low/medium/high/xhigh
- [ ] Checkpoint commit pratiği
- [ ] Proje içinde `.claude/memories.md` veya auto-memory kullanımını alışkanlık et
- [ ] Dumb zone belirtilerini tanımayı öğren
- [ ] İlk skill'i yaz: `.claude/skills/<isim>/SKILL.md` (ör: test, review, refactor)
- [ ] İlk hook'u kur: PostToolUse ile format-on-edit

**Hafta 3: MCP, Doğrulama ve Review**
- [ ] 1-2 MCP sunucusu bağla (resmi GitHub MCP + Playwright MCP iyi başlangıç)
- [ ] Zero-context reviewer oturumu: ikinci terminal veya subagent ile
- [ ] Test → kod → review döngüsünü benimse
- [ ] `.claude/agents/reviewer.md` subagent'ı tanımla
- [ ] İlk deterministik güvenlik hook'u: PreToolUse ile tehlikeli komut engeli

**Hafta 4: Worktree ve Paralellik**
- [ ] `claude --worktree <branch>` ile ilk native worktree denemesi
- [ ] 2 basit görevi paralel worktree'de çalıştır
- [ ] `claude --bg "görev"` ile ilk arka plan ajanını dene; `claude agents` ile izle
- [ ] L2 seviyesine ulaş: bireysel verimlilik ~5x

**30 Gün Sonu Hedef:** CLI'yi güvensiz hissetmeden kullanabiliyor, context/maliyet/hata yönetimi disipline oturmuş, en az birer skill/hook/MCP/subagent deneyimlemiş.

### Gün 30-60: Takım ve Otomasyon (L2 → L3)

- [ ] Organizasyon `CLAUDE_TEMPLATE.md` şablonunu oluştur ve takıma aç
- [ ] Handoff mimarisi: stratejik katman (Opus/Fable) → uygulama katmanı (Sonnet/Haiku/Codex) → zero-context review
- [ ] GitHub Actions ile otomatik AI review kur (native installer kullan, `--max-budget-usd` koy)
- [ ] İlk plugin'i (veya organize `.claude/` klasörünü) takım reposuna commit'le
- [ ] VPS üzerinde 24/7 ajan filosu kur (`claude --bg` ile veya Claude Squad ile)
- [ ] Tailscale mesh ağı kur
- [ ] Takım arkadaşlarına 1:1 eğitim ver
- [ ] İlk uzun gecelik görevini başarıyla tamamla
- [ ] `/schedule` ile günlük otomatik rapor denemesi
- [ ] Agent Teams'i aç (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`), 2-3 teammate ile oyna
- [ ] Maliyet takip sistemi kur (Console hard-limit + CSV log)
- [ ] Haftalık AI retro'ları başlat
- [ ] Eval setinin ilk versiyonunu yaz
- [ ] L3 seviyesine ulaş

### Gün 60-90: Otonom Fabrika (L3 → L4)

- [ ] PreToolUse güvenlik hook'unu olgunleştir (veya `destructive_command_guard` kur)
- [ ] Sentry + Linear + otomatik triage hattı
- [ ] Triage cron servisini `claude --bg` ile ayağa kaldır, `claude agents` ile izle
- [ ] Knowledge Graph'i kurmaya başla (SQLite + FTS5 veya Neo4j)
- [ ] 10+ skill/agents kütüphanesi
- [ ] Çoklu model routing (Opus/Sonnet/Haiku/GPT/Codex)
- [ ] Çoklu CLI orkestratör seçimi (Claude Squad / Vibe Kanban / Composio AO)
- [ ] Periyodik eval ölçüm (haftalık skor)
- [ ] İlk tam otonom PR production'a merge olsun
- [ ] Güvenlik politikalarını ve onay seviyelerini yazılı hale getir
- [ ] MCP portföyünü genişlet (Sentry, Postgres, Figma, Linear, Slack)
- [ ] L4 seviyesine ulaş: sen uyurken iş üreten bir fabrika

### 90 Gün Sonrası Sürekli İyileştirme

- Ayda bir yeni model dene (Fable/Opus/Sonnet güncellemeleri geldiğinde)
- A/B test: aynı görev farklı model/prompt ile karşılaştır
- Skill/agents/hooks kütüphanesini büyüt
- Graf hafızasını günlük besle, eval setini güncelle
- Yeni takım üyelerine workflow'u öğret ve plugin dağıt
- Organizasyon genelinde yaygınlaştır
- Gerekirse kendi internal CLI/MCP plugin'lerini yaz
- AAA seviye desktop araçları (AjanŞirket vb.) ile orkestrasyonu görselleştir

---
