use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub struct FileIO {
    path: String,
}

impl FileIO {
    pub fn new(path: &str) -> Self {
    Self { path: path.to_string() }
}

/// Read whole file
pub fn read_all(&self) -> io::Result<String> {
    std::fs::read_to_string(&self.path)
}

/// Read file line by line
pub fn read_lines(&self) -> io::Result<Vec<String>> {
    let file = File::open(&self.path)?;
let reader = BufReader::new(file);
Ok(reader.lines().collect::<Result<_, _>>()?)
}

/// Append text to the file
pub fn append(&self, content: &str) -> io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(&self.path)?;
writeln!(file, "{}", content)?;
Ok(())
}

/// Overwrite the whole file
pub fn write(&self, content: &str) -> io::Result<()> {
    std::fs::write(&self.path, content)
}

/// Write or replace a specific line (line_number starts from 1)
pub fn write_line(&self, line_number: usize, content: &str) -> io::Result<()> {
    let mut lines = self.read_lines().unwrap_or_default();

// Ensure enough lines exist
    if line_number > lines.len() {
    lines.resize(line_number, "".to_string());
}

// Replace the specific line
lines[line_number - 1] = content.to_string();

// Write back the whole file
self.write(&lines.join("\n"))
}
}

/// Helper function
pub fn file(path: &str) -> FileIO {
    FileIO::new(path)
}