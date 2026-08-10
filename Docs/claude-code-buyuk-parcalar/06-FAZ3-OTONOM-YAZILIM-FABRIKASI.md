## 6. FAZ 3: L3 → L4 — Otonom Yazılım Fabrikası (Ay 3-6)

Bu seviyede kurduğunuz sistem, insan uyanık olmasa da iş üreten, kendi kendine triage yapan, hataları düzelten ve PR açan bir **otonom fabrikaya** dönüşür.

### 6.1 Reaktif Ajanlar: Sentry → Linear → PR Pipeline

Canlıdaki hataları otonom olarak çözen bir hat kurun.

#### Mimari
```
[Canlı Hata Oluşur]
       ↓
[Sentry yakalar]
       ↓ webhook
[Linear'da Bug kartı açılır]
       ↓ cron 10dk'da bir kontrol
[Triage Ajanı çalışır]
       ↓
[İzole worktree + branch açılır]
       ↓
[Claude Code sorunu çözer]
       ↓
[Testleri ve lint'i kendisi çalıştırır]
       ↓
[PR açılır]
       ↓
[Sabah insan gelir ve sadece review yapar]
```

#### Kurulum Özeti

**1. Linear + Sentry Entegrasyonu:**
- Sentry → Integrations → Linear'ı bağlayın
- "New Issue" oluştuğunda Linear'da "Bug" etiketli kart oluşturulsun

**2. Triage Cron Servisi (VPS üzerinde):**

**`/opt/factory/triage.py`:**
```python
#!/usr/bin/env python3
"""
Linear'daki yeni Bug kartlarını tarar ve Claude Code ajanıyla otonom düzeltme başlatır.
"""
import os
import subprocess
import requests
import json
import datetime
from pathlib import Path

LINEAR_API = "https://api.linear.app/graphql"
TOKEN = os.environ["LINEAR_ACCESS_TOKEN"]
REPO_PATH = Path("/opt/factory/repo")
TASKS_PATH = Path("/opt/factory/tasks")

def fetch_new_bugs():
    query = """
    query {
      issues(
        filter: {
          state: { name: { eq: "Todo" } }
          labels: { name: { eq: "Bug" } }
        }
        first: 5
      ) {
        nodes {
          id
          title
          description
          identifier
        }
      }
    }
    """
    r = requests.post(LINEAR_API,
        headers={"Authorization": TOKEN, "Content-Type": "application/json"},
        json={"query": query})
    r.raise_for_status()
    return r.json()["data"]["issues"]["nodes"]

def spawn_agent(bug):
    """Bir bug için izole worktree ve Claude oturumu başlat."""
    safe_name = bug["identifier"].lower().replace("-", "_")
    branch = f"fix/{safe_name}"
    worktree = TASKS_PATH / safe_name

    if worktree.exists():
        return False  # Zaten işleniyor

    TASKS_PATH.mkdir(parents=True, exist_ok=True)
    subprocess.run(["git", "worktree", "add", "-b", branch, str(worktree), "origin/main"],
                   cwd=REPO_PATH, check=True)

    # Görev tanım dosyasını hazırla
    task_md = f"""# Otonom Hata Düzeltme
**Linear ID:** {bug['identifier']}
**Başlık:** {bug['title']}
**Tarih:** {datetime.datetime.now().isoformat()}

## Hata Açıklaması
{bug['description']}

## Talimatlar
1. Önce hatayı anla ve reproduce et (mümkünse test ile)
2. Hatayı düzelt
3. Hatanın tekrar etmemesi için regression testi yaz
4. Tüm testlerin geçtiğini doğrula
5. Değişiklikleri commit et
6. PR açmak için gerekli komutları hazırla

## Kurallar
- Asla API key veya secret kullanma
- Migration gerekiyorsa dur ve bildirim gönder
- 3 denemede çözemiyorsan dur ve rapor yaz
- Sadece hatayı düzelt, ekstra refactor yapma
"""
    (worktree / "TASK.md").write_text(task_md)

    # Native arka plan ajanı ile başlat (tmux'a gerek duymadan;
    # istenirse tmux içinde de çalıştırılabilir)
    cmd = (f"cd {worktree} && claude --bg --worktree {branch} --max-budget-usd 3 "
           f"--max-turns 80 'TASK.md oku ve hatayı düzelt. Bittiğinde beni bilgilendir.' "
           f"> agent.log 2>&1")
    subprocess.run(["bash", "-c", cmd], check=True)

    # Linear'da durumu güncelle (In Progress'e çek)
    move_to_in_progress(bug["id"])
    return True

def move_to_in_progress(issue_id):
    mutation = """
    mutation($id: String!) {
      issueUpdate(id: $id, input: { stateId: "<IN_PROGRESS_STATE_ID>" }) {
        success
      }
    }
    """
    # Not: Gerçek stateId'yi Linear API'den almanız gerek
    requests.post(LINEAR_API,
        headers={"Authorization": TOKEN, "Content-Type": "application/json"},
        json={"query": mutation, "variables": {"id": issue_id}})

def main():
    bugs = fetch_new_bugs()
    if not bugs:
        print("Yeni bug bulunamadı.")
        return
    for bug in bugs:
        print(f"🚀 Ajan tetikleniyor: {bug['identifier']} — {bug['title']}")
        try:
            spawn_agent(bug)
        except Exception as e:
            print(f"❌ Hata: {e}")

if __name__ == "__main__":
    main()
```

