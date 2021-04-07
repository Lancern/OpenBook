//! This module provides a platform independent definition of a file system that supports the
//! operations needed by OpenBook.
//!
//! OpenBook requires the following operations to be supported by the file system:
//! * Read a file;
//! * List all files under a directory;
//! * Watch for file changes. The following file system events will be watched:
//!   * A new file is created;
//!   * An existing file is deleted;
//!   * A file or directory is moved (renamed);
//!   * A file is written to.
//!
//! OpenBook relies on the [`FileSystem`] trait that provides the aforementioned file system
//! operations. The `local` submodule provides a [`FileSystem`] implementation that operates on the
//! local file system.
//!
//! [`FileSystem`]: trait.FileSystem.html
//!

pub mod local;

use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

use crate::error::{Error, Result};

/// A platform independent definition of a file system that supports the operations needed by
/// OpenBook.
pub trait FileSystem: Sync {
    /// Type of the directory iterator that iterates over all files under a certain directory.
    type DirIter: Iterator<Item = Result<PathBuf>>;

    /// Type of the file system watcher that emits events when the state of the file system changes.
    type Watcher: FileSystemWatcher;

    /// Read the whole content of the specified file as a string.
    fn read_file_as_string<P: AsRef<Path>>(&self, path: P) -> Result<String>;

    /// Create a `DirIter` that iterates over all files and subdirectories under the specified
    /// directory.
    ///
    /// Note that the returned iterator will not iterate the specified directory **recursively**.
    fn read_directory<P: AsRef<Path>>(&self, path: P) -> Result<Self::DirIter>;

    /// Create a file system watcher that emits events into the specified event sink.
    fn create_watcher(&self, event_sink: Box<dyn FileSystemEventSink>) -> Result<Self::Watcher>;
}

/// Watches state changes in the file system and emits corresponding events.
pub trait FileSystemWatcher {
    /// Watch the specified file system path for changes with the specified mode.
    ///
    /// For differences on different values of `mode`, please refer to [`FileSystemWatchMode`].
    ///
    /// [`FileSystemWatchMode`]: enum.FileSystemWatchMode.html
    fn watch<P: AsRef<Path>>(&self, path: P, mode: FileSystemWatchMode) -> Result<()>;
}

/// Specifies whether the file system watcher should watch the specified path recursively.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileSystemWatchMode {
    /// The file system watcher should only watch the specified path, excluding its containing files
    /// and subdirectories.
    Normal,

    /// The file system watcher should watch the whole directory at the specified path (if it is a
    /// directory), including its containing files and subdirectories.
    Recursive,
}

/// File system events emitted by a file system watcher.
#[derive(Debug)]
pub enum FileSystemEvent {
    /// A file is been created at the given path.
    Create(PathBuf),

    /// The file that was at the given path is been deleted.
    Delete(PathBuf),

    /// A file is been moved (renamed) from one location to another.
    Rename {
        /// The old path of the moved file.
        from: PathBuf,

        /// The new path of the moved file.
        to: PathBuf,
    },

    /// The file at the specified path is been written to.
    Write(PathBuf),

    /// An error occurred in the file system watcher.
    Error(Error, Option<PathBuf>),
}

/// File system watchers emit file system events into this sink.
pub trait FileSystemEventSink {
    /// Send the specified event into this sink.
    fn send(&self, event: FileSystemEvent) -> Result<()>;
}

impl FileSystemEventSink for Sender<FileSystemEvent> {
    fn send(&self, event: FileSystemEvent) -> Result<()> {
        self.send(event).map_err(Error::from_inner)
    }
}
