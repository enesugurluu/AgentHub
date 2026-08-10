## 2. Olgunluk Modeli: L0'dan L4'e Yolculuk

Kuruluma geçmeden önce, **nerede olduğunuzu** ve **nereye gittiğinizi** netleştirmek için 5 seviyeli bir olgunluk modeli tanımlıyoruz:

### SEVİYE 0 (L0): Vibe Coder
- Claude Code'u rastgele prompt'larla kullanır
- Proje bağlamını her seferinde sıfırdan açıklar
- Yazılan kodu tamamen manuel okur ve test eder
- Hiçbir otomasyon yok
- **Belirti:** "Bugün Claude çok iyi/kötü yazıyor" ruh haline bağımlılık

### SEVİYE 1 (L1): Bilinçli Kullanıcı
- CLAUDE.md dosyası ile proje bağlamını kalıcılaştırır
- Temel slash komutlarını kullanır (`/clear`, `/compact`, `/cost`)
- Basit görevleri (refactor, test yazma) yüksek başarıyla yaptırır
- Manuel doğrulama yapar ama sistematiktir
- **Gelinen nokta:** Bireysel verimlilik artışı ~2-3x

### SEVİYE 2 (L2): Sistem Operatörü
- Multi-agent doğrulama kurar (Generator + Zero-Context Reviewer)
- Memory sistemi aktif (Hot/Warm/Cold katmanlar)
- Git worktree ile paralel iş akışı
- Checkpoint ve context reset disiplini
- Claude Code'u IDE'ye tam entegre etmiştir
- **Gelinen nokta:** Bireysel verimlilik ~5-10x

### SEVİYE 3 (L3): Takım Mimarı / Tech Lead
- Organizasyon genelinde CLAUDE.md şablonları ve standartları
- Handoff mimarisi: Claude Code (strateji) ↔ Codex/DeepSeek (boilerplate)
- CI/CD içinde otomatik ajan doğrulaması
- Fleet ops: tmux + Tailscale ile 24/7 çalışan ajan filosu
- Maliyet optimizasyonu ve izleme dashboard'u
- **Gelinen nokta:** Takım verimliliği ~10-20x

### SEVİYE 4 (L4): Senior Lead / Otonom Fabrika Operatörü
- Knowledge Graph tabanlı kalıcı hafıza (GraphRAG)
- Sentry/Linear entegrasyonu ile otonom bug triage ve PR
- Closed-loop self-improvement: Hermes benzeri beceri öğrenme
- Multi-model orkestrasyonu (Claude + GPT + DeepSeek her iş için doğru rolde)
- Güvenlik politikaları ve guardrails ile tam otonom operasyon
- Ölçüm, eval ve A/B test altyapısı
- **Gelinen nokta:** Organizasyonel çarpan ~30-50x

> **Bu raporun hedefi:** Sizi L0'dan L4'e çıkarmak için somut, adım adım, uygulanabilir bir plan sunmak.

---