**3. Cron kurulumu (VPS üzerinde):**
```bash
# crontab -e
# Her 10 dakikada bir triage çalış
*/10 * * * * cd /opt/factory && /usr/bin/python3 triage.py >> /var/log/factory.log 2>&1
```

### 6.2 Knowledge Graph ile Kurumsal Hafıza (GraphRAG)

L4 seviyesinde basit markdown notları yetmez. Karmaşık ilişkiler için **graf tabanlı** bir hafızaya ihtiyacınız var.

#### Neden Vektör DB (RAG) Yetmez?

Vektör benzerlik araması "Redis deprecate" dokümanını bulabilir ama şu soruyu cevaplayamaz:

> "Redis'i neden deprecated ettik? Hangi incident sonrası ADR-007 yazıldı ve hangi alternatif seçildi?"

Bu bir **multi-hop reasoning** sorusudur ve sadece graf veri yapıları ile deterministik olarak çözülebilir.

#### Basit Graf Kurulumu (SQLite üzerinde)

Ağır bir graph database kurmadan başlangıç için SQLite üzerinde özel bir şema kullanabilirsiniz. Gerçek production için Neo4j veya AWS Neptune önerilir.

**`graph_memory.py`:**
```python
import sqlite3
import json
from datetime import datetime
from pathlib import Path

class GraphMemory:
    """Basit Knowledge Graph — Obsidian vault ile entegre."""

    def __init__(self, db_path: str = "graph_memory.db"):
        self.conn = sqlite3.connect(db_path)
        self._init_schema()

    def _init_schema(self):
        c = self.conn.cursor()
        c.executescript("""
        CREATE TABLE IF NOT EXISTS entities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            type TEXT NOT NULL,  -- 'adr', 'incident', 'service', 'person', 'decision'
            description TEXT,
            metadata TEXT,  -- JSON
            created_at TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS edges (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_id INTEGER NOT NULL REFERENCES entities(id),
            target_id INTEGER NOT NULL REFERENCES entities(id),
            relation TEXT NOT NULL,
            -- Standart kenar tipleri: supersedes, blocks, caused_by, uses, decided_in, mitigates
            evidence TEXT,  -- Kaynak dosya/commit referansı
            created_at TEXT DEFAULT (datetime('now')),
            UNIQUE(source_id, target_id, relation)
        );

        CREATE INDEX IF NOT EXISTS idx_edges_source ON edges(source_id);
        CREATE INDEX IF NOT EXISTS idx_edges_target ON edges(target_id);
        CREATE INDEX IF NOT EXISTS idx_entities_name ON entities(name);
        """)
        self.conn.commit()

    def add_entity(self, name: str, ent_type: str, description: str = "", **metadata):
        c = self.conn.cursor()
        c.execute("""
            INSERT OR IGNORE INTO entities (name, type, description, metadata)
            VALUES (?, ?, ?, ?)
        """, (name, ent_type, description, json.dumps(metadata)))
        self.conn.commit()
        return self.get_entity(name)

    def get_entity(self, name: str):
        c = self.conn.cursor()
        c.execute("SELECT * FROM entities WHERE name = ?", (name,))
        row = c.fetchone()
        if not row:
            return None
        return {"id": row[0], "name": row[1], "type": row[2],
                "description": row[3], "metadata": json.loads(row[4] or "{}")}

    def add_edge(self, source_name: str, relation: str, target_name: str, evidence: str = ""):
        s = self.get_entity(source_name)
        t = self.get_entity(target_name)
        if not s or not t:
            raise ValueError(f"Entity bulunamadı: {source_name} veya {target_name}")
        c = self.conn.cursor()
        c.execute("""
            INSERT OR IGNORE INTO edges (source_id, target_id, relation, evidence)
            VALUES (?, ?, ?, ?)
        """, (s["id"], t["id"], relation, evidence))
        self.conn.commit()

    def trace(self, entity_name: str, depth: int = 3):
        """Bir entity'nin ilişkilerini BFS ile gez."""
        visited = set()
        queue = [(entity_name, 0)]
        results = []
        while queue:
            name, d = queue.pop(0)
            if name in visited or d > depth:
                continue
            visited.add(name)
            ent = self.get_entity(name)
            if not ent:
                continue
            c = self.conn.cursor()
            # Giden kenarlar
            c.execute("""
                SELECT e.relation, t.name, t.type, e.evidence
                FROM edges e JOIN entities t ON e.target_id = t.id
                JOIN entities s ON e.source_id = s.id
                WHERE s.name = ?
            """, (name,))
            for rel, target, ttype, evidence in c.fetchall():
                results.append({
                    "from": name, "relation": rel, "to": target,
                    "target_type": ttype, "evidence": evidence, "depth": d
                })
                queue.append((target, d + 1))
            # Gelen kenarlar
            c.execute("""
                SELECT e.relation, s.name, s.type, e.evidence
                FROM edges e JOIN entities s ON e.source_id = s.id
                JOIN entities t ON e.target_id = t.id
                WHERE t.name = ?
            """, (name,))
            for rel, source, stype, evidence in c.fetchall():
                results.append({
                    "from": source, "relation": rel, "to": name,
                    "source_type": stype, "evidence": evidence, "depth": d
                })
                queue.append((source, d + 1))
        return results

# Örnek Kullanım
if __name__ == "__main__":
    gm = GraphMemory()

    # Varlıkları ekle
    gm.add_entity("Redis", "service", "In-memory cache ve queue")
    gm.add_entity("ADR-007", "adr", "Redis'ten BullMQ+Postgres queue'ya geçiş kararı")
    gm.add_entity("INCIDENT-142", "incident", "Outage: Redis OOM nedeniyle queue tıkandı")
    gm.add_entity("BullMQ", "service", "Redis tabanlı queue kütüphanesi")
    gm.add_entity("Postgres", "service", "Ana veritabanı")

    # İlişkiler
    gm.add_edge("INCIDENT-142", "caused_by", "Redis", "sentry: incident-142")
    gm.add_edge("ADR-007", "decided_in_response_to", "INCIDENT-142", "docs/adr/007.md")
    gm.add_edge("ADR-007", "supersedes", "Redis", "docs/adr/007.md")
    gm.add_edge("BullMQ", "replaces", "Redis", "docs/adr/007.md")
    gm.add_edge("BullMQ", "uses", "Postgres", "Yeni queue persistence")

    # Soru: Redis neden deprecated?
    print("Redis sorgusu:")
    for r in gm.trace("Redis", depth=2):
        print(f"  {r['from']} --[{r['relation']}]--> {r['to']}")
```

