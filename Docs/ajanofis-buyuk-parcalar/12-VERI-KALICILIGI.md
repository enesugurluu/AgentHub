## 12. Veri Kalıcılığı

### 12.1 Veritabanı Şeması (SQLite)
Tüm operasyonel veri yerel SQLite dosyasında tutulur (proje başına veya global):

```sql
-- Ajanlar
CREATE TABLE agents (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  role TEXT NOT NULL,
  avatar_color TEXT,
  motor TEXT NOT NULL,           -- claude, codex vb.
  model TEXT,
  system_prompt TEXT,
  config_json TEXT,               -- yetkiler, bütçe, tur limiti
  worktree_path TEXT,
  status TEXT DEFAULT 'idle',     -- idle/running/thinking/waiting/error/done
  created_at TEXT,
  hired_at TEXT,
  fired_at TEXT
);

-- Görevler (Kanban)
CREATE TABLE tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  acceptance_criteria TEXT,
  column TEXT DEFAULT 'backlog',
  assigned_agent_id INTEGER REFERENCES agents(id),
  parent_task_id INTEGER REFERENCES tasks(id),
  priority INTEGER DEFAULT 3,
  budget REAL,                    -- USD limiti
  spent_tokens_input INTEGER DEFAULT 0,
  spent_tokens_output INTEGER DEFAULT 0,
  spent_cost REAL DEFAULT 0,
  worktree_path TEXT,
  created_at TEXT,
  started_at TEXT,
  completed_at TEXT,
  blocked_by INTEGER
);

-- Hafıza notları (indeks; markdown dosyaları vault/ içinde asıl kaynak)
CREATE TABLE notes (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  path TEXT NOT NULL UNIQUE,             -- vault içindeki göreli yol
  note_type TEXT,                        -- adr, incident, entity, gotcha...
  aliases TEXT,                          -- JSON array, wikilink çözümleme için
  embedding BLOB,                        -- 384-dim float (all-MiniLM-L6-v2 yerel)
  created_at TEXT,
  updated_at TEXT
);
CREATE TABLE note_links (
  source_id INTEGER REFERENCES notes(id),
  target_id INTEGER REFERENCES notes(id),
  relation TEXT,                         -- references, supersedes, caused_by, ...
  context TEXT,                          -- linkin etrafındaki paragraf
  valid_from TEXT, valid_until TEXT,     -- bi-temporal (opsiyonel)
  PRIMARY KEY (source_id, target_id, relation, valid_from)
);
CREATE VIRTUAL TABLE note_fts USING fts5(
  title, content, path, note_type, tags,
  content='notes', content_rowid='id',
  tokenize='unicode61 remove_diacritics 2'
);

-- Vektör benzerlik araması için sqlite-vec sanal tablosu
CREATE VIRTUAL TABLE note_vec USING vec0(
  embedding float[384]
);

-- Olay/aktivite logu (denetim)
CREATE TABLE events (
  id INTEGER PRIMARY KEY,
  agent_id INTEGER REFERENCES agents(id),
  task_id INTEGER REFERENCES tasks(id),
  event_type TEXT,                -- spawn, output, error, approval, complete
  payload TEXT,
  timestamp TEXT
);

-- Ayarlar
CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT);
```

### 12.2 Konuşma Geçmişi
- Her ajan oturumunun çıktısı `~/.agentcompany/logs/<agent>/<task>-<timestamp>.jsonl` dosyasına JSONL olarak yazılır
- Her satır: `{timestamp, type (input/output/system/event), content, tokens, cost}`
- Bu log'lar incelemek ve hafızayı güncellemek için de kullanılır

### 12.3 Konfigürasyon
- Global konfigürasyon: `~/.agentcompany/config.toml`
- Proje bazlı konfigürasyon: `<proje>/.agentcompany.toml`
- API anahtarları OS keychain'de saklanır (Windows Credential Manager, macOS Keychain, Linux Secret Service)

---

