use notify::{RecommendedWatcher, Watcher, RecursiveMode, EventKind};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use std::time::Duration;

pub enum FileWatcherEvent {
    FileChanged(PathBuf),
    FileCreated(PathBuf),
    FileDeleted(PathBuf),
    DirectoryChanged(PathBuf),
    Error(String),
}

pub struct FileWatcherService {
    watcher: RecommendedWatcher,
    event_receiver: mpsc::Receiver<FileWatcherEvent>,
}

impl FileWatcherService {
    pub fn new() -> Result<(Self, mpsc::UnboundedReceiver<FileWatcherEvent>), Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let watcher = RecommendedWatcher::new(move |res| {
            match res {
                Ok(event) => {
                    if let Some(path) = event.paths.first() {
                        let file_event = match event.kind {
                            EventKind::Modify(_) => FileWatcherEvent::FileChanged(path.clone()),
                            EventKind::Create(_) => FileWatcherEvent::FileCreated(path.clone()),
                            EventKind::Remove => FileWatcherEvent::FileDeleted(path.clone()),
                            EventKind::Any | EventKind::Access(_) | EventKind::Other => {
                                // More granular handling could be added here
                                if path.is_dir() {
                                    FileWatcherEvent::DirectoryChanged(path.clone())
                                } else {
                                    FileWatcherEvent::FileChanged(path.clone())
                                }
                            }
                        };
                        if let Err(e) = event_tx.send(file_event) {
                            eprintln!("Failed to send file watcher event: {}", e);
                        }
                    }
                },
                Err(e) => {
                    if let Err(e) = event_tx.send(FileWatcherEvent::Error(e.to_string())) {
                        eprintln!("Failed to send file watcher error: {}", e);
                    }
                }
            }
        }, notify::Config::default().with_poll_interval(Duration::from_secs(1)))?;

        Ok((
            FileWatcherService {
                watcher,
                event_receiver: rx, // This receiver is not used by the service itself, but could be for internal state
            },
            event_rx, // This is the channel for external consumers
        ))
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        println!("Watching path: {:?}", path.as_ref());
        Ok(())
    }

    pub fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        self.watcher.unwatch(path.as_ref())?;
        println!("Unwatching path: {:?}", path.as_ref());
        Ok(())
    }
}
