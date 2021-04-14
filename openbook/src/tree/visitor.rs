//! This module defines a visitor type for traversing the OpenBook document tree.
//!
//! OpenBook document tree visitors should implement the [`Visitor`] trait and implement all of its
//! three associative functions:
//! * `visit_globalized_books`: This function will be called when traversing a [`GlobalizedBooks`]
//! node;
//! * `visit_book`: This function will be called when traversing a [`Book`] node;
//! * `visit_section`: This function will be called when traversing a [`Section`] node.
//!
//! The OpenBook document tree will be traversed in a **depth-first pre-order traverse** manner.
//!
//! To start traversing from a node, you can call the `visit` function:
//!
//! ```ignore
//! let node = parse_tree();
//! let mut visitor = create_visitor();
//! visit(&node, &mut visitor);
//! ```
//!
//! [`GlobalizedBooks`]: ../struct.GlobalizedBooks.html
//! [`Book`]: ../struct.Book.html
//! [`Section`]: ../struct.Section.html
//! [`Visitor`]: trait.Visitor.html
//!

use crate::tree::{Book, GlobalizedBooks, Section};

/// OpenBook document tree visitors. User-defined visitors should implement this trait.
pub trait Visitor {
    /// Visit the given [`GlobalizedBooks`] node.
    ///
    /// [`GlobalizedBooks`]: ../struct.GlobalizedBooks.html
    fn visit_globalized_books(&mut self, globalized_books: &GlobalizedBooks);

    /// Visit the given [`Book`] node.
    ///
    /// [`Book`]: ../struct.Book.html
    fn visit_book(&mut self, book: &Book);

    /// Visit the given [`Section`] node.
    ///
    /// [`Section`]: ../struct.Section.html
    fn visit_section(&mut self, section: &Section);
}

/// Extension trait for types that supports `visit` operation. In normal cases, users should not use
/// this trait directly.
pub trait VisitorHost {
    /// Visit this object with the specified visitor.
    fn visit<V: Visitor>(&self, visitor: &mut V);
}

impl VisitorHost for GlobalizedBooks {
    fn visit<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_globalized_books(self);
        for (_, book) in &self.books {
            book.visit(visitor);
        }
    }
}

impl VisitorHost for Book {
    fn visit<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_book(self);
        self.preface.visit(visitor);
        for s in &self.sections {
            s.visit(visitor);
        }
    }
}

impl VisitorHost for Section {
    fn visit<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_section(self);
        for s in &self.subsections {
            s.visit(visitor);
        }
    }
}

/// Visit the specified [`VisitorHost`] object with the specified visitor.
///
/// [`VisitorHost`]: trait.VisitorHost.html
pub fn visit<H: VisitorHost, V: Visitor>(host: &H, visitor: &mut V) {
    host.visit(visitor);
}
