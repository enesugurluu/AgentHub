use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorktreeInfo {
    pub path: String,
    #[serde(default)]
    pub agent_id: String,
    pub agent_name: String,
    pub branch_name: String,
    pub created_at: u64,
    pub parent_repo_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum BranchStrategy {
    #[serde(rename = "existingBranch")]
    ExistingBranch {
        name: String,
    },
    #[serde(rename = "newBranchFrom")]
    NewBranchFrom {
        #[serde(rename = "baseBranch")]
        base_branch: String,
        name: String,
    },
}

/// Worktree silme seçenekleri (docs 6.2; ADR-5/WP-05):
/// - `delete` (varsayılan): dizin + branch silinir (kirliyse `force` gerekir)
/// - `keep`: yalnızca yönetim metadata'sı (`.agenthub.json`) silinir; dizin + branch kalır
/// - `commit_and_keep`: değişiklikler commit'lenir, metadata silinir, dizin + branch saklanır
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WorktreeRemoveOptions {
    pub action: String,
    pub force: bool,
}

impl Default for WorktreeRemoveOptions {
    fn default() -> Self {
        Self {
            action: "delete".to_string(),
            force: false,
        }
    }
}

fn sanitize_agent_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>()
}

fn generate_worktree_name(agent_name: &str) -> String {
    let sanitized = sanitize_agent_name(agent_name);
    let uuid = Uuid::new_v4().to_string();
    let suffix = &uuid[..8];
    format!("{sanitized}-{suffix}")
}

fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute git command: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[tauri::command]
pub fn worktree_create(repo_path: String, agent_id: String, agent_name: String, branch_strategy: BranchStrategy) -> Result<WorktreeInfo, String> {
    let repo_dir = Path::new(&repo_path);
    if !repo_dir.exists() || !repo_dir.join(".git").exists() {
        return Err(format!("Invalid repository path: {}", repo_path));
    }

    let mut attempts = 0;
    let mut worktree_path = PathBuf::new();

    while attempts < 3 {
        let worktree_name = generate_worktree_name(&agent_name);
        // Store inside the repo's .git directory to avoid polluting parent directories
        let agenthub_dir = repo_dir.join(".git").join("agenthub-worktrees");

        if !agenthub_dir.exists() {
            fs::create_dir_all(&agenthub_dir).map_err(|e| format!("Failed to create worktree directory: {e}"))?;
        }

        worktree_path = agenthub_dir.join(&worktree_name);

        if !worktree_path.exists() {
            break;
        }
        attempts += 1;
    }

    if attempts >= 3 {
        return Err("Failed to generate a unique worktree path after 3 attempts.".into());
    }

    let worktree_path_str = worktree_path.to_string_lossy().to_string();
    let final_branch_name: String;

    match &branch_strategy {
        BranchStrategy::ExistingBranch { name } => {
            // Check if branch exists locally or on origin
            let check_local = run_git_command(repo_dir, &["rev-parse", "--verify", name]);
            if check_local.is_err() {
                 let check_origin = run_git_command(repo_dir, &["rev-parse", "--verify", &format!("origin/{name}")]);
                 if check_origin.is_err() {
                      return Err(format!("Existing branch '{name}' not found locally or on origin."));
                 }
            }

            // `git worktree add <path> <branch>` correctly sets up the worktree for an existing branch.
            run_git_command(repo_dir, &["worktree", "add", &worktree_path_str, name])
                .map_err(|e| format!("Failed to create worktree for existing branch: {e}"))?;
            final_branch_name = name.clone();
        },
        BranchStrategy::NewBranchFrom { base_branch, name } => {
            let mut actual_base = base_branch.clone();

            let check_base = run_git_command(repo_dir, &["rev-parse", "--verify", base_branch]);
            if check_base.is_err() {
                 let origin_base = format!("origin/{base_branch}");
                 let check_origin_base = run_git_command(repo_dir, &["rev-parse", "--verify", &origin_base]);
                 if check_origin_base.is_err() {
                     return Err(format!("Base branch '{base_branch}' not found locally or on origin."));
                 }
                 actual_base = origin_base;
            }

            run_git_command(repo_dir, &["worktree", "add", "-b", name, &worktree_path_str, &actual_base])
                .map_err(|e| format!("Failed to create worktree with new branch: {e}"))?;
            final_branch_name = name.clone();
        }
    }

    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let info = WorktreeInfo {
        path: worktree_path_str.clone(),
        agent_id,
        agent_name: agent_name.clone(),
        branch_name: final_branch_name,
        created_at,
        parent_repo_path: repo_path,
    };

    let metadata_path = worktree_path.join(".agenthub.json");
    if let Ok(json) = serde_json::to_string(&info) {
        let _ = fs::write(metadata_path, json);
    }

    Ok(info)
}

