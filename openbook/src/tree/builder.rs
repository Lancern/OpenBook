//! This module defines builder types for building the nodes in an OpenBook document tree.
//!

use crate::tree::{Book, BookConfig, GlobalizedBooks};

/// Build [`GlobalizedBooks`] nodes in a declarative way.
///
/// [`GlobalizedBooks`]: ../struct.GlobalizedBooks.html
#[derive(Clone, Debug)]
pub struct GlobalizedBooksBuilder {
    config: Option<BookConfig>,
    books: Vec<(String, Book)>,
}

impl GlobalizedBooksBuilder {
    /// Create a new `GlobalizedBooksBuilder` instance.
    pub fn new() -> Self {
        Self {
            config: None,
            books: Vec::new(),
        }
    }

    /// Set the book's configuration.
    pub fn set_config(mut self, config: BookConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Add a [`Book`] node as a child of the [`GlobalizedBooks`] node under build. The book is not
    /// associated to any languages.
    ///
    /// [`Book`]: ../struct.Book.html
    /// [`GlobalizedBooks`]: ../struct.GlobalizedBooks.html
    pub fn add_default_book(mut self, book: Book) -> Self {
        self.books.push((String::new(), book));
        self
    }

    /// Add a [`Book`] node as a child of the [`GlobalizedBooks`] node under build. The book is
    /// associated with the specified language.
    ///
    /// [`Book`]: ../struct.Book.html
    /// [`GlobalizedBooks`]: ../struct.GlobalizedBooks.html
    pub fn add_localized_book(mut self, language: String, book: Book) -> Self {
        self.books.push((language, book));
        self
    }

    /// Build the [`GlobalizedBooks`] object.
    ///
    /// [`GlobalizedBooks`]: ../struct.GlobalizedBooks.html
    pub fn build(self) -> GlobalizedBooks {
        GlobalizedBooks {
            config: self.config.unwrap_or_default(),
            books: self.books,
        }
    }
}
