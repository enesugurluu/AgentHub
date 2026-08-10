## 11. MCP Entegrasyonu

MCP (Model Context Protocol), agent'ların dış dünyaya standart bir arayüzle erişmesi için Anthropic tarafından geliştirilmiş ve AAIF (Agentic AI Foundation) bünyesinde açık standart olarak yönetilen protokoldür. 2026 itibarıyla 97M+ indirilmeye ulaşmış, tüm büyük agent framework'leri tarafından destekleniyor. AjanŞirket'in dış sistem bağlantısının birincil yolu MCP olacak.

### 11.1 MCP'nin AjanŞirket'teki Rolü

MCP, agent ↔ tool dikey entegrasyonunu sağlar. A2A (Agent-to-Agent) ise agent ↔ agent yatay koordinasyonu sağlar (bkz. Bölüm 13). İki protokol birbirini tamamlar:

```
┌─────────────────────────────────────────────────────────────────┐
│  AjanŞirket Rust Core                                           │
│                                                                 │
│  ┌─────────────────┐          ┌─────────────────────────────┐  │
│  │  Motor Adaptörü │          │  MCP Hub (rmcp client)      │  │
│  │  - claude       │          │  ┌─────────────────────┐    │  │
│  │  - codex        │  stdio/  │  │  GitHub MCP         │    │  │
│  │  - gemini ...   │<────────>│  │  Linear/Jira MCP    │    │  │
│  └────────┬────────┘  HTTP    │  │  Sentry MCP         │    │  │
│           │                  │  │  Postgres MCP       │    │  │
│           │                  │  │  Figma MCP          │    │  │
│           │                  │  │  Playwright MCP     │    │  │
│           │                  │  │  Özel (internal)    │    │  │
│           │                  │  └─────────────────────┘    │  │
│           │                  │                              │  │
│           │                  │  Tool registry, izinler,    │  │
│           │                  │  cache, rate-limit          │  │
│           │                  └──────────────┬──────────────┘  │
│           │                                 │                  │
│           │   Event bus / A2A benzeri       │                  │
│           └─────────────────────────────────┘                  │
└─────────────────────────────────────────────────────────────────┘
```

### 11.2 MCP Hub Mimarisi (Rust)

- **Çekirdek:** Resmi Rust MCP SDK'sı `rmcp` crate'i (v0.8+) kullanılır. Bu crate child-process transport'u ile CLI MCP sunucularını spawn etmeye ve Streamable HTTP ile uzak sunuculara bağlanmaya izin verir.
- **Gerekçe:** rmcp modelcontextprotocol tarafından yayınlanan resmi referans implementasyondur. `mcp_client_rs` daha basit bir alternatif, `rust-mcp-sdk` ise çoklu istemci + Axum tabanlı sunucu ve DNS-rebinding koruması için tercih edilebilir. AjanŞirket için child-process ve Streamable HTTP istemcisi yeterli olacağından rmcp en başta seçilir.
- **Tool registry:** Mevcut tüm MCP araçları, isim/sağlayıcı/açıklama/giriş şeması ile birlikte merkezi bir kayıt defterine alınır. Bu, işe alım sırasında her ajan için tool whitelist'i tanımlamayı ve UI'da araç keşfini sağlar.
- **İzin modeli:** Her MCP aracı ayrı ayrı yetkilendirilebilir. Tehlikeli tool'lar (drop, deploy, force push) varsayılan olarak kapalı. İlk kullanımda kullanıcı onayı istenir, sonrasında kalıcı izin kaydedilir.
- **Credential yönetimi:** MCP sunucularının API anahtarları OS keychain'de saklanır; ajan süreçlerine doğrudan çevre değişkeni olarak enjekte edilmez.
- **Güvenlik:** HTTP/SSE transport'lu uzak MCP sunucuları varsayılan olarak sadece HTTPS kabul eder, localhost dışı bağlantılarda onay istenir.

### 11.3 Desteklenecek Standart MCP Sunucuları

İlk sürümde hazır gelen MCP sunucuları:

| MCP Sunucusu | Sağladığı yetenek |
|:---|:---|
| GitHub Resmi MCP | Issue/PR/branch/commit işlemleri, code search |
| Linear/Jira MCP | Kart oluşturma/güncelleme, sprint bilgisi |
| Sentry MCP | Production hata listesi, stack trace |
| Postgres/SQLite MCP | Salt-okunur veritabanı keşfi (izinli) |
| Playwright MCP | Tarayıcı otomasyonu, E2E test |
| Figma MCP | Tasarım tokenlarını çekme |
| Slack/Discord MCP | Bildirim, kanal mesajı |
| Google Drive/Notion MCP | Doküman okuma |
| File System MCP | Güvenli dosya erişimi (izinli path) |
| Memory MCP (builtin) | Kalıcı notlar, agent-hafıza entegrasyonu |

### 11.4 MCP ve CLI Agent Arasındaki Fark

AjanŞirket iki tip "dış bağlantı" arasında net ayrım yapar:

1. **CLI Ajanları (Claude Code, Codex, Gemini CLI, Aider, OpenCode...)** — Kod üreten, dosya değiştiren, komut çalıştıran "iş yapan" ajanlar. Bölüm 7'deki `AgentAdapter` trait'i üzerinden PTY/stdio ile konuşulur. Her biri kendi worktree'sinde izole çalışır.
2. **MCP Araçları** — Sadece "tek bir işlevi" (fetch, sorgu, rapor) gerçekleştiren aletler. Ajanlar görevini yaparken bu aletleri çağırır (veri çekmek, PR açmak, hata aramak için). MCP araçları kod yazmaz veya dosya değiştirmez (ilgili MCP aracı özellikle öyle tasarlanmadıkça).

### 11.5 MCP ve Hafıza Sistemi

- Knowledge Graph'ımız için özel bir dahili MCP sunucusu yazılabilir (bkz. sqlite-graph veya obra/knowledge-graph MCP paterni). Bu sunucu `kg_node`, `kg_search`, `kg_neighbors`, `kg_paths`, `kg_link` gibi tool'lar sağlar.
- Böylece tüm ajan (CLI) motorları aynı bilgi deposuna MCP üzerinden erişir. Hafıza tüm ajanların paylaştığı ortak bir katman olarak çalışır.
- Ajanın kendi ürettiği öğrenmeler otomatik olarak bu MCP'ye beslenir; doğrulama sonrası graf'a eklenir (bkz. Bölüm 9.4 Memory Keeper ajanı).

### 11.6 Gelecek: Kullanıcı Özel MCP'ler

- Kullanıcı Ayarlar → MCP menüsünden özel stdio MCP sunucusu ekleyebilir (komut ve argümanları ile).
- Market/topluluk paylaşımı için sonraki sürümlerde "MCP Market" eklenebilir.
- Güvenlik için tüm özel sunucular sandbox modda (izinleri başlangıçta tamamen kapalı) çalıştırılır.

---