#[tauri::command]
pub fn worktree_remove(
    worktree_path: String,
    options: Option<WorktreeRemoveOptions>,
) -> Result<(), String> {
    let opts = options.unwrap_or_default();
    let wt_path = Path::new(&worktree_path);
    if !wt_path.exists() {
        return Err(format!("Worktree path does not exist: {worktree_path}"));
    }

    let metadata_path = wt_path.join(".agenthub.json");
    if !metadata_path.exists() {
        return Err("Not a managed agenthub worktree (missing .agenthub.json).".into());
    }

    let metadata_content = fs::read_to_string(&metadata_path)
        .map_err(|e| format!("Failed to read metadata: {e}"))?;

    let info: WorktreeInfo = serde_json::from_str(&metadata_content)
        .map_err(|e| format!("Failed to parse metadata: {e}"))?;

    match opts.action.as_str() {
        // keep: yönetimden çıkar, dizin + branch kalır (docs 6.2 "Worktree'yi koru").
        "keep" => {
            fs::remove_file(&metadata_path).map_err(|e| format!("Metadata silinemedi: {e}"))?;
            Ok(())
        }
        // commit_and_keep: değişiklikleri commit'le ve sakla (docs 6.2 "Commit'le sakla").
        "commit_and_keep" => {
            run_git_command(wt_path, &["add", "-A"])
                .map_err(|e| format!("Commit hazırlanamadı: {e}"))?;
            let msg = format!("agenthub: preserve worktree for {}", info.agent_name);
            // Değişiklik yoksa commit hata verir — bu kabul edilebilir (dizin zaten temiz).
            let _ = run_git_command(wt_path, &["commit", "-m", &msg]);
            fs::remove_file(&metadata_path).map_err(|e| format!("Metadata silinemedi: {e}"))?;
            Ok(())
        }
        // delete (varsayılan): mevcut davranış.
        _ => {
            let is_force = opts.force;
            let repo_dir = Path::new(&info.parent_repo_path);

            // To allow safe deletion, we need to temporarily move/remove the metadata file
            // so `git worktree remove` doesn't complain about untracked files if it's not forced.
            if !is_force {
                let _ = fs::remove_file(&metadata_path);
            }

            let mut args = vec!["worktree", "remove"];
            if is_force {
                args.push("--force");
            }
            args.push(&worktree_path);

            let result = run_git_command(repo_dir, &args);

            // If it failed and we removed the metadata file, try to put it back
            // (though in a real scenario, this might mean the worktree is dirty,
            // so returning an error is correct. The metadata file might be gone, which is less ideal).
            if let Err(e) = result {
                // Not: let-chain `&& let` edition 2024 gerektirir (proje 2021'de) — iç içe if kalır.
                if !is_force {
                    if let Ok(json) = serde_json::to_string(&info) {
                        let _ = fs::write(&metadata_path, json);
                    }
                }
                return Err(format!("Failed to remove worktree: {e}"));
            }

            Ok(())
        }
    }
}

/// Ajanın worktree'sini **garanti eder** (ADR-5/WP-05): varsa mevcut yönetilen
/// worktree'yi döndürür, yoksa `agent/<slug>-<suffix>` branch'iyle oluşturur.
/// `worktree_create` gibi `.git/agenthub-worktrees` altında, `.agenthub.json`
/// metadata'sıyla yönetilir.
pub fn ensure_agent_worktree(
    repo_path: &str,
    agent_id: &str,
    agent_name: &str,
    base_branch: &str,
) -> Result<WorktreeInfo, String> {
    // İdempotent: ajan için mevcut yönetilen worktree varsa yeniden oluşturma.
    let existing = worktree_list(repo_path.to_string())?;
    if let Some(info) = existing.into_iter().find(|wt| wt.agent_id == agent_id) {
        return Ok(info);
    }

    let suffix = &Uuid::new_v4().to_string()[..8];
    let strategy = BranchStrategy::NewBranchFrom {
        base_branch: base_branch.to_string(),
        name: format!("agent/{}-{}", sanitize_agent_name(agent_name), suffix),
    };
    worktree_create(
        repo_path.to_string(),
        agent_id.to_string(),
        agent_name.to_string(),
        strategy,
    )
}

