use fuser::{Filesystem, MountOption, FileAttr, FileType, Request, ReplyAttr, ReplyEntry, ReplyData, ReplyDirectory, ReplyWrite, ReplyOpen, ReplyCreate};
use libc::{ENOENT, EIO, EINVAL};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::time::{Duration, SystemTime};
use tokio::sync::oneshot;
use log::{info, error};
use std::sync::{Arc, Mutex};

const TTL: Duration = Duration::from_secs(1); // 1 second
const BLOCK_SIZE: u32 = 512; // Standard block size

const ROOT_INO: u64 = 1;
const DEFAULT_FILE_PERM: u32 = 0o644;
const DEFAULT_DIR_PERM: u32 = 0o755;
const DEFAULT_UID: u32 = 501; // Example UID
const DEFAULT_GID: u32 = 20;  // Example GID

#[derive(Debug, Clone)]
pub enum FsNode {
    File(File),
    Directory(Directory),
}

#[derive(Debug, Clone)]
pub struct File {
    pub ino: u64,
    pub name: String,
    pub content: Vec<u8>,
    pub attr: FileAttr,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub ino: u64,
    pub name: String,
    pub children: HashMap<String, u64>, // name -> inode
    pub attr: FileAttr,
}

/// An in-memory virtual file system.
pub struct VirtualFileSystem {
    next_ino: u64,
    nodes: HashMap<u64, FsNode>, // inode -> FsNode
    // Parent map: child_ino -> parent_ino
    parent_map: HashMap<u64, u64>,
}

impl VirtualFileSystem {
    pub fn new() -> Arc<Mutex<Self>> {
        let mut fs = VirtualFileSystem {
            next_ino: ROOT_INO + 1,
            nodes: HashMap::new(),
            parent_map: HashMap::new(),
        };

        // Create root directory
        let root_attr = FileAttr {
            ino: ROOT_INO,
            size: 0,
            blocks: 0,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind: FileType::Directory,
            perm: DEFAULT_DIR_PERM,
            nlink: 2, // . and ..
            uid: DEFAULT_UID,
            gid: DEFAULT_GID,
            rdev: 0,
            flags: 0,
            blksize: BLOCK_SIZE,
        };
        fs.nodes.insert(ROOT_INO, FsNode::Directory(Directory {
            ino: ROOT_INO,
            name: "/".to_string(),
            children: HashMap::new(),
            attr: root_attr,
        }));

        Arc::new(Mutex::new(fs))
    }

    fn get_new_ino(&mut self) -> u64 {
        let ino = self.next_ino;
        self.next_ino += 1;
        ino
    }

    pub fn create_file(&mut self, parent_ino: u64, name: &str, content: Vec<u8>) -> anyhow::Result<u64> {
        let new_ino = self.get_new_ino();
        let size = content.len() as u64;
        let blocks = (size + (BLOCK_SIZE as u64 - 1)) / (BLOCK_SIZE as u64);
        let now = SystemTime::now();

        let file_attr = FileAttr {
            ino: new_ino,
            size,
            blocks,
            atime: now,
            mtime: now,
            ctime: now,
            crtime: now,
            kind: FileType::RegularFile,
            perm: DEFAULT_FILE_PERM,
            nlink: 1,
            uid: DEFAULT_UID,
            gid: DEFAULT_GID,
            rdev: 0,
            flags: 0,
            blksize: BLOCK_SIZE,
        };

        let file_node = File {
            ino: new_ino,
            name: name.to_string(),
            content,
            attr: file_attr,
        };

        if let Some(FsNode::Directory(parent_dir)) = self.nodes.get_mut(&parent_ino) {
            if parent_dir.children.contains_key(name) {
                return Err(anyhow::anyhow!("File already exists"));
            }
            parent_dir.children.insert(name.to_string(), new_ino);
            parent_dir.attr.mtime = now;
            parent_dir.attr.ctime = now;
            parent_dir.attr.size = parent_dir.children.len() as u64 * 1024; // Update directory size heuristic
            self.nodes.insert(new_ino, FsNode::File(file_node));
            self.parent_map.insert(new_ino, parent_ino);
            info!("Created file '{}' (ino: {}) in directory (ino: {})", name, new_ino, parent_ino);
            Ok(new_ino)
        } else {
            error!("Parent directory (ino: {}) not found or not a directory.", parent_ino);
            Err(anyhow::anyhow!("Parent not a directory"))
        }
    }