#### Graf Yaşam Döngüsü (Mevcut Kılavuzlarınızdan Entegre)

7 aşamalı graf kurulum ve bakım süreci:

| Aşama | İsim | Aktivite | Çıktı |
|:---|:---|:---|:---|
| **1** | Frame Question | Sistemin cevaplaması gereken kritik soruları yaz | `questions.md` |
| **2** | Name Entities | Projedeki servis/ADR/incident/karar kişilerini listele | `entities.csv` |
| **3** | Type Relationships | Standart edge vokabüleri belirle: `supersedes`, `blocks`, `caused_by`, `uses`, `mitigates` | `edge-vocab.md` |
| **4** | Attach Evidence | Her kenar için kaynak (commit hash, dosya, sentry link) | Evidence Register |
| **5** | Run Daily Loop | Günlük kararları ve incident'leri grafa ekle | Günlük ajanın otomatik |
| **6** | Exercise Failure | Bilinçli duplicate/stale veri enjekte et — sistemin nasıl davrandığını test et | Repair Log |
| **7** | Measure Runs | 10 soruluk bir eval seti hazırla, cevapların kalitesini ölç | Eval Report |

### 6.3 Beceri Tabanlı Self-Improvement (Hermes Desenleri)

Kapalı öğrenme döngüsü: her tamamlanan görevden sonra **yeni bir beceri** türet ve bir sonraki görevde kullan.

