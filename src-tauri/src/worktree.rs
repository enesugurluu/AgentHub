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

fn sanitize_agent_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>()
}

fn generate_worktree_name(agent_name: &str) -> String {
    let sanitized = sanitize_agent_name(agent_name);
    let uuid = Uuid::new_v4().to_string();
    let suffix = &uuid[..8];
    format!("{}-{}", sanitized, suffix)
}

fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute git command: {}", e))?;

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
            fs::create_dir_all(&agenthub_dir).map_err(|e| format!("Failed to create worktree directory: {}", e))?;
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
                 let check_origin = run_git_command(repo_dir, &["rev-parse", "--verify", &format!("origin/{}", name)]);
                 if check_origin.is_err() {
                      return Err(format!("Existing branch '{}' not found locally or on origin.", name));
                 }
            }

            // `git worktree add <path> <branch>` correctly sets up the worktree for an existing branch.
            run_git_command(repo_dir, &["worktree", "add", &worktree_path_str, name])
                .map_err(|e| format!("Failed to create worktree for existing branch: {}", e))?;
            final_branch_name = name.clone();
        },
        BranchStrategy::NewBranchFrom { base_branch, name } => {
            let mut actual_base = base_branch.clone();

            let check_base = run_git_command(repo_dir, &["rev-parse", "--verify", base_branch]);
            if check_base.is_err() {
                 let origin_base = format!("origin/{}", base_branch);
                 let check_origin_base = run_git_command(repo_dir, &["rev-parse", "--verify", &origin_base]);
                 if check_origin_base.is_err() {
                     return Err(format!("Base branch '{}' not found locally or on origin.", base_branch));
                 }
                 actual_base = origin_base;
            }

            run_git_command(repo_dir, &["worktree", "add", "-b", name, &worktree_path_str, &actual_base])
                .map_err(|e| format!("Failed to create worktree with new branch: {}", e))?;
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
pub fn worktree_remove(worktree_path: String, force: Option<bool>) -> Result<(), String> {
    let wt_path = Path::new(&worktree_path);
    if !wt_path.exists() {
        return Err(format!("Worktree path does not exist: {}", worktree_path));
    }

    let metadata_path = wt_path.join(".agenthub.json");
    if !metadata_path.exists() {
        return Err("Not a managed agenthub worktree (missing .agenthub.json).".into());
    }

    let is_force = force.unwrap_or(false);

    let metadata_content = fs::read_to_string(&metadata_path)
        .map_err(|e| format!("Failed to read metadata: {}", e))?;

    let info: WorktreeInfo = serde_json::from_str(&metadata_content)
        .map_err(|e| format!("Failed to parse metadata: {}", e))?;

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
        if !is_force {
             if let Ok(json) = serde_json::to_string(&info) {
                  let _ = fs::write(&metadata_path, json);
             }
        }
        return Err(format!("Failed to remove worktree: {}", e));
    }

    Ok(())
}

#[tauri::command]
pub fn worktree_list(repo_path: String) -> Result<Vec<WorktreeInfo>, String> {
    let repo_dir = Path::new(&repo_path);
    if !repo_dir.exists() || !repo_dir.join(".git").exists() {
        return Err(format!("Invalid repository path: {}", repo_path));
    }

    let output = run_git_command(repo_dir, &["worktree", "list", "--porcelain"])
        .map_err(|e| format!("Failed to list worktrees: {}", e))?;

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

    Err(format!("No valid managed worktree found for agent ID {}", agent_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn setup_mock_repo() -> (PathBuf, PathBuf) {
        let dir = tempdir().unwrap().into_path();
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
        let remove_result = worktree_remove(info.path.clone(), Some(false));
        assert!(remove_result.is_ok(), "Failed to remove worktree: {:?}", remove_result.err());
        assert!(!wt_path.exists(), "Worktree path should not exist after removal");

        let _ = fs::remove_dir_all(temp_dir);
    }
}
