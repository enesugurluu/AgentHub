## 8. Context ve Hafıza Mimarisi: Graf Mühendisliği

### 8.1 3-Katmanlı Hafıza Mimarisi (Kanıtlanmış Desen)

| Katman | Teknoloji | Erişim Hızı | Maliyet | İçerik |
|:---|:---|:---|:---|:---|
| **Hot (Anlık)** | Claude dahili context window + CLAUDE.md | Çok hızlı | Token başına | Aktif görev, dosya içerikleri, açık talimatlar |
| **Warm (Oturum Arası)** | Obsidian/QMD markdown notları, `.claude/memories.md` | Hızlı (arama) | Sıfır (disk) | ADR'ler, alınan dersler, proje conventions, sık kullanılan desenler |
| **Cold (Kurumsal)** | Knowledge Graph (SQLite/Neo4j/Neptune) + SQLite FTS5 | Orta | Düşük | Karar ilişkileri, incident geçmişi, entity ilişkileri, "neden" sorularının cevapları |

### 8.2 Context Penceresi Yönetimi

Claude Sonnet'in context penceresi yaklaşık 200k token olsa bile **pratikte optimum çalışma aralığı 50-80k token**dır. Bunun üstünde "dumb zone" riski ciddi şekilde artar.

**Token Bütçesi Hesabı:**
```
1 token ≈ 4 karakter (İngilizce) ≈ 3-4 kelime
50k token ≈ 37.500 kelime ≈ 150-200 sayfa kitap
80k token ≈ üst sınır, bundan sonra dikkat!
```

**/cost kontrolü:**
Claude Code içindeyken düzenli olarak `/cost` yazarak tüketimi kontrol edin. 50k sınırına yaklaştığınızda `/compact` veya gerekirse `/clear` planlayın.

### 8.3 Bağlam Zehirlenmesi (Context Poisoning) Temizleme

Bir oturumda yanlış bilgiler defalarca işlenmeye başlandığında Claude bu yanlış bilgiyi "doğru" kabul eder ve düzeltmekte zorlanır.

**Zehirlenme Belirtileri:**
- Sizin düzeltmenize rağmen aynı yanlışı tekrar yapıyor
- Daha önce kabul ettiği bir gerçeği şimdi reddediyor
- "Bunu zaten düzelttim" diyor ama düzeltmemiş
- Kodda bir hata zinciri oluşuyor: düzelttiği yer başka yer bozuyor

**Temizleme Prosedürü:**
```
1. Claude'a mevcut durumu özetletin:
   "Şu ana kadar ne yaptığımızı, hangi hataları gördüğümüzü ve ne durumda
    olduğumuzu 10 maddede bir PROGRESS.md dosyasına yaz."

2. Hatalı hipotezleri açıkça reddedin:
   "X yaklaşımı yanlış, bunu tamamen bırakıyoruz. Y hipotezini test edeceğiz.
    Önceki tartışmalardaki X ile ilgili her şeyi unut."

3. Gerekirse /clear ile temiz oturum:
   /clear

4. Yeni oturumda PROGRESS.md ile başlayın:
   "PROGRESS.md'yi oku. X yaklaşımı yanlıştı ve atıldı. Y yaklaşımı ile devam ediyoruz."
```

### 8.4 Hafıza Tehlikeleri (Memory Hazards) ve Önlemler

Üç klasik hafıza hatası ve mimari önlemleri:

| Tehlike | Belirti | Önlem |
|:---|:---|:---|
| **Duplicate Identity** | Farklı isimli ama aynı varlık (örn: "auth-service", "auth_srv", "authentication") mükerrer kayıt | Entity resolution için merge fonksiyonu; canonical name kuralı |
| **Stale Commitment** | Eski önceliğin/kararın artık geçerli olmadığı halde yeni kararları etkilemesi | Her karara `effective_date` ve `superseded_by` alanı ekle |
| **Contradictory Decisions** | ADR-003'te "Redis kullan" derken ADR-007'de "Redis kaldırılıyor" olması | Grafta `supersedes` kenarı ile açıkça işaretle; eski kaydı silme, yerine "yerine şu geçti" bağlantısı koy |

---
