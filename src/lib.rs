//! FileIO crate: fluent file operations in Rust
//!
//! Quick start:
//! ```rust
//! use fileio::file;
//!
//! let f = file("example.txt");
//! f.write_line(1, "Hello world!").unwrap();
//! assert_eq!(f.read_range(1, 1).unwrap(), vec!["Hello world!".to_string()]);
//! ```

pub mod fileio;

pub use fileio::FileIO;
pub use fileio::file;

/// Commonly used items
pub mod prelude {
    pub use crate::{file, FileIO};
}