# FileIO

Fluent file operations in Rust: read, write, append, replace, insert, and manipulate lines with a simple API.

## Features

- Reading
  - `.read_all()` → Read entire file as a `String`
  - `.read_lines()` → Read file line by line (`Vec<String>`)
  - `.read_non_empty_lines()` → Read non-empty trimmed lines
  - `.read_range(start, end)` → Read a 1-based inclusive range of lines
  - `.count_lines()` → Count lines efficiently
- Writing
  - `.write(content)` → Overwrite the whole file
  - `.write_lines(iter)` → Overwrite with many lines
  - `.append(content)` → Append a line at the end
  - `.append_lines(iter)` → Append multiple lines
  - `.write_line(n, content)` → Replace a specific line (1-based)
  - `.insert_line(n, content)` → Insert a line without overwriting
  - `.insert_lines(n, iter)` → Insert multiple lines at a position
  - `.remove_line(n)` / `.remove_lines(start, end)` → Remove one or many lines
  - `.find_replace(find, replace)` → Replace all occurrences in the whole file
- Utilities
  - `.exists()` → Check if file exists
  - `.create_if_missing()` → Create an empty file if it doesn’t exist
  - `.is_empty()` → Whether the file is empty or missing
  - `file(path)` → Convenience constructor

Under the hood, `BufReader`/`BufWriter` are used for good performance, and `PathBuf` is used for path robustness. Backwards compatibility is kept.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fileio = "...."
```

### Usage:
```rust
use fileio::file;

fn main() {
    let f = file("/full/path/to/file/example.txt");

    // Append a line
    f.append("This is a new line!").unwrap();

    // Replace line 2
    f.write_line(2, "Updated line 2").unwrap();

    // Insert a line at line 1
    f.insert_line(1, "Inserted line 1").unwrap();

    // Read and print all lines
    for line in f.read_lines().unwrap() {
        println!("{}", line);
    }

    // Read the entire file as string
    let content = f.read_all().unwrap();
    println!("Whole file:\n{}", content);
}
```