/// Worktree'ye `.env.local` yazar (docs 10.3 runtime izolasyonu): port offset +
/// test DB + AGENTHUB_* değişkenleri. Mevcut dosyadaki anahtarlar **korunur**;
/// yalnızca eksik anahtarlar eklenir. `.env.local` `.gitignore` kapsamındadır.
pub fn prepare_worktree_env(worktree_path: &Path, agent_id: i64) -> Result<(), String> {
    let env_path = worktree_path.join(".env.local");
    let port = 3000 + (agent_id * 10);

    let mut desired: Vec<(String, String)> = vec![
        ("PORT".to_string(), port.to_string()),
        ("REDIS_DB".to_string(), agent_id.to_string()),
        ("TEST_DB".to_string(), format!("test_{agent_id}")),
        ("AGENTHUB_AGENT_ID".to_string(), agent_id.to_string()),
        (
            "AGENTHUB_WORKTREE".to_string(),
            worktree_path.to_string_lossy().to_string(),
        ),
    ];

    // Mevcut anahtarları oku; çakışanları ekleme (ajanın elle verdiği değerler korunur).
    let mut existing_keys: Vec<String> = Vec::new();
    if let Ok(content) = fs::read_to_string(&env_path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, _)) = line.split_once('=') {
                existing_keys.push(key.trim().to_string());
            }
        }
    }
    desired.retain(|(key, _)| !existing_keys.iter().any(|e| e == key));
    if desired.is_empty() {
        return Ok(());
    }

    let mut content = fs::read_to_string(&env_path).unwrap_or_default();
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    for (key, value) in desired {
        content.push_str(&format!("{key}={value}\n"));
    }
    fs::write(&env_path, content)
        .map_err(|e| format!(".env.local yazılamadı ({}): {e}", env_path.display()))?;
    Ok(())
}

