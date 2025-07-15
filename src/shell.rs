use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ShellMessage {
    Execute(String),
    Kill(Uuid),
    Input(Uuid, String),
}

pub struct ShellManager {
    current_shell: Option<Child>,
    shell_type: ShellType,
}

#[derive(Debug, Clone)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

impl ShellManager {
    pub fn new() -> Self {
        ShellManager {
            current_shell: None,
            shell_type: ShellType::Zsh, // Default to zsh
        }
    }

    pub async fn execute_command(&mut self, command: String) -> String {
        let mut cmd = Command::new(self.get_shell_command());
        cmd.arg("-c")
           .arg(&command)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();

                let mut output = String::new();

                // Read stdout
                let mut stdout_reader = BufReader::new(stdout);
                let mut stdout_line = String::new();
                while stdout_reader.read_line(&mut stdout_line).await.unwrap_or(0) > 0 {
                    output.push_str(&stdout_line);
                    stdout_line.clear();
                }

                // Read stderr
                let mut stderr_reader = BufReader::new(stderr);
                let mut stderr_line = String::new();
                while stderr_reader.read_line(&mut stderr_line).await.unwrap_or(0) > 0 {
                    output.push_str(&stderr_line);
                    stderr_line.clear();
                }

                // Wait for the process to complete
                let _ = child.wait().await;

                output
            }
            Err(e) => format!("Error executing command: {}", e),
        }
    }

    fn get_shell_command(&self) -> &str {
        match self.shell_type {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
            ShellType::PowerShell => "pwsh",
        }
    }

    pub async fn start_interactive_shell(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::new(self.get_shell_command());
        cmd.stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let child = cmd.spawn()?;
        self.current_shell = Some(child);
        Ok(())
    }
}
