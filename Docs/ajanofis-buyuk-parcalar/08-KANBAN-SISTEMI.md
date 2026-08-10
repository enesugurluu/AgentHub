## 8. Kanban Sistemi

### 8.1 Sütunlar
Varsayılan sütunlar, özelleştirilebilir:

| Sütun | Amaç | WIP limiti (varsayılan) |
|:---|:---|:---|
| **Backlog** | Kullanıcının veya CEO'nun eklediği işler | Sınırsız |
| **Analiz (PM)** | PM Analyst'in parçalara ayırdığı, acceptance criteria çıkardığı kartlar | 3 |
| **To Do** | Atama bekleyen, hazır görevler | 8 |
| **In Progress** | Bir çalışana atanmış, aktif iş | Ajan sayısına eşit (1 ajan=1 görev) |
| **Review** | QA/CTO incelemesinde; adversarial/maker-checker | 4 |
| **Done** | Tamamlanmış, merge edilmiş iş | Sınırsız |

WIP limitleri aşıldığında CEO otomatik olarak yeni görev dağıtmaz ve UI'da sarı uyarı gösterir. Swimlane'ler: proje, öncelik (P0-P3), veya ajan bazlı gruplama. Bağımlılık okları (React Flow ile) karta bağlanabilir: "B, A bitmeden başlayamaz".

### 8.2 Kart Yapısı
Her görev kartı şunları içerir:
- `id`, başlık, açıklama, kabul kriterleri
- Atanan ajan(lar)
- Bağlı worktree/branch
- Üst görev (parent task)
- Bağlı bilgi notları (memory graph linkleri)
- Öncelik, etiketler
- Tahmini token/maliyet
- Durum, oluşturma/bitiş zamanları
- Bloklayan/blocklanan görevler

### 8.3 Etkileşim
- Sürükle bırak ile sütunlar arası taşı
- Kartı doğrudan ofisteki bir ajanın masasına bırak → o ajana atanır
- Kartı aç: detaylar, sohbet, çıktı geçmişi, dosyalar
- "CEO'ya ver" → kartı CEO queue'suna bırak, CEO kendi böler/dağıtır
- "Paralel çalıştır" → aynı kart için birden fazla ajan (aynı veya farklı motor) aynı sorunu çözer, en iyi sonucu seç (competitive mode)

### 8.4 CEO Otomasyonu
Kullanıcı kartları sadece Backlog'a bırakır. CEO otomatik:
1. Kartta görev tanımını okur
2. Analiz ve parçalama için PM'e verir (gerekirse)
3. Bağımsız parçaları tespit eder
4. Uygun uzmanlara dağıtır
5. Bitince review zincirine alır
6. Tüm parçalar bittiğinde sentezler

---

