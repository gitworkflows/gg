use notify::{RecommendedWatcher, Watcher, RecursiveMode, EventKind};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use std::time::Duration;

use super::FileWatcherEvent;

pub struct FileWatcherService {
    watcher: RecommendedWatcher,
    event_receiver: mpsc::UnboundedReceiver<FileWatcherEvent>,
}

impl FileWatcherService {
    pub fn new() -> Result<(Self, mpsc::UnboundedReceiver<FileWatcherEvent>), Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::unbounded_channel();

        let watcher = RecommendedWatcher::new(move |res| {
            match res {
                Ok(event) => {
                    let event_type = event.kind;
                    let paths = event.paths;
                    for path in paths {
                        let file_event = if event_type.is_create() {
                            FileWatcherEvent::FileCreated(path)
                        } else if event_type.is_modify() {
                            FileWatcherEvent::FileChanged(path)
                        } else if event_type.is_remove() {
                            FileWatcherEvent::FileDeleted(path)
                        } else if event_type.is_any() && path.is_dir() {
                            FileWatcherEvent::DirectoryChanged(path)
                        } else {
                            continue; // Ignore other event types for now
                        };
                        if tx.send(file_event).is_err() {
                            eprintln!("FileWatcherService: Failed to send event, receiver dropped.");
                            break;
                        }
                    }
                },
                Err(e) => {
                    if tx.send(FileWatcherEvent::Error(e.to_string())).is_err() {
                        eprintln!("FileWatcherService: Failed to send error event, receiver dropped.");
                    }
                },
            }
        }, notify::Config::default().with_poll_interval(Duration::from_secs(1)))?; // Poll every second

        Ok((Self { watcher, event_receiver: rx }, rx))
    }

    pub fn watch(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        println!("FileWatcherService: Watching {:?}", path);
        Ok(())
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.watcher.unwatch(path)?;
        println!("FileWatcherService: Unwatching {:?}", path);
        Ok(())
    }

    // Dummy constructor for when real watcher fails
    pub fn new_dummy() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        FileWatcherService {
            watcher: notify::RecommendedWatcher::new(move |res| { /* do nothing */ }, notify::Config::default()).unwrap(),
            event_receiver: rx, // This receiver will never receive anything from the dummy watcher
        }
    }
}
