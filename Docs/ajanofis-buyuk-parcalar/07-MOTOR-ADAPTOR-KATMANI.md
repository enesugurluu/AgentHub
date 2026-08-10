## 7. Çoklu AI Motor Adaptör Katmanı

AjanŞirket'in en önemli teknik kararlarından biri, her AI CLI'ı aynı arayüzün arkasında soymutlayan bir **AgentAdapter** trait'idir. Bu, CEO'nun hangi motoru kullanırsa kullansın aynı "dil"i konuşmasını sağlar; yeni bir CLI eklendiğinde sadece trait'i implemente eden bir modül yazmak yeterlidir.

### 7.1 AgentAdapter Trait (Rust)

```rust
// src-tauri/src/agents/mod.rs
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::process::Child;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentCapabilities {
    pub name: String,                  // "Claude Code"
    pub vendor: String,                // "Anthropic"
    pub version: String,               // "2.1.90"
    pub supports_worktree: bool,
    pub supports_bg: bool,
    pub supports_budget: bool,
    pub supports_effort: bool,
    pub supports_json_output: bool,
    pub supports_non_interactive: bool,
    pub installed: bool,
    pub install_hint: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SpawnOptions {
    pub workdir: PathBuf,
    pub task_file: PathBuf,            // AGENT_TASK.md
    pub model: Option<String>,
    pub effort: Option<Effort>,        // low/medium/high/xhigh/max
    pub max_budget_usd: Option<f64>,
    pub max_turns: Option<u32>,
    pub env: Vec<(String, String)>,
    pub non_interactive: bool,         // --print / -p
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentEvent {
    Spawned { pid: u32 },
    Output { text: String },           // stdout chunk
    Error  { text: String },           // stderr chunk
    ApprovalRequested { request: ApprovalRequest },
    Progress { turn: u32, cost: f64, tokens_in: u64, tokens_out: u64 },
    Completed { result: TaskResult },
    Failed { error: String },
    Exited { code: i32 },
}

#[async_trait]
pub trait AgentAdapter: Send + Sync {
    /// Motor kimliği (claude, codex, gemini, opencode, aider, ...)
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;

    /// Kurulum mu? version ne? native mı yoksa npm/gem/pip ile mi kurulu?
    async fn detect(&self) -> Result<AgentCapabilities>;

    /// Kurulum komutu (native veya paket yöneticisi)
    fn install_command(&self) -> Vec<String>;

    /// Belirtilen seçeneklerle motoru spawn et; Child process + event stream döndürür.
    async fn spawn(&self, opts: SpawnOptions) -> Result<AgentHandle>;

    /// Ajanın izin isteğine cevap (approve/deny/düzenle)
    async fn respond_to_approval(&self, handle: &AgentHandle, resp: ApprovalResponse) -> Result<()>;

    /// Ajanı nazikçe durdur
    async fn stop(&self, handle: &AgentHandle) -> Result<()>;

    /// Zorla öldür
    async fn kill(&self, handle: &AgentHandle) -> Result<()>;
}

pub struct AgentHandle {
    pub id: String,                     // uuid
    pub adapter_id: String,
    pub child: Child,
    pub pty_master: Option<std::fs::File>, // portable-pty master
    pub workdir: PathBuf,
    pub worktree_path: Option<PathBuf>,
}
```

### 7.2 CLI Spawn Stratejileri

Her CLI için komut ve flag eşleşmeleri farklıdır. Örnek:

```rust
// src-tauri/src/agents/claude.rs
pub struct ClaudeAdapter;

#[async_trait]
impl AgentAdapter for ClaudeAdapter {
    fn id(&self) -> &'static str { "claude" }
    fn display_name(&self) -> &'static str { "Claude Code" }

    async fn detect(&self) -> Result<AgentCapabilities> {
        // which claude, claude --version, claude doctor parse
        Ok(AgentCapabilities {
            name: "Claude Code".into(),
            vendor: "Anthropic".into(),
            version: detect_version("claude", "--version").await?,
            supports_worktree: true,
            supports_bg: true,
            supports_budget: true,
            supports_effort: true,
            supports_json_output: true,
            supports_non_interactive: true,
            installed: which::which("claude").is_ok(),
            install_hint: Some(
                "curl -fsSL https://claude.ai/install.sh | bash".into()
            ),
        })
    }

    fn install_command(&self) -> Vec<String> {
        vec!["bash".into(), "-c".into(),
             "curl -fsSL https://claude.ai/install.sh | bash".into()]
    }

    async fn spawn(&self, opts: SpawnOptions) -> Result<AgentHandle> {
        let mut cmd = tokio::process::Command::new("claude");
        cmd.current_dir(&opts.workdir);

        if opts.non_interactive {
            cmd.arg("--print");
            cmd.arg("--output-format").arg("stream-json");
        }
        if let Some(wt) = &opts.worktree_path {
            cmd.arg("--worktree").arg(wt.file_name().unwrap());
        }
        if let Some(b) = opts.max_budget_usd {
            cmd.arg("--max-budget-usd").arg(b.to_string());
        }
        if let Some(t) = opts.max_turns {
            cmd.arg("--max-turns").arg(t.to_string());
        }
        if let Some(e) = &opts.effort {
            cmd.arg("--effort").arg(e.as_str()); // low/medium/high/xhigh/max
        }
        // Task dosyasını stdin'den prompt olarak gönder,
        // veya $ARGUMENTS biçiminde argüman
        cmd.arg("-").stdin(std::process::Stdio::piped())
           .stdout(std::process::Stdio::piped())
           .stderr(std::process::Stdio::piped());

        for (k, v) in &opts.env { cmd.env(k, v); }

        // portable-pty ile etkileşimli oturum da açılabilir;
        // --print modu için pipe yeterlidir.
        let child = cmd.spawn()?;
        Ok(AgentHandle::new(self.id(), child, opts.workdir.clone()))
    }
    // ... stop/kill/respond_to_approval implementasyonları
}
```

