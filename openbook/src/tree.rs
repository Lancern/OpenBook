//! This module defines the document tree of an OpenBook project.
//!
//! Each OpenBook project can be parsed into a tree structure like the following:
//!
//! ```text
//! GlobalizedBooks
//! |- BookConfig { ... }
//! |- Book [lang="en"]
//! |  |- BookConfig { ... }
//! |  |- Chapter [name="User Manual"]
//! |  |  |- Section [name="What is OpenBook"]
//! |  |  |- Section [name="How to use OpenBook"]
//! |  |     |- Section [name="How to build"]
//! |  |- Chapter [name="Developer Manual"]
//! |     |- Section [name="How to contribute"]
//! |- Book [lang="zh"]
//!    |- BookConfig { ... }
//!    |- Chapter [name="用户手册"]
//!    |  |- Section [name="什么是 OpenBook"]
//!    |  |- Section [name="如何使用 OpenBook"]
//!    |     |- Section [name="如何构建"]
//!    |- Chapter [name="开发者手册"]
//!       |- Section [name="如何贡献"]
//! ```
//!
//! The root of the project tree is represented by [`GlobalizedBooks`] struct. It is a container for
//! all books written in different languages. A book written in a specific language is represented
//! by the [`Book`] struct. Each book has its own logical structure represented by [`Chapter`]s and
//! [`Section`]s. In sections are the book's actual content.
//!
//! [`GlobalizedBooks`]: struct.GlobalizedBooks.html
//! [`Book`]: struct.Book.html
//! [`Chapter`]: struct.Chapter.html
//! [`Section`]: struct.Section.html
//!

use std::path::PathBuf;

/// The root of the OpenBook project tree.
///
/// This is the container for all books contained in the project that are written in different
/// natural languages.
#[derive(Clone, Debug, Default)]
pub struct GlobalizedBooks {
    /// The global configuration.
    pub config: BookConfig,

    /// All books contained in this project.
    ///
    /// The first field of an element represents the name of the natural language in which the
    /// book is written. The second field of an element is the book's node.
    pub books: Vec<(String, Book)>,
}

/// A book written in some language.
#[derive(Clone, Debug, Default)]
pub struct Book {
    /// The local configuration.
    ///
    /// Entries in local configuration overwrite the corresponding entries given in the global
    /// configuration.
    pub config: BookConfig,

    /// All chapters contained in this book.
    pub chapters: Vec<Chapter>,
}

/// Book configuration.
///
/// The configuration is separated into global configuration and local configuration. Entries in the
/// local configuration overwrite the corresponding entries in the local configuration.
#[derive(Clone, Debug, Default)]
pub struct BookConfig {
    /// Path to the root directory of the book.
    pub root: PathBuf,

    /// Structural configuration of the book.
    ///
    /// This configuration entry sets the path to the 4 special document in a book: `README.md`,
    /// `SUMMARY.md`, `GLOSSARY.md` and `LANGS.md`.
    pub structure: BookStructureConfig,

    /// Title of the book.
    pub title: Option<String>,

    /// Description of the book.
    pub description: Option<String>,

    /// Author of the book.
    pub author: Option<String>,

    /// ISBN of the book.
    pub isbn: Option<String>,

    /// Language of the book.
    ///
    /// This field should be encoded in
    /// [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
    pub language: Option<String>,

    /// Text direction of the book.
    pub direction: Option<TextDirection>,
}

/// Book structural configurations.
///
/// This configuration provides the paths to the 4 special document in a book:
///
/// * `README.md`, which is the "first page" within a book;
/// * `SUMMARY.md`, which can be used for providing an alternate table of contents to the book;
/// * `GLOSSARY.md`, which provides a list of terms to annotate;
/// * `LANGS.md`, which provides a description of the mapping from language names to books that are
/// written in that language.
#[derive(Clone, Debug, Default)]
pub struct BookStructureConfig {
    /// Path to the `README.md` file.
    pub readme: Option<PathBuf>,

    /// Path to the `SUMMARY.md` file.
    pub summary: Option<PathBuf>,

    /// Path to the `GLOSSARY.md` file.
    pub glossary: Option<PathBuf>,

    /// Path to the `LANGS.md` file.
    pub languages: Option<PathBuf>,
}

/// Text direction of a book.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection {
    /// Left to right.
    Ltr,

    /// Right to left.
    Rtl,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Ltr
    }
}

/// A chapter within a book.
#[derive(Clone, Debug, Default)]
pub struct Chapter {
    /// Name of the chapter.
    pub name: String,

    /// All subsections in this chapter.
    pub sections: Vec<Section>,
}

/// A section within a chapter.
#[derive(Clone, Debug, Default)]
pub struct Section {
    /// Path to the file that contains the content in this section.
    pub file: PathBuf,

    /// Name of the section.
    pub name: String,

    /// Content in the section.
    pub content: String,

    /// All subsections of this section.
    pub subsections: Vec<Section>,
}
