## 2. Yüksek Seviye Mimari

```
┌───────────────────────────────────────────────────────────────────────┐
│              AJANŞIRKET DESKTOP APP (Tauri 2 Penceresi)                │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                  REACT 19 + TypeScript FRONTEND                 │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────┐ │ │
│  │  │  Office  │ │  Kanban  │ │ Memory   │ │ Terminal │ │Settings│ │ │
│  │  │  Floor   │ │  Board   │ │  Graph   │ │ Tabs/xtrm│ │Hire/...│ │ │
│  │  │ (SVG)    │ │ (dnd-kit)│ │ (Sigma)  │ │(xterm.js)│ │       │ │ │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └───┬───┘ │ │
│  │       │              │            │             │            │     │ │
│  └───────┴──────────────┴────────────┴─────────────┴────────────┴─────┘ │
│         │                  IPC (invoke + Channel/Event)                │
│  ┌──────┴──────────────────────────────────────────────────────────┐   │
│  │                   RUST BACKEND (Tauri 2.9 Core)                 │   │
│  │                                                                 │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────────┐│   │
│  │  │ Orkestrasyon │ │  PTY / Proc  │ │   Motor Adaptör Kaydı    ││   │
│  │  │ Motoru (CEO) │ │  Yöneticisi  │ │ (claude/codex/gemini/...)││   │
│  │  │ Görev bölme, │ │ portable-pty │ │  AgentRegistry + detect ││   │
│  │  │ dağıtım, A2A │ │ xterm.js kan.│ │  Spawn/stop/kill/stream ││   │
│  │  └──────┬───────┘ └──────┬───────┘ └───────────┬──────────────┘│   │
│  │         │                │                     │               │   │
│  │  ┌──────┴────────┐ ┌─────┴──────┐ ┌────────────┴──────────┐    │   │
│  │  │ Görev/Kanban  │ │ Git Worktr.│ │ Knowledge Graph        │    │   │
│  │  │ Yöneticisi    │ │ Yöneticisi│ │ (SQLite + FTS5 +       │    │   │
│  │  │ (statü, atama)│ │ (create/rm)│ │ sqlite-vec + graphology)│   │   │
│  │  └───────────────┘ └────────────┘ └────────────────────────┘    │   │
│  │                                                                 │   │
│  │  ┌───────────────┐ ┌──────────────┐ ┌────────────────────────┐  │   │
│  │  │ MCP Hub       │ │ Event Bus    │ │ Güvenlik/Policy Engine │  │   │
│  │  │ (resmi rmcp   │ │ (Tauri event ││ capability-based izin, │  │   │
│  │  │  Rust SDK)    │ │  + channels) ││ deny-list, onay akışı  │  │   │
│  │  └───────────────┘ └──────────────┘ └────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  Spawn edilmiş CLI ajanları (her biri kendi worktree + PTY):          │
│  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐            │
│  │  claude   │ │  codex    │ │  gemini   │ │ opencode  │  ...       │
│  │ (worktree) │ │ (worktree)│ │ (worktree)│ │ (worktree)│            │
│  └───────────┘ └───────────┘ └───────────┘ └───────────┘            │
└───────────────────────────────────────────────────────────────────────┘
```

### Mimari Özeti
- **Katman 1 (UI):** React 19 + TypeScript + Tailwind 4 + shadcn/ui. Office Floor (SVG), Kanban (dnd-kit), Memory Graph (Sigma.js + Graphology), Terminal (xterm.js + WebGL renderer), Hire/Fire ayarları.
- **Katman 2 (Çekirdek):** Tauri 2.9 Rust backend. Orkestrasyon motoru (CEO), PTY süreç yöneticisi (portable-pty), adaptör kaydı, görev/kanban, worktree, bilgi grafı, MCP Hub, event bus, güvenlik politikası.
- **Katman 3 (Protokol):** İki açık standart bir arada kullanılır:
  - **MCP (Dikey):** ajan ↔ araç entegrasyonu. Resmi Rust MCP SDK'sı (`rmcp`) ile child-process ve Streamable HTTP.
  - **A2A (Yatay):** ajan ↔ ajan iletişim. Google A2A v1.0 protokolü üzerinden Task lifecycle (submitted/working/input-required/completed/failed/canceled) ve Agent Card keşfi. Bu protokol 2026'da endüstri standardı haline geldi; çoklu çerçeve (LangGraph, Google ADK, MS Agent Framework) arası çalışabilirlik için gerekli.
- **Katman 4 (Ajanlar):** Harici CLI'lar (Claude Code, Codex, Gemini, OpenCode, Aider, Cline, Kilo, Copilot, Cursor, Qwen...). Her ajan kendi OS sürecinde, kendi git worktree'sinde izole çalışır; stdio/PTY üzerinden kontrol edilir.
- **Katman 5 (Veri):** Yerel tek-dosya SQLite (kanban, ajan, ayarlar, entity/edge, events, FTS5, sqlite-vec vektör) + JSONL (episodic konuşma geçmişi) + Markdown vault (bilgi notları/wiki-link) + yerel log.
- **Event bus:** Tüm ajan çıktıları, onay istekleri, maliyet/turn ilerlemeleri Tauri Channel üzerinden anlık React tarafına akar. Bu desen Tempest ve Terax gibi 2026 açık kaynak Tauri terminal projelerinde kanıtlanmıştır.

### Veri Akışı Örneği (Bir Görev Baştan Sona)

1. Kullanıcı kanban kartını bir ajan masasına sürükler.
2. React → `invoke("spawn_task", { agent_id, task })` ile Rust'a çağrı.
3. Orkestrasyon motoru (CEO):
   - Worktree yöneticisi üzerinden yeni worktree + branch oluşturur.
   - `AGENT_TASK.md` yazar (görev, kabul kriteri, ilgili not linkleri, bütçe).
   - Motor adaptörünü çağırarak CLI sürecini PTY ile spawn eder.
   - Event bus üzerinden `Spawned`, sonra anlık `Output` eventleri akar.
4. Kullanıcı ofis katında ajan masasında "working" animasyonunu görür; çıktı alttaki terminal sekmesinde akar.
5. Onay gerekirse `ApprovalRequested` event'i gelir, UI mavi bildirim gösterir, kullanıcı cevaplar.
6. Bütçe aşılırsa `Blocked { reason: budget_exceeded }` gelir, ajan otomatik duraklatılır.
7. Görev bitince `Completed { result }` event'i gelir; CEO review ajanı tetikler, sonuç kanbana/backlink'e bağlanır.
8. Memory Keeper ajanı gece epizodik hafızadan yeni semantic/procedural notlar önerir.

---
