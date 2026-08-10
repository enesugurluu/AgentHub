## 9. Hafıza Sistemi (Bağlantılı Bilgi Grafı)

AjanŞirket'in temel farklılaştırıcısı, ajanların ve kullanıcının öğrendiklerini Obsidian-tarzı çift yönlü bağlantılı bir graf olarak saklayan hafıza katmanıdır. Vektör benzerlik araması (RAG) tek başına yetmez; multi-hop ilişki sorguları ve kanıt zinciri gerektiren sorular ("Redis'i neden bıraktık?", "Hangi incident sonrası ADR-007 yazıldı?", "Bu kararın etkilediği servisler hangileri?") sadece graf veri yapılarıyla deterministik olarak cevaplanır.

### 9.1 Mimari Referans

Bu mimari doğrudan üretimde kendini kanıtlamış iki açık kaynak projeden esinleniyor:

1. **obra/knowledge-graph** (Jesse Vincent tarafından, 2026) — Obsidian vault'unu SQLite + FTS5 + sqlite-vec + graphology ile indeksleyen, CLI ve MCP sunucusu olarak çalışan araç. Semantic arama, yol bulma, Louvain topluluk tespiti, PageRank, BFS traversal, fuzzy eşleştirme yapıyor. Xenova/all-MiniLM-L6-v2 (22 MB, yerel) ile gömme vektörleri üretiyor.
2. **sqlite-graph** (crates.io, 2026) — Üzerine inşa edilen gömülü graf veritabanı; recursive CTE ile traversal, bi-temporal edge, FTS5 + vektör füzyonu (RRF), entity deduplication (Jaro-Winkler), tek dosya backup.

AjanŞirket aynı kalıbı (tek dosya SQLite + FTS5 + sqlite-vec + graf algoritmaları) kullanır, ancak doğrudan Rust backend içine gömülü olarak. Veri sahipliği kullanıcıda, bulut zorunlu değil.

### 9.2 Katmanlı Hafıza Modeli (Episodic / Semantic / Procedural)

Üretim çoklu-ajan sistemlerinde 2026 standardı üç-katmanlı hafızadır:

| Katman | İçerik | Saklama |
|:---|:---|:---|
| **Episodic (Olay)** | Her konuşma, ajan oturumu, verilen komutun tam transkripti, alınan çıktı, sonuç. Zaman damgası + maliyet + durumla birlikte. | JSONL loglar (`~/.agentcompany/logs/...jsonl`) + SQLite `events` tablosu |
| **Semantic (Bilgi)** | Kalıcı notlar, ADR'ler, incident raporları, kod standartları, mimari kararlar, entity tanımları ve aralarındaki ilişkiler. **Wiki-linkli markdown dosyaları** (vault) + indeks (grafı). | Markdown dosyalar (vault) + SQLite FTS5 + sqlite-vec vektör + graf kenarları |
| **Procedural (Beceri)** | Nasıl yapılır bilgisi; tekrar kullanılabilir iş akışları (deploy checklist, debug playbook, PR şablonu). Skill dosyaları olarak. | `.agentcompany/skills/<isim>/{SKILL.md,scripts/...}` |

CEO ve Memory Keeper ajanı episodic hafızayı düzenli olarak "sindirip" semantic hafızaya damıtır; tekrarlayan başarılı prosedürleri procedural hafızaya (skill) dönüştürür.

### 9.3 Veri Modeli

```sql
-- Varlık (düğüm): bir markdown notu/entity
CREATE TABLE entities (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,              -- "Redis", "ADR-007", "INCIDENT-142"
  entity_type TEXT NOT NULL,       -- adr, incident, service, person, decision, skill, file...
  path TEXT UNIQUE,                -- vault içindeki dosya yolu (örn. adr/007-queue.md)
  description TEXT,
  metadata TEXT,                   -- JSON (tags, author, aliases...)
  embedding BLOB,                  -- 384-dim float vector (all-MiniLM-L6-v2)
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT
);

-- İlişki (kenar)
CREATE TABLE edges (
  id INTEGER PRIMARY KEY,
  source_id INTEGER NOT NULL REFERENCES entities(id),
  target_id INTEGER NOT NULL REFERENCES entities(id),
  relation TEXT NOT NULL,          -- supersedes, blocks, caused_by, uses, decided_in, mitigates, references, related_to
  evidence TEXT,                   -- kaynak dosya/commit hash/Sentry linki
  valid_from TEXT,                 -- bi-temporal başlangıç (opsiyonel)
  valid_until TEXT,                -- bi-temporal bitiş (opsiyonel; eski kararlar iz bırakır)
  created_at TEXT DEFAULT (datetime('now')),
  UNIQUE(source_id, target_id, relation, valid_from)
);

CREATE INDEX idx_edges_source ON edges(source_id);
CREATE INDEX idx_edges_target ON edges(target_id);

-- Tam metin arama (FTS5)
CREATE VIRTUAL TABLE entity_fts USING fts5(
  name, description, content, path, entity_type,
  content='entities', content_rowid='id',
  tokenize='unicode61'
);

-- Triggers ile FTS otomatik senkron
CREATE TRIGGER entities_ai AFTER INSERT ON entities BEGIN
  INSERT INTO entity_fts(rowid, name, description, content, path, entity_type)
  VALUES (new.id, new.name, new.description, '', new.path, new.entity_type);
END;
-- (update/delete trigger'ları benzer)
```

