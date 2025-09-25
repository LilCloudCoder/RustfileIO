use fileio::file;

fn main() {
    let f = file("/Users/lilcoder/life/mastery/rust/fileIO/src/example.txt"); // make sure path is writable

    f.append("Line 1").unwrap();
    f.append("Line 2").unwrap();

    f.write_line(2, "Updated line 2").unwrap();
    f.insert_line(1, "Inserted line 1").unwrap();

    println!("File content:");
    for line in f.read_lines().unwrap() {
        println!("{}", line);
    }
}