**Klasör yapısı:**
```
.claude/
└── skills/
    ├── index.json          # Beceri kataloğu
    ├── setup-nextjs.md     # Next.js kurulum becerisi
    ├── debug-prisma.md     # Prisma hata ayıklama
    ├── auth-patterns.md    # Auth desenleri
    └── review-checklist.md # Review kontrol listesi
```

**Beceri dosyası formatı (örnek: `debug-prisma.md`):**
```markdown
# Beceri: Prisma Hata Ayıklama
**Ne zaman uygulanır:** Prisma ile ilgili P2002, P2025, connection pool hatası alındığında.

## Adımlar
1. Önce Prisma Client'ın generate edildiğini doğrula: `npx prisma generate`
2. Connection string'i kontrol et: `postgresql://user:pass@host:5432/db?schema=public&connection_limit=10&pool_timeout=20`
3. Migration'ların uygulanıp uygulanmadığını kontrol et: `npx prisma migrate status`
4. P2002 (unique constraint) hatası için: findUnique yerine create işleminde alan tekrarını kontrol et
5. P2025 (record not found) için: foreign key cascade eksikliği veya silinmiş kayıt
6. Transaction timeout için: bağlantı havuzunu artır veya transaction aralığını daralt

## Yaygın Tuzaklar
- Next.js hot reload birden fazla Prisma Client örneği oluşturur → globalThis kullan
- Edge runtime'da connection pooling farklı ayar gerektirir (Prisma Data Proxy)
- Decimal alanları number'a cast etmeden karşılaştırma yapma

## Son Başarılı Kullanım
- Tarih: 2026-08-01
- Sorun: P2025 hatası user silinmeden ilişkili postlar var
- Çözüm: Cascade delete ekledim
- Kanıt: commit abc123
```

**Beceri öğrenme döngüsü prompt'u:**

Her büyük görev bitiminde Claude'a şu komutu ver:
```
Bu görevi tamamladık. Şimdi bu deneyimden bir "beceri" dosyası türet:
1. Bu görevde ne öğrendin?
2. Hangi hatalar yapıldı ve nasıl çözüldü?
3. Gelecekte benzer bir görevde hangi adımlar izlenmeli?
4. Hangi tuzaklara dikkat edilmeli?

Bunu .claude/skills/[konu-adi].md dosyasına yaz. Ayrıca index.json'ı güncelle.

