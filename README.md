# FileIO

Fluent file operations in Rust: read, write, append, replace, insert, and manipulate lines with a simple API.

## Features

- `.read_all()` → Read entire file as a String
- `.read_lines()` → Read file line by line
- `.append(content)` → Append a line at the end
- `.write(content)` → Overwrite the whole file
- `.write_line(line_number, content)` → Replace a specific line
- `.insert_line(line_number, content)` → Insert a line without overwriting

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