    pub fn create_directory(&mut self, parent_ino: u64, name: &str) -> anyhow::Result<u64> {
        let new_ino = self.get_new_ino();
        let now = SystemTime::now();

        let dir_attr = FileAttr {
            ino: new_ino,
            size: 0, // Size of directory is usually 0 or a multiple of block size
            blocks: 0,
            atime: now,
            mtime: now,
            ctime: now,
            crtime: now,
            kind: FileType::Directory,
            perm: DEFAULT_DIR_PERM,
            nlink: 2, // . and ..
            uid: DEFAULT_UID,
            gid: DEFAULT_GID,
            rdev: 0,
            flags: 0,
            blksize: BLOCK_SIZE,
        };

        let dir_node = Directory {
            ino: new_ino,
            name: name.to_string(),
            children: HashMap::new(),
            attr: dir_attr,
        };

        if let Some(FsNode::Directory(parent_dir)) = self.nodes.get_mut(&parent_ino) {
            if parent_dir.children.contains_key(name) {
                return Err(anyhow::anyanyhow!("Directory already exists"));
            }
            parent_dir.children.insert(name.to_string(), new_ino);
            parent_dir.attr.mtime = now;
            parent_dir.attr.ctime = now;
            parent_dir.attr.nlink += 1; // Parent's link count increases for new child directory
            self.nodes.insert(new_ino, FsNode::Directory(dir_node));
            self.parent_map.insert(new_ino, parent_ino);
            info!("Created directory '{}' (ino: {}) in directory (ino: {})", name, new_ino, parent_ino);
            Ok(new_ino)
        } else {
            error!("Parent directory (ino: {}) not found or not a directory.", parent_ino);
            Err(anyhow::anyhow!("Parent not a directory"))
        }
    }

    fn get_attr_for_ino(&self, ino: u64) -> Option<FileAttr> {
        self.nodes.get(&ino).map(|node| match node {
            FsNode::File(file) => file.attr,
            FsNode::Directory(dir) => dir.attr,
        })
    }
}

impl Filesystem for VirtualFileSystem {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        info!("lookup(parent={}, name={:?})", parent, name);
        let name_str = name.to_str().unwrap_or("");