Diğer adaptörler benzer mantıkla:
- **Codex CLI**: `codex` komutu; `--approval-mode`, `--model`, sandbox ayarları.
- **Gemini CLI**: `gemini` komutu; Apache-2.0, çoklu model.
- **OpenCode**: `opencode`; Go tabanlı TUI.
- **Aider**: `aider`; `--model`, `--no-auto-commits`, `--architect` gibi özel flagler.
- **Cline / Roo Code / Kilo Code**: VS Code extension olmakla birlikte CLI modları da var.

### 7.3 Çıktı ve Durum Çözümleme (Stream Parser)

Her ajan için farklı bir stream çözümleyici gerekir:

| CLI | Çıktı formatı | Maliyet/progress bilgisi nerede? |
|:---|:---|:---|
| Claude Code | `--output-format stream-json` ile JSON event stream; normalde ANSI terminal | `/usage` komutu veya stream'de `usage` olayı |
| Codex CLI | JSONL + ANSI karışık | stderr üzerinde `[tur X/Y]` kalıbı veya JSON |
| Gemini CLI | ANSI TUI, `--print` ile düz metin | Çıktı sonu özeti |
| OpenCode | JSONL event stream | `cost` event'i |
| Aider | Düz metin çıktısı, regex'le parse | `Tokens: ...` satırı |

Her adaptör kendi parser'ını uygular; sonuçta ortak `AgentEvent` kanalına döker:
```rust
pub enum AgentEvent {
    Spawned { pid: u32 },
    Output { text: String },
    Error { text: String },
    ApprovalRequested { request: ApprovalRequest },
    Progress { turn: u32, cost: f64, tokens_in: u64, tokens_out: u64 },
    Completed { result: TaskResult },
    Failed { error: String },
}
```

### 7.4 Onay Akışı (Human-in-the-loop)

- PTY yakalayıcı izin/approval pattern'lerini yakalar:
  - Claude: "Do you want to proceed?"
  - Aider: "Allow? (y)es/(n)o/(a)ll"
  - Genel: regex `\[y/n\]`, "Allow?"
- Yakalanan onay UI'ya `ApprovalRequested` eventi olarak gönderilir
- Kullanıcı UI'dan dört seçenekten birini seçer:
  1. **Allow once** — bir kerelik onay
  2. **Always allow this pattern** — kalıcı kural ekle
  3. **Deny** — reddet
  4. **Edit command** — komutu düzenleyip tekrar çalıştır
- Cevap PTY'ye uygun tuş vuruşu veya stdin girişi olarak geri yollar.

### 7.5 Kurulum ve Sağlık Kontrolü

Uygulama açılışında:
1. Tüm adaptörlerin `detect()` fonksiyonu çağrılır; kurulu olmayanlar "Kur" butonlu olarak listelenir
2. Kurulum komutu kullanıcı onayıyla çalıştırılır (`install_command()`)
3. Her ajan için temel bir `claude doctor` benzeri doğrulama çalıştırılır
4. Hangi versiyonda olduğu ve hangi flag'leri desteklediği tespit edilip UI'da "Uyumlu" rozeti konur

### 7.6 Mevcut CLI'ları Seçme Kriterleri

AjanŞirket adaptör eklerken şu kriterlere bakar:
- Stabil CLI arayüzü (breaking change sıklığı düşük)
- stdin/stdout ile script edilebilirlik (TUI ama script modu var mı?)
- `--print` / `-p` / JSON output desteği
- Worktree veya benzeri izolasyon desteği
- Güvenlik denetim modeli (onay prompt'ları tutarlı mı?)
- Lisans (kapalı beta / açık kaynak)

---