/// `node_modules` paylaşımı (docs 10.1 m.3): ana repodaki `node_modules`'e bağıl
/// symlink (Windows: junction). Başarısızlık sessiz — disk paylaşımı iyileştirmedir,
/// spawn'ı engellemez.
pub fn link_node_modules(worktree_path: &Path, repo_path: &Path) -> bool {
    let src = repo_path.join("node_modules");
    let dst = worktree_path.join("node_modules");
    if !src.exists() || dst.exists() {
        return false;
    }

    #[cfg(unix)]
    {
        match std::os::unix::fs::symlink(&src, &dst) {
            Ok(_) => {
                tracing::info!("node_modules bağlandı: {}", dst.display());
                true
            }
            Err(e) => {
                tracing::warn!("node_modules symlink başarısız: {e}");
                false
            }
        }
    }
    #[cfg(windows)]
    {
        match std::os::windows::fs::symlink_dir(&src, &dst) {
            Ok(_) => {
                tracing::info!("node_modules junction bağlandı: {}", dst.display());
                true
            }
            Err(e) => {
                tracing::warn!(
                    "node_modules junction başarısız (ayrıcalık gerekebilir): {e}"
                );
                false
            }
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

/// Ajanın yönetilen worktree'sini listeler (spawn öncesi UI bilgisi; WP-07 masa ataması).
#[tauri::command]
pub fn worktree_for_agent(repo_path: String, agent_id: String) -> Result<WorktreeInfo, String> {
    let list = worktree_list(repo_path)?;
    list.into_iter()
        .find(|wt| wt.agent_id == agent_id)
        .ok_or_else(|| format!("agent {agent_id} için yönetilen worktree bulunamadı"))
}

#[tauri::command]
pub fn worktree_list(repo_path: String) -> Result<Vec<WorktreeInfo>, String> {
    let repo_dir = Path::new(&repo_path);
    if !repo_dir.exists() || !repo_dir.join(".git").exists() {
        return Err(format!("Invalid repository path: {}", repo_path));
    }

    let output = run_git_command(repo_dir, &["worktree", "list", "--porcelain"])
        .map_err(|e| format!("Failed to list worktrees: {e}"))?;

    let mut worktrees = Vec::new();

    for line in output.lines() {
        if line.starts_with("worktree ") {
            let path_str = line.trim_start_matches("worktree ").trim();
            let wt_path = Path::new(path_str);

            let metadata_path = wt_path.join(".agenthub.json");
            if metadata_path.exists() {
                if let Ok(metadata_content) = fs::read_to_string(&metadata_path) {
                    if let Ok(info) = serde_json::from_str::<WorktreeInfo>(&metadata_content) {
                        worktrees.push(info);
                    }
                }
            }
        }
    }

    Ok(worktrees)
}

pub fn resolve_worktree_path_for_agent(repo_path: &str, agent_id: &str) -> Result<String, String> {
    let repo_dir = Path::new(repo_path);
    let worktrees_root = repo_dir.join(".git").join("agenthub-worktrees");
    let worktrees_root = worktrees_root
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize worktree root: {e}"))?;

    let worktrees = worktree_list(repo_path.to_string())?;
    for wt in worktrees {
        if wt.agent_id == agent_id {
            let wt_path = Path::new(&wt.path);
            if !wt_path.exists() {
                continue;
            }

            let canonical = wt_path
                .canonicalize()
                .map_err(|e| format!("Failed to canonicalize worktree path: {e}"))?;

            if canonical.starts_with(&worktrees_root) {
                return Ok(canonical.to_string_lossy().to_string());
            }
        }
    }

    Err(format!("No valid managed worktree found for agent ID {agent_id}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn setup_mock_repo() -> (PathBuf, PathBuf) {
        // tempfile 3.27: `into_path()` deprecated — `keep()` aynı semantikte.
        let dir = tempdir().unwrap().keep();
        let repo_path = dir.join("main-repo");
        fs::create_dir_all(&repo_path).unwrap();

        Command::new("git").current_dir(&repo_path).args(["init"]).output().unwrap();
        Command::new("git").current_dir(&repo_path).args(["config", "user.name", "Test User"]).output().unwrap();
        Command::new("git").current_dir(&repo_path).args(["config", "user.email", "test@example.com"]).output().unwrap();

        fs::write(repo_path.join("README.md"), "Initial setup").unwrap();
        Command::new("git").current_dir(&repo_path).args(["add", "."]).output().unwrap();
        Command::new("git").current_dir(&repo_path).args(["commit", "-m", "Initial commit"]).output().unwrap();

        (dir, repo_path)
    }

    #[test]
    fn test_sanitize_agent_name() {
        assert_eq!(sanitize_agent_name("agent name!"), "agentname");
        assert_eq!(sanitize_agent_name("Agent-123_test*"), "Agent-123_test");
    }

    #[test]
    fn test_worktree_create_and_remove() {
        let (temp_dir, repo_path) = setup_mock_repo();
        let repo_path_str = repo_path.to_string_lossy().to_string();

        let branch_output = Command::new("git")
            .current_dir(&repo_path)
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .unwrap();
        let current_branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();

        let strategy = BranchStrategy::NewBranchFrom {
            base_branch: current_branch,
            name: "feat-1".to_string(),
        };

        let result = worktree_create(repo_path_str.clone(), "uuid-123".to_string(), "agent-1".to_string(), strategy);
        assert!(result.is_ok(), "Failed to create worktree: {:?}", result.err());

        let info = result.unwrap();
        assert!(info.path.contains("agent-1-"));

        let wt_path = Path::new(&info.path);
        assert!(wt_path.exists());
        assert!(wt_path.join(".agenthub.json").exists());

        let list_result = worktree_list(repo_path_str.clone());
        assert!(list_result.is_ok());
        let list = list_result.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].agent_name, "agent-1");

        // Safe removal should now work because we handle the metadata file explicitly.
        let remove_result = worktree_remove(info.path.clone(), None);
        assert!(remove_result.is_ok(), "Failed to remove worktree: {:?}", remove_result.err());
        assert!(!wt_path.exists(), "Worktree path should not exist after removal");

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn ensure_agent_worktree_is_idempotent() {
        let (_temp_dir, repo_path) = setup_mock_repo();
        let repo_str = repo_path.to_string_lossy().to_string();

        let first = ensure_agent_worktree(&repo_str, "42", "Test Ajan", "main").unwrap();
        assert!(first.path.contains("TestAjan-") || first.path.contains("Test-Ajan-") || first.path.contains("TestAjan"));
        assert!(Path::new(&first.path).join(".agenthub.json").exists());

        // İkinci çağrı yeni worktree oluşturmaz.
        let second = ensure_agent_worktree(&repo_str, "42", "Test Ajan", "main").unwrap();
        assert_eq!(first.path, second.path);

        let list = worktree_list(repo_str).unwrap();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn prepare_worktree_env_writes_offset_and_preserves_existing() {
        let (_temp_dir, repo_path) = setup_mock_repo();
        let info = ensure_agent_worktree(&repo_path.to_string_lossy(), "7", "Env Ajan", "main").unwrap();
        let wt = Path::new(&info.path);

        prepare_worktree_env(wt, 7).unwrap();
        let content = fs::read_to_string(wt.join(".env.local")).unwrap();
        assert!(content.contains("PORT=3070"), "port offset yanlış: {content}");
        assert!(content.contains("REDIS_DB=7"));
        assert!(content.contains("TEST_DB=test_7"));
        assert!(content.contains("AGENTHUB_AGENT_ID=7"));

        // Mevcut anahtarlar korunur; eksikler eklenir.
        fs::write(wt.join(".env.local"), "PORT=9999\n").unwrap();
        prepare_worktree_env(wt, 7).unwrap();
        let content = fs::read_to_string(wt.join(".env.local")).unwrap();
        assert!(content.contains("PORT=9999"), "mevcut PORT korunmalıydı: {content}");
        assert!(content.contains("REDIS_DB=7"));
    }

    #[test]
    fn worktree_remove_keep_keeps_dir_and_branch() {
        let (_temp_dir, repo_path) = setup_mock_repo();
        let info = ensure_agent_worktree(&repo_path.to_string_lossy(), "9", "Keep Ajan", "main").unwrap();
        let wt_path = Path::new(&info.path);
        assert!(wt_path.exists());

        let opts = WorktreeRemoveOptions {
            action: "keep".to_string(),
            force: false,
        };
        worktree_remove(info.path.clone(), Some(opts)).unwrap();

        // Dizin duruyor, yönetim metadata'sı gitti, branch hâlâ mevcut.
        assert!(wt_path.exists());
        assert!(!wt_path.join(".agenthub.json").exists());
        let branch = String::from_utf8_lossy(
            &Command::new("git").current_dir(&wt_path).args(["branch", "--show-current"]).output().unwrap().stdout,
        ).trim().to_string();
        assert!(branch.starts_with("agent/"), "branch korunmalıydı: {branch}");
    }

    #[test]
    fn worktree_remove_commit_and_keep_commits_changes() {
        let (_temp_dir, repo_path) = setup_mock_repo();
        let info = ensure_agent_worktree(&repo_path.to_string_lossy(), "11", "Commit Ajan", "main").unwrap();
        let wt_path = Path::new(&info.path);

        // Kirli değişiklik bırak.
        fs::write(wt_path.join("yeni-dosya.txt"), "degisiklik").unwrap();

        let opts = WorktreeRemoveOptions {
            action: "commit_and_keep".to_string(),
            force: false,
        };
        worktree_remove(info.path.clone(), Some(opts)).unwrap();

        assert!(wt_path.exists());
        assert!(!wt_path.join(".agenthub.json").exists());
        // Değişiklik commit'lenmiş olmalı (çalışma ağacı temiz).
        let status = String::from_utf8_lossy(
            &Command::new("git").current_dir(&wt_path).args(["status", "--porcelain"]).output().unwrap().stdout,
        ).trim().to_string();
        assert!(status.is_empty(), "commit sonrası çalışma ağacı temiz olmalı: {status}");
    }

    #[test]
    fn link_node_modules_is_best_effort() {
        let (_temp_dir, repo_path) = setup_mock_repo();
        // Ana repoda node_modules yoksa fonksiyon false döner, hata üretmez.
        let info = ensure_agent_worktree(&repo_path.to_string_lossy(), "13", "Link Ajan", "main").unwrap();
        let linked = link_node_modules(Path::new(&info.path), &repo_path);
        assert!(!linked || Path::new(&info.path).join("node_modules").exists());
    }
}