        if let Some(FsNode::Directory(parent_dir)) = self.nodes.get(&parent) {
            if let Some(&ino) = parent_dir.children.get(name_str) {
                if let Some(attr) = self.get_attr_for_ino(ino) {
                    reply.entry(&TTL, &attr, 0); // 0 generation
                    return;
                }
            }
        }
        reply.error(ENOENT);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        info!("getattr(ino={})", ino);
        if let Some(attr) = self.get_attr_for_ino(ino) {
            reply.attr(&TTL, &attr);
        } else {
            reply.error(ENOENT);
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, size: u32, _flags: i32, _lock_owner: Option<u64>, reply: ReplyData) {
        info!("read(ino={}, offset={}, size={})", ino, offset, size);
        if let Some(FsNode::File(file)) = self.nodes.get(&ino) {
            let start = offset as usize;
            let end = (offset + size as i64) as usize;
            if start < file.content.len() {
                let data = &file.content[start..std::cmp::min(end, file.content.len())];
                reply.data(data);
            } else {
                reply.data(&[]);
            }
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        info!("readdir(ino={}, offset={})", ino, offset);
        if let Some(FsNode::Directory(dir)) = self.nodes.get(&ino) {
            let mut entries = vec![
                (ino, FileType::Directory, ".".to_string()),
                (self.parent_map.get(&ino).cloned().unwrap_or(ROOT_INO), FileType::Directory, "..".to_string()),
            ];

            for (name, &child_ino) in &dir.children {
                if let Some(child_node) = self.nodes.get(&child_ino) {
                    entries.push((child_ino, child_node.attr().kind, name.clone()));
                }
            }

            let mut i = 0;
            for entry in entries.into_iter().skip(offset as usize) {
                // The offset is 1-based for FUSE, so we add 1 to `i`
                reply.add(entry.0, offset + i as i64 + 1, entry.1, entry.2);
                i += 1;
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }

    fn write(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, data: &[u8], _flags: i32, _lock_owner: Option<u64>, reply: ReplyWrite) {
        info!("write(ino={}, offset={}, size={})", ino, offset, data.len());
        if let Some(FsNode::File(file)) = self.nodes.get_mut(&ino) {
            let start = offset as usize;
            let end = start + data.len();

            if end > file.content.len() {
                file.content.resize(end, 0);
            }
            file.content[start..end].copy_from_slice(data);

            file.attr.size = file.content.len() as u64;
            file.attr.blocks = (file.attr.size + (BLOCK_SIZE as u64 - 1)) / (BLOCK_SIZE as u64);
            file.attr.mtime = SystemTime::now();
            file.attr.ctime = SystemTime::now();

            reply.written(data.len() as u32);
        } else {
            reply.error(ENOENT);
        }
    }

    fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, _flags: u32, reply: ReplyEntry) {
        info!("mkdir(parent={}, name={:?})", parent, name);
        let name_str = name.to_str().unwrap_or("");
        match self.create_directory(parent, name_str) {
            Ok(ino) => {
                if let Some(attr) = self.get_attr_for_ino(ino) {
                    reply.entry(&TTL, &attr, 0);
                } else {
                    reply.error(EIO); // Should not happen if create_directory succeeds
                }
            }
            Err(_) => reply.error(EIO), // Or EEXIST if already exists
        }
    }

    fn rmdir(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        info!("rmdir(parent={}, name={:?})", parent, name);
        let name_str = name.to_str().unwrap_or("");

        if let Some(FsNode::Directory(parent_dir)) = self.nodes.get_mut(&parent) {
            if let Some(&child_ino) = parent_dir.children.get(name_str) {
                if let Some(FsNode::Directory(child_dir)) = self.nodes.get(&child_ino) {
                    if child_dir.children.is_empty() {
                        parent_dir.children.remove(name_str);
                        self.nodes.remove(&child_ino);
                        self.parent_map.remove(&child_ino);
                        parent_dir.attr.mtime = SystemTime::now();
                        parent_dir.attr.ctime = SystemTime::now();
                        parent_dir.attr.nlink -= 1;
                        reply.ok();
                        return;
                    } else {
                        reply.error(EINVAL); // Directory not empty
                        return;
                    }
                }
            }
        }
        reply.error(ENOENT);
    }

    fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, _flags: u32, reply: ReplyCreate) {
        info!("create(parent={}, name={:?})", parent, name);
        let name_str = name.to_str().unwrap_or("");
        match self.create_file(parent, name_str, Vec::new()) {
            Ok(ino) => {
                if let Some(attr) = self.get_attr_for_ino(ino) {
                    reply.created(&TTL, &attr, 0, 0, 0); // fh, flags, generation
                } else {
                    reply.error(EIO);
                }
            }
            Err(_) => reply.error(EIO),
        }
    }

    fn unlink(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        info!("unlink(parent={}, name={:?})", parent, name);
        let name_str = name.to_str().unwrap_or("");

        if let Some(FsNode::Directory(parent_dir)) = self.nodes.get_mut(&parent) {
            if let Some(&child_ino) = parent_dir.children.get(name_str) {
                if let Some(FsNode::File(_)) = self.nodes.get(&child_ino) {
                    parent_dir.children.remove(name_str);
                    self.nodes.remove(&child_ino);
                    self.parent_map.remove(&child_ino);
                    parent_dir.attr.mtime = SystemTime::now();
                    parent_dir.attr.ctime = SystemTime::now();
                    reply.ok();
                    return;
                }
            }
        }
        reply.error(ENOENT);
    }

    fn rename(&mut self, _req: &Request, parent: u64, name: &OsStr, new_parent: u64, new_name: &OsStr, _flags: u32, reply: ReplyEntry) {
        info!("rename(parent={}, name={:?}, new_parent={}, new_name={:?})", parent, name, new_parent, new_name);
        let name_str = name.to_str().unwrap_or("");
        let new_name_str = new_name.to_str().unwrap_or("");

        // Find the node to rename
        let old_child_ino;
        if let Some(FsNode::Directory(old_parent_dir)) = self.nodes.get_mut(&parent) {
            if let Some(&ino) = old_parent_dir.children.get(name_str) {
                old_child_ino = ino;
            } else {
                reply.error(ENOENT);
                return;
            }
        } else {
            reply.error(ENOENT);
            return;
        }

        // Check if new_name already exists in new_parent
        if let Some(FsNode::Directory(new_parent_dir)) = self.nodes.get(&new_parent) {
            if new_parent_dir.children.contains_key(new_name_str) {
                reply.error(EIO); // Or EEXIST
                return;
            }
        } else {
            reply.error(ENOENT); // New parent not found
            return;
        }

        // Perform the rename
        let mut node_to_move = self.nodes.remove(&old_child_ino).unwrap();
        if let Some(FsNode::Directory(old_parent_dir)) = self.nodes.get_mut(&parent) {
            old_parent_dir.children.remove(name_str);
            old_parent_dir.attr.mtime = SystemTime::now();
            old_parent_dir.attr.ctime = SystemTime::now();
        }

        match &mut node_to_move {
            FsNode::File(file) => {
                file.name = new_name_str.to_string();
                file.attr.mtime = SystemTime::now();
                file.attr.ctime = SystemTime::now();
            }
            FsNode::Directory(dir) => {
                dir.name = new_name_str.to_string();
                dir.attr.mtime = SystemTime::now();
                dir.attr.ctime = SystemTime::now();
                // Update nlink for directories if moving between parents
                if parent != new_parent {
                    if let Some(FsNode::Directory(old_parent_dir)) = self.nodes.get_mut(&parent) {
                        old_parent_dir.attr.nlink -= 1;
                    }
                    if let Some(FsNode::Directory(new_parent_dir)) = self.nodes.get_mut(&new_parent) {
                        new_parent_dir.attr.nlink += 1;
                    }
                }
            }
        }

        if let Some(FsNode::Directory(new_parent_dir)) = self.nodes.get_mut(&new_parent) {
            new_parent_dir.children.insert(new_name_str.to_string(), old_child_ino);
            new_parent_dir.attr.mtime = SystemTime::now();
            new_parent_dir.attr.ctime = SystemTime::now();
            self.nodes.insert(old_child_ino, node_to_move);
            self.parent_map.insert(old_child_ino, new_parent);
            reply.ok();
        } else {
            // This case should ideally not be reached if checks above are thorough
            reply.error(EIO);
        }
    }
}

/// Starts the virtual filesystem in a separate Tokio task.
/// Returns a `oneshot::Sender` to signal shutdown.
pub async fn start_virtual_filesystem(mount_point: &str) -> anyhow::Result<oneshot::Sender<()>> {
    let fs = VirtualFileSystem::new();
    let (tx, rx) = oneshot::channel();
    let mount_point_path = PathBuf::from(mount_point);

    // Ensure the mount point exists
    if !mount_point_path.exists() {
        tokio::fs::create_dir_all(&mount_point_path).await?;
    }

    let mount_options = vec![
        MountOption::FSName("warp_vfs".to_string()),
        MountOption::NoDev,
        MountOption::NoSuid,
        MountOption::AllowOther, // Allows other users to access (useful for testing)
    ];

    info!("Mounting virtual filesystem at: {}", mount_point);

    // Spawn the FUSE mounting operation in a blocking thread
    let fs_clone = Arc::clone(&fs);
    tokio::task::spawn_blocking(move || {
        let mount_result = fuser::mount2(fs_clone, &mount_point_path, &mount_options);
        if let Err(e) = mount_result {
            error!("Failed to mount FUSE filesystem: {}", e);
        } else {
            info!("FUSE filesystem unmounted successfully.");
        }
    });

    // Wait for the shutdown signal
    tokio::spawn(async move {
        rx.await.ok();
        info!("Received shutdown signal for virtual filesystem.");
        // Unmount the filesystem when signal is received
        if let Err(e) = fuser::unmount(mount_point_path) {
            error!("Failed to unmount FUSE filesystem: {}", e);
        }
    });

    Ok(tx)
}

/// Unmounts the virtual filesystem.
pub async fn unmount_virtual_filesystem(mount_point: &str) -> anyhow::Result<()> {
    info!("Attempting to unmount virtual filesystem from: {}", mount_point);
    // This function is now primarily for external calls if the `oneshot::Sender` is not available.
    // The `start_virtual_filesystem` function now handles unmounting via the oneshot channel.
    // However, keeping this for explicit unmount calls if needed.
    if let Err(e) = fuser::unmount(mount_point) {
        error!("Failed to unmount FUSE filesystem from {}: {}", mount_point, e);
        Err(anyhow::anyhow!("Failed to unmount FUSE: {}", e))
    } else {
        info!("Successfully unmounted virtual filesystem from {}", mount_point);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use tempfile::tempdir;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_vfs_mount_and_file_operations() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let mount_point = temp_dir.path().join("vfs_mount");
        let mount_point_str = mount_point.to_str().unwrap();

        let shutdown_tx = start_virtual_filesystem(mount_point_str).await?;

        // Give FUSE time to mount
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Test creating a file
        let file_path = mount_point.join("test_file.txt");
        fs::write(&file_path, "Hello VFS!")?;
        assert!(file_path.exists());

        // Test reading a file
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, "Hello VFS!");

        // Test updating a file
        fs::write(&file_path, "Updated content.")?;
        let updated_content = fs::read_to_string(&file_path)?;
        assert_eq!(updated_content, "Updated content.");

        // Test creating a directory
        let dir_path = mount_point.join("test_dir");
        fs::create_dir(&dir_path)?;
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        // Test creating a file inside the new directory
        let nested_file_path = dir_path.join("nested.txt");
        fs::write(&nested_file_path, "Nested content")?;
        assert!(nested_file_path.exists());

        // Test listing directory contents
        let entries: Vec<String> = fs::read_dir(&mount_point)?
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(entries.contains(&"test_file.txt".to_string()));
        assert!(entries.contains(&"test_dir".to_string()));

        // Test deleting a file
        fs::remove_file(&file_path)?;
        assert!(!file_path.exists());

        // Test deleting a directory (must be empty)
        fs::remove_file(&nested_file_path)?; // Remove nested file first
        fs::remove_dir(&dir_path)?;
        assert!(!dir_path.exists());

        // Send shutdown signal
        shutdown_tx.send(()).unwrap();

        // Give FUSE time to unmount
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Verify unmount
        let unmount_result = fs::read_dir(&mount_point);
        assert!(unmount_result.is_err(), "Mount point should be empty or inaccessible after unmount");

        Ok(())
    }

