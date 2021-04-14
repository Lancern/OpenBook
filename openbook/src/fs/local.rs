//! Provide an implementation of [`FileSystem`] that operates on the local file system.
//!
//! [`FileSystem`]: ..\trait.FileSystem.html
//!

use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::error::{Error, Result};
use crate::fs::{
    FileSystem, FileSystemEvent, FileSystemEventSink, FileSystemWatchMode, FileSystemWatcher,
};

/// An implementation of [`FileSystem`] that operates on the local file system.
pub struct LocalFileSystem;

impl LocalFileSystem {
    /// Create a new `LocalFileSystem` instance.
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for LocalFileSystem {
    type DirIter = LocalFileSystemIter;

    type Watcher = LocalFileSystemWatcher;

    fn has_file<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_file()
    }

    fn has_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_dir()
    }

    fn read_file_as_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        std::fs::read_to_string(path).map_err(Error::from_inner)
    }

    fn read_directory<P: AsRef<Path>>(&self, path: P) -> Result<Self::DirIter> {
        LocalFileSystemIter::new(path)
    }

    fn create_watcher(&self, event_sink: Box<dyn FileSystemEventSink>) -> Result<Self::Watcher> {
        LocalFileSystemWatcher::new(event_sink)
    }
}

/// Iterates over all files and subdirectories under a local directory.
pub struct LocalFileSystemIter {
    inner: ReadDir,
}

impl LocalFileSystemIter {
    /// Create a new `LocalFileSystemIter` instance that iterates over all files and subdirectories
    /// under the specified directory.
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let inner = std::fs::read_dir(dir).map_err(Error::from_inner)?;

        Ok(Self { inner })
    }
}

impl Iterator for LocalFileSystemIter {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|r| r.map(|e| e.path()).map_err(Error::from_inner))
    }
}

/// Filter out un-interesting file system events produced by the underlying `notify` crate.
fn filter_raw_fs_event(raw_event: DebouncedEvent) -> Option<FileSystemEvent> {
    match event {
        DebouncedEvent::Create(path) => Some(FileSystemEvent::Create(path)),
        DebouncedEvent::Remove(path) => Some(FileSystemEvent::Delete(path)),
        DebouncedEvent::Rename(from, to) => Some(FileSystemEvent::Rename { from, to }),
        DebouncedEvent::Write(path) => Some(FileSystemEvent::Write(path)),
        DebouncedEvent::Error(e, path) => Some(FileSystemEvent::Error(Error::from_inner(e), path)),
        _ => None,
    }
}

/// Watches file system state changes in the local file system.
pub struct LocalFileSystemWatcher {
    raw_watcher: Mutex<RecommendedWatcher>,
}

impl LocalFileSystemWatcher {
    /// Create a new `LocalFileSystemWatcher` instance that emits file system events into the
    /// specified event sink.
    pub fn new(event_sink: Box<dyn FileSystemEventSink>) -> Result<Self> {
        let (raw_events_send, raw_events_recv) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            loop {
                let event = match raw_events_recv.recv() {
                    Ok(e) => e,
                    Err(_) => return,
                };

                let user_event = filter_raw_fs_event(event);
                if let Some(e) = user_event {
                    event_sink.send(e).ok(); // Ignore all errors during event_sink.send
                }
            }
        });

        let raw_watcher = Mutex::new(
            notify::watcher(raw_events_send, Duration::new(0, 0)).map_err(Error::from_inner)?,
        );
        Ok(Self { raw_watcher })
    }
}

impl FileSystemWatcher for LocalFileSystemWatcher {
    fn watch<P: AsRef<Path>>(&self, path: P, mode: FileSystemWatchMode) -> Result<()> {
        let notify_mode = match mode {
            FileSystemWatchMode::Normal => RecursiveMode::NonRecursive,
            FileSystemWatchMode::Recursive => RecursiveMode::Recursive,
        };

        // TODO: refactor mutex lock code as a function to provide a single `expect` message.
        let mut lock = self.raw_watcher.lock().expect("mutex lock failed");
        lock.watch(path, notify_mode).map_err(Error::from_inner)
    }
}
