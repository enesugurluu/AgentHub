## 6. İşe Alım ve İşten Çıkarma (Hire/Fire)

Kullanıcı ajan şirketini büyütmek için yeni çalışanlar (ajanlar) "işe alır", gereksiz gördüklerini "işten çıkarır". Bu akış sadece ayar ekranı değil, kullanıcıya metaforu hissettiren bir sihirbaz deneyimi olarak tasarlanır.

### 6.1 İşe Alım (Hire Wizard)

UI: Settings → İşe Al düğmesi → 3 adımlı modal sihirbaz.

**Adım 1: Rol Seçimi**

Kullanıcı iki yoldan birini seçer:
- **Hazır şablondan seç:** CEO, CTO, Backend Dev, Frontend Dev, QA, DevOps, Designer, PM, Researcher, Memory Keeper, Security Reviewer, DB Admin
- **Sıfırdan özel rol tanımla**

**Adım 2: Motor ve Yetenekler**

- Motor seçimi: kurulu ve uyumlu CLI'lar listelenir (Claude Code, Codex, Gemini, OpenCode, Aider, Cline, Kilo, Cursor, Copilot, Qwen...). Kurulu olmayanlar için "Kur" butonu görünür.
- Model seçimi: motor destekliyorsa model katmanı seçtirir (Fable/Opus/Sonnet/Haiku; eşdeğerleri).
- Zeka/Effort seviyesi: low/medium/high/xhigh/max (komut zinciri maliyetini etkiler)
- Bütçe limiti (USD/görev ve USD/ay)
- Max turn limiti
- İzin profili:
  - Full (güvenli kum havuzunda her işlem)
  - Standart (onay gerektiren eylemler izinli)
  - Sınırlı (sadece okuma, test çalıştırma, kod öneri)
  - Custom: kullanıcı izinleri elle ayarlar

**Adım 3: Uzmanlık ve Kişilik**

- Sistem prompt/ajanın ana talimatı (markdown editor)
- Ekli skills: mevcut becerilerden seçer (deploy checklist, debug playbook vb.)
- Ekli MCP araçları: hangi dış araçlara erişebileceği
- Biyografi/isim/ikon (ofis katında görünecek)
- Renk/rozet

"İşe Al" butonuna basınca:
1. Ajan veritabanına kaydedilir
2. Ofis katında uygun boş bir masa konumu atanır
3. (Opsiyonel) Hoş geldin animasyonu
4. Artık kanbana görev bırakılabilir

### 6.2 İşten Çıkarma (Fire)

- Ajanın inspector panelinde "İşten Çıkar" düğmesi
- Onay diyaloğu:
  - "Bu ajanın açık görevlerini ne yapalım?"
  - [x] Görevleri Backlog'a geri al
  - [ ] Görevleri CEO'ya devret
  - [ ] Worktree'yi sil
  - [ ] Konuşma loglarını sakla (varsayılan açık)
- Onay sonrası ajan pasif duruma gelir; isterse kalıcı olarak silinebilir
- Worktree'ler silinmezse kullanıcı elle temizleyebilir

### 6.3 Ön Ayar (Preset) Roller

Uygulama ile birlikte gelen rollerin tanımları (özet):

| Rol | Motor | Effort | İzinler | Skills |
|:---|:---|:---|:---|:---|
| CEO | Claude Opus/Fable | max | Full (ama politikaya tabi) | göreva ayrıştırma, önceliklendirme |
| CTO | Claude Opus | xhigh | Standart | mimari review, teknoloji seçimi |
| PM | Sonnet | high | Sınırlı | gereksinim ayrıştırma, acceptance kriteri |
| Backend | Sonnet/Codex | medium | Standart | API, DB, test |
| Frontend | Sonnet/Gemini | medium | Standart | React, CSS, erişilebilirlik |
| QA | Haiku/Codex | low | Sınırlı | test yazma, regression, kırma denemesi |
| DevOps | Sonnet | high | Standart (+Docker/k8s) | CI/CD, deploy, izleme |
| Designer | Sonnet/Gemini | medium | Sınırlı | Figma, CSS, token, erişilebilirlik |
| Reviewer | Haiku/GPT | low | Sınırlı | zero-context adverserial review |
| Memory Keeper | Haiku | low | Read + vault yazma | indeksleme, ilişki çıkarma |
| Security | Opus | high | Standart | güvenlik denetimi |

Bu preset'ler kullanıcı tarafından düzenlenebilir. Yeni presetler kaydedilebilir ve dışa aktarılabilir (`.agentcompany/agents/*.agent.json`).

### 6.4 Ajanlar Arası Transfer

- Bir ajan üzerinde çalışan görev sürükle bırak ile başka bir ajana devredilebilir.
- Devir sırasında çalışan iş "mola"ya alınır (ajan pasif); süreç geçici olarak duraklatılır; hedef ajan kendi worktree'sinde görevi devralır.
- Geçmiş log ve çıktılar devir teslim dokümanı olarak yeni ajana sunulur.

### 6.5 Takım Şablonları

Kullanıcı belli proje tipleri için hazır "şirket şablonları" kaydedebilir:
- Startup Web App (8 çalışan)
- Backend-only (5 çalışan)
- Solo Geliştirici Asistanı (3 çalışan + CEO)
- Güvenlik Denetim Takımı (4 çalışan)

Bu şablonlar tek tıklamayla kurulur.

---