    #[tokio::test]
    async fn test_vfs_unmount_explicit() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let mount_point = temp_dir.path().join("vfs_mount_explicit");
        let mount_point_str = mount_point.to_str().unwrap();

        let fs_arc = VirtualFileSystem::new();
        let mount_options = vec![
            MountOption::FSName("warp_vfs_explicit".to_string()),
            MountOption::NoDev,
            MountOption::NoSuid,
            MountOption::AllowOther,
        ];

        // Mount without using the oneshot channel for shutdown
        let mount_point_path_clone = mount_point.clone();
        let fs_arc_clone = Arc::clone(&fs_arc);
        let mount_handle = tokio::task::spawn_blocking(move || {
            let mount_result = fuser::mount2(fs_arc_clone, &mount_point_path_clone, &mount_options);
            if let Err(e) = mount_result {
                error!("Failed to mount FUSE filesystem in explicit test: {}", e);
            }
        });

        tokio::time::sleep(Duration::from_millis(500)).await; // Give FUSE time to mount

        // Create a file to ensure it's mounted
        let file_path = mount_point.join("explicit_file.txt");
        fs::write(&file_path, "Explicit test")?;
        assert!(file_path.exists());

        // Explicitly unmount
        unmount_virtual_filesystem(mount_point_str).await?;

        // Give time for unmount to propagate
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Verify unmount
        let unmount_result = fs::read_dir(&mount_point);
        assert!(unmount_result.is_err(), "Mount point should be empty or inaccessible after explicit unmount");

        // Abort the mount handle as it's now detached
        mount_handle.abort();
        let _ = mount_handle.await;

        Ok(())
    }
}