> **Bi-temporal edge** özelliği: bir karar sonradan yanlışlanırsa kenarı silmek yerine `valid_until` ile tarih damgalayarak yeni kenar eklenir. Böylece "o tarihte biz bunu biliyor muyduk?" sorusu cevaplanabilir.

### 9.4 Arama ve Traversal

- **Anahtar kelime:** SQLite FTS5 üzerinde tam metin
- **Semantik:** sqlite-vec ile kosinüs benzerliği (yerel 22 MB embedding modeli; opsiyonel olarak harici embedding API)
- **Hibrit:** Reciprocal Rank Fusion (RRF) ile FTS ve vektör sonuçlarını birleştirme
- **Graf traversal:** Recursive CTE ile N-hop BFS/DFS, yol bulma, ortak komşu (iki varlık arasındaki en kısa ilişki zinciri)
- **Graf algoritmaları:** Louvain community detection, PageRank (merkezi düğüm bulma), betweenness centrality (köprü düğümler). Bu algoritmalar Rust tarafında graphology-benzeri bir kütüphane (veya basit implementasyon) ile veya graphology-wasm kullanarak Web Worker içinde çalıştırılabilir.

### 9.5 Wiki-link Çözümleme

Obsidian ile birebir uyum için aynı çözümleme mantığı kullanılır:

- `[[hedef]]`, `[[hedef#başlık]]`, `[[hedef#^blockid]]`, `[[hedef|görüntülenen isim]]` sözdizimi
- **shortestPossible** çözümleme: aynı isimde birden fazla dosya varsa en kısa yol kazansın; belirsizlik durumunda kullanıcıya soru işareti çıkar.
- `!([[resim.png]])` embed sözdizimi
- Tablo içinde `\|` kaçışı (Obsidian gibi)
- `#etiket` ve `#başlık` referansları

Frontend tarafında `remark-wiki-link-plus` veya `remark-obsidian-link` kullanılır; Rust tarafında vault indeksleme safhasında basit bir regex/pulldown-cst taraması veya wikilink kütüphanesi kullanılır. Bitbonsai/mcpvault'daki `parseWikiLink`, `scanHeadings`, `scanBlockIds`, `resolveWikiLink` gibi saf fonksiyonlar referans alınabilir.

### 9.6 Memory Keeper Ajanı (Otomatik Hafıza)

Her gece (veya kullanıcı manuel tetiklediğinde) düşük zeka seviyesinde bir ajan (Haiku 4.5 ile) çalışır:

1. Son episodik logları (events tablosu, JSONL) tarar
2. Tekrarlayan hata paternleri, yeni alınan kararlar, tekrar kullanılan çözümleri ayıklar
3. Yeni entity'ler önerir ve ilişkiler kurmak için draft oluşturur
4. Kullanıcı onayına sunar: "Aşağıdaki 7 notu grafa ekleyeyim mi?"
5. Onay sonrası markdown dosyaları vault'a yazılır, SQLite indeks güncellenir

Bu, kullanıcının not tutma yükünü azaltır; aynı zamanda HALÜSİNASYON riskini minimize eder çünkü tüm öneriler kullanıcı onayından geçer.

### 9.7 Vault Dosya Yapısı

```
~/.agentcompany/vault/
├── adr/
│   ├── 001-postgresql.md
│   └── 007-queue-bullmq.md
├── incidents/
│   └── incident-142-redis-oom.md
├── services/
│   ├── auth.md
│   └── payment.md
├── decisions/
├── patterns/
│   └── retry-exponential-backoff.md
├── gotchas/
│   └── prisma-transaction-timeout.md
├── people/
├── projects/
├── daily/            -- günlük otomatik notlar
├── templates/
└── index.md
```

Kullanıcı aynı vault'u doğrudan Obsidian ile açıp düzenleyebilir; AjanŞirket değişiklikleri `notify` crate'i ile anlık fark edip indeksi günceller.

### 9.8 Görselleştirme

- **2D varsayılan:** Sigma.js v3 + graphology, WebGL hızlı render, 10K+ düğüm akıcı
- **3B mod (opsiyonel):** three-forcegraph / react-force-graph-3d, görsel keşif ve sunum
- Renk kodlaması: entity_type'a göre; düğüm çapı PageRank skoruyla orantılı
- Kenarlar relation tipine göre renk/çizgi tipi (düz = references, kesik = caused_by, vb.)
- Düğüme tıklayınca sağ panelde markdown içerik + backlink listesi (Obsidian'daki gibi)
- "İzole et" komutu: bir düğümün N-hop mahallesini izole edip detaylı inceleme

### 9.9 Gizlilik

- Tüm veri yerelde (tek SQLite dosyası + markdown dosyaları); `cp memory.db backup.db` ile anlık yedek
- Opsiyonel uçtan-uca şifreli bulut senkronizasyonu (sonraki sürüm)
- Embedding modeli varsayılan olarak yerel çalışır; isteğe bağlı olarak harici embedding API seçilebilir

---
