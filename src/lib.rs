//! FileIO crate: fluent file operations in Rust
//!
//! Quick start:
//! ```rust
//! use fileio::file;
//!
//! let f = file("example.txt");
//! f.write_line(1, "Hello world!").unwrap();
//! ```

pub mod fileio;

pub use fileio::FileIO;
pub use fileio::file;