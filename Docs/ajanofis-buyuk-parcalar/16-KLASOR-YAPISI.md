## 16. Klasör Yapısı

```
agentcompany/
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs            # Tauri builder, plugin kayıtları
│   │   ├── commands/         # Tauri IPC komutları (invoke)
│   │   ├── agents/           # AgentAdapter trait ve implementasyonlar
│   │   │   ├── mod.rs
│   │   │   ├── registry.rs   # detect/install/kayıt
│   │   │   ├── claude.rs
│   │   │   ├── codex.rs
│   │   │   ├── gemini.rs
│   │   │   ├── opencode.rs
│   │   │   └── ...
│   │   ├── orchestrator.rs   # CEO mantığı (görev bölme, dağıtım, A2A)
│   │   ├── pty.rs            # PTY yönetimi (portable-pty)
│   │   ├── worktree.rs       # Git worktree işlemleri
│   │   ├── kanban.rs         # Görev/kanban yönetimi
│   │   ├── memory/           # Bilgi grafı alt modülü
│   │   │   ├── mod.rs
│   │   │   ├── db.rs         # rusqlite + FTS5 + sqlite-vec
│   │   │   ├── parser.rs     # wiki-link/frontmatter/etiket
│   │   │   ├── graph.rs      # recursive CTE traversal, algoritmalar
│   │   │   ├── embed.rs      # embedding (yerel/uzak)
│   │   │   └── indexer.rs    # dosya izleme + artımlı güncelleme
│   │   ├── mcp/              # MCP hub
│   │   │   ├── mod.rs
│   │   │   ├── client.rs     # rmcp client wrapper
│   │   │   ├── registry.rs   # tool whitelist/izin
│   │   │   └── servers.rs    # yerel/uzak sunucu yönetimi
│   │   ├── security.rs       # politika motoru, regex denetim, audit
│   │   ├── db.rs             # ana SQLite bağlantısı + migration
│   │   ├── events.rs         # Event bus (Channel)
│   │   ├── install.rs        # CLI ajanlarını kurma/update
│   │   └── telemetry.rs      # opsiyonel kullanım istatistiği
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/         # Tauri capability JSON dosyaları
│
├── src/                      # React frontend
│   ├── App.tsx
│   ├── main.tsx
│   ├── router.tsx
│   ├── components/
│   │   ├── OfficeFloor/      # Ofis görünümü (SVG)
│   │   │   ├── OfficeFloor.tsx
│   │   │   ├── AgentDesk.tsx
│   │   │   ├── StatusBadge.tsx
│   │   │   ├── MeetingRoom.tsx
│   │   │   └── Decor.tsx
│   │   ├── Kanban/           # Kanban panosu
│   │   │   ├── Board.tsx
│   │   │   ├── Column.tsx
│   │   │   ├── Card.tsx
│   │   │   └── Swimlane.tsx
│   │   ├── Memory/           # Hafıza grafı
│   │   │   ├── GraphView.tsx       # Sigma.js canvas
│   │   │   ├── NoteEditor.tsx      # markdown edit
│   │   │   ├── Backlinks.tsx
│   │   │   └── SearchPalette.tsx   # cmdk benzeri
│   │   ├── Terminal/         # Terminal sekmeleri
│   │   │   ├── XTerminal.tsx       # xterm.js React wrapper
│   │   │   ├── TerminalTabs.tsx
│   │   │   └── ApprovalDialog.tsx  # onay iletişimi
│   │   ├── Settings/
│   │   │   ├── HireWizard.tsx      # işe alım sihirbazı
│   │   │   ├── FireDialog.tsx
│   │   │   ├── AgentInspector.tsx
│   │   │   ├── MCPSettings.tsx
│   │   │   ├── PolicyEditor.tsx
│   │   │   └── BillingDashboard.tsx
│   │   ├── TopBar/
│   │   │   ├── CostMeter.tsx
│   │   │   └── GlobalSearch.tsx
│   │   └── ui/               # shadcn/ui bileşenleri
│   ├── store/                # Zustand store
│   │   ├── agents.ts
│   │   ├── kanban.ts
│   │   ├── memory.ts
│   │   ├── terminal.ts
│   │   └── settings.ts
│   ├── hooks/                # React hooks
│   │   ├── useAgentEvents.ts
│   │   ├── useTerminal.ts
│   │   └── useGraphData.ts
│   ├── lib/
│   │   ├── ipc.ts            # Tauri invoke köprüsü
│   │   ├── markdown.ts       # remark pipeline
│   │   ├── wiki-link.ts      # [[link]] çözümleme
│   │   └── utils.ts
│   ├── styles/               # global CSS, tema
│   └── assets/
│
├── vault/                    # Geliştirici referans notlar (uygulama ile ilgili)
├── package.json
├── vite.config.ts
├── tailwind.config.ts
├── tsconfig.json
└── README.md
```

---

