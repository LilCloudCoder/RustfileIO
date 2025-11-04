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
fileio = ">=0.1.2"
```

## Quick start
```rust
use fileio::file;

fn main() -> std::io::Result<()> {
    let f = file("src/example.txt");
    f.create_if_missing()?;

    // Overwrite with fresh content
    f.write_lines(["alpha", "beta", "gamma"])?;

    // Insert and update lines (1-based)
    f.insert_line(1, "start")?;
    f.write_line(3, "BETA")?;

    // Append more
    f.append_lines(["delta", "epsilon"])?;

    // Read a range and print
    let first_three = f.read_range(1, 3)?;
    println!("first_three: {:?}", first_three);

    Ok(())
}
```

## Examples
See `src/main.rs` for a more comprehensive example demonstrating inserts, deletes, find/replace, and counts.

## Safety and portability
- Uses a relative path in examples so it works on any machine.
- Validates line numbers are 1-based and returns clear `InvalidInput` errors.
- Missing files are handled gracefully with helpers like `.create_if_missing()` and `.is_empty()`.
