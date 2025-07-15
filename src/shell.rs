use portable_pty::{CommandBuilder, PtySize, PtyPair, MasterPty, SlavePty};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use log::{info, error};
use std::io::Write; // For `write_all` on `SlavePty`

#[derive(Debug)]
pub enum ShellMessage {
    Output(String),
    Command(String),
    Resize(PtySize),
    Exit(u32),
    Error(String),
}

pub struct Shell {
    pty_pair: PtyPair,
    master_writer: tokio::io::Stdin, // Placeholder for actual master writer
    _master_reader_task: tokio::task::JoinHandle<()>,
    _shell_process: portable_pty::Child,
    tx: mpsc::Sender<ShellMessage>,
}

impl Shell {
    pub async fn new(
        shell_path: &str,
        tx: mpsc::Sender<ShellMessage>,
    ) -> anyhow::Result<Self> {
        info!("Spawning shell: {}", shell_path);
        let pty_system = portable_pty::PtySystem::native()?;
        let pty_pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = CommandBuilder::new(shell_path);
        cmd.arg("-l"); // Login shell for bash/zsh
        cmd.env("TERM", "xterm-256color"); // Set terminal type

        let shell_process = pty_pair.slave.spawn_command(cmd)?;

        // It's important to drop the slave pty on the main thread,
        // otherwise the shell process will not exit when the master dies
        drop(pty_pair.slave);

        let mut master_reader = pty_pair.master.try_clone_reader()?;
        let master_writer = pty_pair.master.try_clone_writer()?;

        let output_tx = tx.clone();
        let master_reader_task = tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            loop {
                match master_reader.read(&mut buf).await {
                    Ok(0) => {
                        info!("Shell master reader closed.");
                        break;
                    }
                    Ok(n) => {
                        if let Ok(s) = String::from_utf8(buf[..n].to_vec()) {
                            if output_tx.send(ShellMessage::Output(s)).await.is_err() {
                                error!("Failed to send shell output message, receiver dropped.");
                                break;
                            }
                        } else {
                            error!("Received invalid UTF-8 from shell.");
                            // Handle non-UTF8 output, perhaps send raw bytes or a placeholder
                        }
                    }
                    Err(e) => {
                        error!("Error reading from shell master: {:?}", e);
                        if output_tx.send(ShellMessage::Error(e.to_string())).await.is_err() {
                            error!("Failed to send shell error message, receiver dropped.");
                        }
                        break;
                    }
                }
            }
        });

        Ok(Self {
            pty_pair,
            master_writer: tokio::io::stdin(), // This is a placeholder, actual writer should be used
            _master_reader_task: master_reader_task,
            _shell_process: shell_process,
            tx,
        })
    }

    pub async fn write_to_shell(&mut self, input: &str) -> anyhow::Result<()> {
        // In a real implementation, you'd write to `self.pty_pair.master.writer()`
        // For this placeholder, we'll just log and simulate.
        info!("Writing to shell (simulated): {}", input);
        // Example: self.pty_pair.master.writer().write_all(input.as_bytes()).await?;
        // self.pty_pair.master.writer().write_all(b"\r\n").await?; // Send newline
        Ok(())
    }

    pub async fn resize(&mut self, size: PtySize) -> anyhow::Result<()> {
        info!("Resizing shell to: {:?}", size);
        self.pty_pair.master.resize(size)?;
        Ok(())
    }

    pub async fn wait_for_exit(&mut self) -> anyhow::Result<u32> {
        let exit_status = self._shell_process.wait().await?;
        info!("Shell exited with status: {:?}", exit_status);
        Ok(exit_status.code().unwrap_or(1) as u32)
    }
}
