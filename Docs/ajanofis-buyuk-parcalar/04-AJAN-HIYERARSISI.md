## 4. Ajan Hiyerarşisi

### 4.1 Organizasyon Şeması

```
                           ┌─────────────────┐
                           │      CEO        │
                           │ (Orkestratör)   │
                           │ Ana model: Claude│
                           │   Sonnet/Opus   │
                           └────────┬────────┘
                                    │ A2A protocol (ajan↔ajan)
           ┌────────────┬───────────┼───────────┬────────────┐
           │            │           │           │            │
    ┌──────▼─────┐ ┌────▼────┐ ┌───▼───┐ ┌─────▼─────┐ ┌────▼─────┐
    │    CTO     │ │   PM    │ │ QA    │ │ DevOps/   │ │ Designer │
    │ (Mimari)   │ │ Analyst │ │ Eng.  │ │ Infra     │ │ (UI/UX)  │
    └──────┬─────┘ └─────────┘ └───────┘ └───────────┘ └──────────┘
           │
    ┌──────┴──────────────┐
    │                     │
┌───▼─────┐         ┌─────▼─────┐
│ Frontend│         │ Backend   │
│  Dev    │         │ Dev       │
└─────────┘         └───────────┘

   ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
   [Yatay: Memory Keeper ajanı] — tüm ajanların çıktılarını
   epizodik → semantik → prosedürel hafızaya damıtır
```

### 4.2 Yerleşik (Hazır) Roller

Uygulama ilk kurulumda bir "starter company" şablonu ile gelir. Kullanıcı bu rolleri düzenleyebilir/silebilir/yeni ekleyebilir:

| Rol | Açıklama | Varsayılan Motor | Model Seviyesi |
|:---|:---|:---|:---|
| **CEO** | Tüm şirketin orkestratörü. Görevleri alır, böler, dağıtır, çıktıları birleştirir. Her zaman çalışan tek ajan. | Claude Code | **Opus/Fable** (en yüksek zeka) |
| **CTO** | Mimari kararlar, teknoloji seçimi, code review, şema tasarımı. | Claude Code | Opus |
| **PM Analyst** | Gereksinim analizi, acceptance criteria yazar, görevleri yıkar. | Claude/Codex | Sonnet |
| **Backend Dev** | API, veritabanı, iş mantığı kodu yazar. | Claude Code / Codex | Sonnet/Haiku |
| **Frontend Dev** | React/UI kodu, CSS, etkileşim. | Claude Code / Gemini | Sonnet |
| **QA Engineer** | Test yazar, hata avlar, regression çalıştırır. Sıfır bağlam adversarial reviewer. | DeepSeek/Codex/Haiku | Ucuz model |
| **DevOps Engineer** | CI/CD, docker, dağıtım scriptleri, izleme. | Claude Code | Sonnet |
| **Designer** | UI/UX, CSS/Figma, erişilebilirlik. | Claude/Gemini | Sonnet |
| **Memory Keeper** (opsiyonel) | Bilgi grafını günceller, notları düzenler, çelişkileri çözer. | Haiku | Ucuz model |

### 4.3 CEO Agent Davranışı

CEO klasik "süpervizör-worker" mimarisinin orchestrator'ıdır. Ana sorumlulukları:

1. **Görev Alma:** Kullanıcıdan kanbana bırakılan kartları alır.
2. **Görev Analizi:** Kartın gereksinimlerini okur, alt görevlere böler.
3. **Yetkilendirme (Delegasyon):** Alt görevleri en uygun uzman çalışana atar.
4. **Paralel Yönetim:** Bağımsız görevleri paralel çalıştırır.
5. **Bağımlılık Yönetimi:** Bir görev başka birinin çıktısına ihtiyaç duyarsa sıraya koyar.
6. **Kalite Kontrol:** Çıkan işi QA/CTO'ya inceletir.
7. **Sentez:** Çalışanların çıktılarını birleştirir, ana branch'e merge hazırlar.
8. **İlerleme Raporu:** Durumu kullanıcıya ofis katı ve kanban üzerinde yansıtır.

CEO her zaman ana oturumda çalışmaz; o da diğer ajanlar gibi bir worktree içinde çalışır, ama ana branch'e en yakın olan ve koordinasyondan sorumlu olandır.

### 4.4 İletişim: Toplantı Özelliği
Tartışmalı veya karmaşık konularda CEO bir "toplantı" düzenleyebilir; birden fazla ajan aynı konu hakkında sırayla (veya debate/akran inceleme modunda) konuşur. Bu, Mixture-of-Agents tekniğini uygular.

---

