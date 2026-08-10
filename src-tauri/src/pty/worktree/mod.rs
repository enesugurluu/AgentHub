use portable_pty::CommandBuilder;

pub fn build_command(program: String, args: Vec<String>) -> CommandBuilder {
  let mut cmd = CommandBuilder::new(program);
  for arg in args {
    cmd.arg(arg);
  }
  cmd
}

