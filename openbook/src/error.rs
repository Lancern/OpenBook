//! This module defines the error struct used in OpenBook.
//!

use std::fmt::{Display, Formatter};

/// OpenBook error struct.
///
/// Errors in OpenBook come from two sources:
///
/// * Errors that come from the dependency of OpenBook, which are represented by the `Inner`
/// variant;
/// * Errors that come directly from OpenBook components, which are represented by the `Msg`
/// variant.
#[derive(Clone, Debug)]
pub enum Error {
    Inner(Box<dyn std::error::Error>),
    Msg(String),
}

impl Error {
    pub fn from_inner<E: std::error::Error>(inner: E) -> Self {
        Self::Inner(Box::new(inner))
    }

    pub fn from_message<M: Into<String>>(msg: M) -> Self {
        Self::Msg(msg.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inner(inner) => f.write_fmt(format_args!("OpenBook inner error: {}", inner)),
            Self::Msg(msg) => f.write_fmt(format_args!("OpenBook error: {}", msg)),
        }
    }
}

/// Result type used in OpenBook.
pub type Result<T> = std::result::Result<T, Error>;