Yeni görevlerde bu beceriyi otomatik olarak kullan.
```

### 6.4 Çoklu Model Orkestrasyonu

Her işi en iyi yapan modele verin. Bir modeli her şey için kullanmak **pahalı ve yavaş**tır.

| Görev Tipi | Tavsiye Model (Ağustos 2026) | Neden |
|:---|:---|:---|
| Mimari tasarım, ADR, en zor problemler | **Claude Fable 5** veya Opus 5 (/effort xhigh/max) | En yüksek muhakeme |
| Ana kod geliştirme | **Claude Sonnet 5** (promo $2/$10; 1 Eylül sonrası $3/$15) | Hız + kalite dengesi, en iyi F/P |
| Boilerplate/CRUD/test yazma | **Claude Haiku 4.5** veya DeepSeek-Coder | Hızlı, ucuz, yüksek hacim |
| Zero-context review | GPT-5 / Gemini 3 Pro / Opus 5 çapraz | Farklı kör noktalar, titizlik |
| Büyük refactor | Sonnet 5 / Opus 5 | Kod tabanı bağlamını iyi tutar |
| Dokümantasyon/özet | Haiku 4.5 | Hızlı, tutarlı, ucuz |

**Claude Code'ta model seçimi:**
```bash
# Global varsayılan
claude config set model claude-sonnet-5
# Oturum açarken
claude --model claude-opus-5
# Oturum içinde
/model claude-fable-5
# Zeka seviyesi (overthinking'i token'a dönüştürmeden kısar)
/effort high     # veya low/medium/xhigh/max
```

### 6.5 Native Arka Plan Ajanları ve Zamanlanmış Görevler

Manuel tmux kurgusu yerine Claude Code'un yerleşik özellikleri artık birçok işi otomatik yapıyor:

```bash
# Tek seferde arka plan ajanı (terminal kapansa da devam eder)
claude --bg --max-budget-usd 5 --max-turns 80 \
  "Tüm güncel deprecation uyarılarını bul ve düzelt"

# Çalışan/tamamlanmış ajanları izle
claude agents

# Belirli bir ajan oturumunu attach etme (consola geri dön)
claude -r <session-id>

# Bulut zamanlanmış görev (Scheduled Tasks) — sabah 9'a günlük rapor
claude --schedule "0 9 * * 1-5" "Günlük bağımlılık denetimi ve güvenlik raporu hazırla"
# veya oturum içinde:
/schedule
```

Triage cron'unuzda manuel tmux yerine `claude --bg` kullanmak daha temizdir: loglar Claude tarafından yönetilir, `claude agents` ile izlenir.

### 6.6 Eval ve Kalite Ölçüm Sistemi

Sistemin iyileşip iyileşmediğini ölçmek için sabit bir eval seti gerekir.

**`evals/eval-set.md`:**
```markdown
# Eval Soru Seti
# Ajan cevapları bu sorularda tutarlı şekilde doğru/yanlış olarak skorlanır

## Kategori: Mimari Bilgi
1. Redis kullanımından neden vazgeçtik? (Beklenen: INCIDENT-142, ADR-007, BullMQ)
2. Auth0 ne zaman kullanılmalı, ne zaman kendi auth çözümümüz? (Beklenen: ADR-003, kullanım durumu ayrımı)

## Kategori: Kod Standartları
3. Yeni bir API endpoint için hangi adımlar zorunlu? (auth, validation, error handling, test)
4. Migration nasıl oluşturulur? Elle dosya düzenlemek neden yasak?

## Kategori: Sorun Çözme
5. P2002 Prisma hatası alınırsa ilk kontrol edilecekler?
6. Next.js'te "use client" hangi durumlarda gerekir?
7. Redis connection 5 saniye gecikme yapıyorsa ne yapmak lazım?

## Kategori: Akıl Yürütme
8. Bu kodu production'a alabilir miyiz? Neden/neden olmaz?
   [kod bloğu]
```

**Periyodik ölçüm:**
- Haftada bir, temiz bir Claude oturumunda bu soruları sorun
- Cevapları doğru/yarı/yanlış olarak skorlayın
- Hafıza sistemi (graph, skills, CLAUDE.md) geliştikçe skor yükselmelidir
- Hedef: %90+ doğru cevap oranı

> **L4 Kontrol Noktası:**
> - [ ] Reaktif bug-fix pipeline (Sentry→Linear→PR) aktif
> - [ ] Knowledge Graph kurulu ve besleniyor
> - [ ] Beceri tabanlı öğrenme döngüsü çalışıyor
> - [ ] Çoklu model orkestrasyonu (göre göre model)
> - [ ] Eval seti tanımlı ve düzenli ölçüm yapılıyor
> - [ ] Guardroller ve güvenlik politikaları aktif
> - [ ] Organizasyonel çarpan 30x+ hissediliyor

---

