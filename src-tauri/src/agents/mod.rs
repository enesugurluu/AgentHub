//! CLI ajan adaptörleri (AjanOfis docs Bölüm 7).
//!
//! Her AI CLI (`claude`, `codex`, `gemini`, `opencode`, ...) aynı
//! `EngineAdapter` arayüzü arkasında soyutlanır. FAZ0'da ilk dalga olarak
//! Claude Code gelir; sonraki motorlar aynı kalıpta eklenir.

pub mod claude;

pub use claude::ClaudeAdapter;
