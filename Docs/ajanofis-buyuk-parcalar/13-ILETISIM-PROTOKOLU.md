## 13. İletişim Protokolü

AjanŞirket, ajanlar arası iletişim ve kullanıcı-ajan iletişimi için iki açık standardın birleşimini kullanır: **MCP** (dikey, araç erişimi, bkz. Bölüm 11) ve **A2A** (yatay, ajanlar arası). İkisi de JSON-RPC 2.0 tabanlıdır.

### 13.1 Kullanıcı ↔ CEO

Kullanıcı kanban kartı, ofis katı, terminal veya doğrudan sohbet üzerinden CEO'ya talimat verir. CEO bu girdiyi yorumlayıp görev dağıtımı yapar. Acil müdahalelerde (durdur, izin ver, yön değiştir) tüm ajanlara yayın event'i gönderilebilir.

### 13.2 CEO ↔ Çalışan (A2A Task Lifecycle)

Google A2A protokolü v1.0 (2025'te duyuruldu, 2026'da endüstri standardı) 6 durumlu görev yaşam döngüsünü tanımlar. AjanŞirket birebir bunu uygular:

```
┌────────────┐
│ submitted  │ ← CEO görevi atar
└─────┬──────┘
      ▼
┌────────────┐
│  working   │ ← ajan çalışıyor, periyodik artifact/ilerleme event'leri
└─────┬──────┘
      ├───────────────►┌──────────────┐
      │                │ input-required│ ← ajan onay/ek bilgi istiyor
      │                └──────┬───────┘
      │                       │ CEO (veya kullanıcı) yanıtlar
      └───────────────────────┘
      ▼
┌────────────┐     ┌────────────┐
│ completed  │     │   failed   │
└─────┬──────┘     └────────────┘
      │
      ▼
┌────────────┐
│  canceled  │ ← kullanıcı/CEO iptal ederse
└────────────┘
```

#### Görev Devir (Handoff) Doküman Yapısı

CEO bir çalışana görev devrettiğinde standart bir `AGENT_TASK.md` dosyası worktree içine yazılır:

```markdown
# Task: Payment retry mekanizması
**From:** ceo
**To:** backend-dev-1
**Task ID:** task_01J8X...
**Parent Task:** epic_003
**Priority:** P1
**Budget:** $3.00 (max)
**Deadline:** 2026-08-12 18:00
**Created:** 2026-08-09 10:32

## Context
- İlgili dosyalar: `apps/api/src/payment/`, `packages/shared/src/payment/`
- İlgili hafıza notları: [[adr/004-retry-policy]], [[incidents/incident-142-redis-oom]]
- Bağlı kartlar: #142, #147
- Önceki tamamlanmış işler: feat/auth (bkz. worktree backend-auth)

## Task
Exponential backoff ile retry mekanizması kur. 3 deneme, 2^n saniye gecikme,
jitter ile. Dead-letter queue'ya düşenleri logla.

## Acceptance Criteria
- [ ] Tüm testler geçiyor: `pnpm test src/payment`
- [ ] 3 deneme senaryosu unit testlerle doğrulanmış
- [ ] Rate limit durumunda da doğru davranış
- [ ] `pnpm lint` temiz
- [ ] Mevcut payment flow'u bozmayan değişiklik

## Constraints
- Yeni dış paket ekleme
- Redis kullanımı mevcut connection üzerinden olmalı
- Migration gerekiyorsa dur ve bildir
- Maksimum 500 satırlık değişiklik
- Kaba kuvvetle sorunu çözme; mimariye uygun ol

## Expected Output
- Gerekli kod değişiklikleri
- Yeni test dosyaları
- En sonda `RESULT.md` ile özet, dosya listesi, test çıktısı
```

Bu yapı A2A Task mesajının `context/artifacts/expected-output` alanlarına denk gelir; markdown dosya olarak da saklanır (inspect/replay kolaylığı için).

#### Çalışan → CEO Geri Bildirim

Çalışan görevi tamamladığında veya tıkandığında CEO'ya bir artifact gönderir:
- **Tamamlanma:** `RESULT.md` + commit hash(ler) + test çıktısı + maliyet raporu
- **Tıkanma:** `BLOCKED.md` + neyin eksik/engellendiği + önerilen sonraki adım + ek kaynağa ihtiyaç var mı
- **İlerleme:** Her 30 dakikada bir veya her turda progress event (isteğe bağlı); bu event ofis katında "konuşma balonu" olarak ajan masasının üzerinde kısa bir özet olarak görünür.

### 13.3 Onay Akışı (Human-in-the-loop)

Ajan izin istediğinde (Bölüm 15.2'deki onay-gerektiren-eylemler listesi):
1. Süreç duraklatılır
2. PTY çıktısı üzerinden regex/JSON ile onay isteği çözümlenir
3. UI üzerinde mavi bildirim + masa üzerinde zil ikonu çıkar
4. Kullanıcı dört seçenekten birini seçer:
   - **Allow once:** tek seferlik izin
   - **Allow always this pattern:** kalıcı kural ekle (regex ile)
   - **Deny:** reddet
   - **Edit command:** komutu düzenleyip tekrar çalıştır
5. Cevap PTY'ye uygun tuş vuruşu/yanıt olarak geri gönderilir
6. Ajan çalışmaya devam eder
7. 10 dakika onay gelmezse ajan `blocked` durumuna alınır (token israfı önlenir)

### 13.4 Çalışan ↔ Çalışan İletişimi (Debate/Toplantı)

Toplantı odasında bir araya gelen ajanlar aynı task bağlamında birbirlerinin çıktılarına yanıt verir:
- Her katılımcının ayrı "konuşma sırası" olur
- Sırayla birbirlerinin cevaplarına yanıt verirler (Mixture-of-Agents paterni)
- Toplantı odasında ajanlar aynı iş dizini üzerinde çalışmaz; her birinin kendi worktree'si olur, fakat ortak bir `MEETING.md` dosyası üzerinden tartışma kaydı tutulur
- Sonunda toplantı tutanağı + karar özeti CEO'ya iletilir; CEO son kararı verir veya insan onayına sunar

### 13.5 Memory Keeper İletişimi

Memory Keeper gecelik çalıştığında (Bölüm 9.6):
- Event bus'tan son 24 saat'in tüm `completed` ve `failed` event'lerini çeker
- JSONL episodic logları tarar
- Çıkarımlarını `memory-suggestions-{date}.md` dosyası olarak önerir
- Sabah kullanıcıya bildirim: "7 yeni not önerisi, 3 ilişki önerisi — onaylıyor musunuz?"
- Onay sonrası vault'a yazılır ve SQLite graf indeksi güncellenir.

---
