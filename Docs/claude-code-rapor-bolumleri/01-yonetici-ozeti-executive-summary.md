## 1. Yönetici Özeti (Executive Summary)

### 1.1 Temel Tez

Claude Code (ve benzeri agentik CLI araçları) sadece kod üreten bir yardımcıdan ibaret değildir. Bu araç, **yazılım üretim ekonomisinde paradigmik bir kaymayı** temsil eder:

> **Eski Paradigma:** İnsan kod yazar, AI yardım eder.
> **Yeni Paradigma:** İnsan **sistem operatörü**, AI agent'lar üretim yapar.

Bu raporun iddiası şudur: Bir senior lead'in gerçek kaldıracı, kullandığı modelin benchmark skorunda değil, **o modelin etrafına ördüğü deterministik sistemler**de yatar. (Ref: Mevcut kılavuzlarınızın "Calibration Cost" tezi.)

### 1.2 Ne Kazanacaksınız?

Bu kurulumu tamamladığınızda hedef çıktılar:

| Metrik | Başlangıç (L0) | Hedef (L4) |
|:---|:---|:---|
| Satır kod/saat (insan dokunuşuyla) | ~50-100 | ~2000-5000 |
| Hata ayıklama süresi (ortalama bug) | 2-8 saat | 5-30 dk (otonom triage) |
| PR review döngüsü | 1-3 gün | 1-4 saat |
| İnfrastruktur kurulum çabası | 1-2 hafta | 1-2 saat (ajanla) |
| 24/7 üretim kapasitesi | 0 | Sürekli (fleet ops ile) |

### 1.3 Yanlış Kanıdan Doğruya

| Yaygın Yanılgı | Gerçeklik |
|:---|:---|
| "En güçlü model = en iyi sonuç" | Kalibrasyon maliyeti, model farkından büyük. Model sadakati > model yarışı. |
| "Prompt yazmak yeterli" | Prompt mühendisliği %10; sistem mimarisi, doğrulama ve hafıza %90. |
| "AI ile daha az iş yaparım" | İşin türü değişir: kod yazmaktan → doğrulama ve orkestrasyona. |
| "Ajanlar otonom çalışabilir" | İnsan gözetimi (human-in-the-loop) olmadan ajanlar "dumb zone"da kanser gibi kod yayar. |

---
