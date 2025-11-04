use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Fluent File I/O helper
#[derive(Debug, Clone)]
pub struct FileIO {
    path: PathBuf,
}

impl FileIO {
    /// Create a new FileIO instance
    pub fn new(path: &str) -> Self {
        Self { path: PathBuf::from(path) }
    }

    /// Alternative constructor accepting anything that can be referenced as a Path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Return the underlying path as &Path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check whether the file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Ensure the file exists (creates an empty file if missing)
    pub fn create_if_missing(&self) -> io::Result<()> {
        if !self.exists() {
            OpenOptions::new().create(true).write(true).open(&self.path)?;
        }
        Ok(())
    }

    /// Read entire file as a single String
    pub fn read_all(&self) -> io::Result<String> {
        std::fs::read_to_string(&self.path)
    }

    /// Read file line by line
    pub fn read_lines(&self) -> io::Result<Vec<String>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    /// Read only non-empty lines (trimmed)
    pub fn read_non_empty_lines(&self) -> io::Result<Vec<String>> {
        Ok(self
            .read_lines()?
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }

    /// Count number of lines in file (fast streaming)
    pub fn count_lines(&self) -> io::Result<usize> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        Ok(reader.lines().count())
    }

    /// Whether the file is empty or missing
    pub fn is_empty(&self) -> io::Result<bool> {
        match File::open(&self.path) {
            Ok(mut f) => {
                use std::io::Read;
                let mut buf = [0u8; 1];
                Ok(f.read(&mut buf).unwrap_or(0) == 0)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Overwrite the entire file with new content
    pub fn write(&self, content: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())
    }

    /// Write the entire file with the provided lines (joined with `\n`)
    pub fn write_lines<I, S>(&self, lines: I) -> io::Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut first = true;
        let mut joined = String::new();
        for line in lines {
            if !first { joined.push('\n'); }
            first = false;
            joined.push_str(line.as_ref());
        }
        self.write(&joined)
    }

    /// Append a line to the end of the file
    pub fn append(&self, content: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }

    /// Overwrite the entire file with new content
    pub fn write(&self, content: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path)?;
        file.write_all(content.as_bytes())
    }

    /// Write or replace a specific line (line_number starts from 1)
    pub fn write_line(&self, line_number: usize, content: &str) -> io::Result<()> {
        if line_number == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "line_number must be >= 1",
            ));
        }
        let mut lines = self.read_lines().unwrap_or_default();

        // Ensure enough lines exist
        if line_number > lines.len() {
            lines.resize(line_number, String::new());
        }

        // Replace the specific line
        lines[line_number - 1] = content.to_string();

        // Overwrite the file
        self.write(&lines.join("\n"))
    }

    /// Insert a line at a specific line number (pushes following lines down)
    pub fn insert_line(&self, line_number: usize, content: &str) -> io::Result<()> {
        if line_number == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "line_number must be >= 1",
            ));
        }
        let mut lines = self.read_lines().unwrap_or_default();

        if line_number > lines.len() + 1 {
            lines.resize(line_number - 1, String::new());
        }

        lines.insert(line_number - 1, content.to_string());

        self.write(&lines.join("\n"))
    }
}

/// Helper function for convenience
pub fn file(path: &str) -> FileIO {
    FileIO::new(path)
}