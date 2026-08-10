use portable_pty::CommandBuilder;

pub fn build_command(
  program: String,
  args: Vec<String>,
  cwd: Option<String>,
  env: Vec<(String, String)>,
) -> CommandBuilder {
  let mut cmd = CommandBuilder::new(program);
  for arg in args {
    cmd.arg(arg);
  }
  if let Some(cwd_path) = cwd {
    cmd.cwd(cwd_path);
  }
  for (key, val) in env {
    cmd.env(key, val);
  }
  cmd
}

