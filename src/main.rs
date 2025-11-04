use fileio::file;

fn main() {
    // Use a relative path inside the project to keep the example portable
    let f = file("src/example.txt");

    // Ensure the file exists
    f.create_if_missing().expect("failed to create example file");

    // Start with clean content
    f.write("Start").expect("write failed");

    // Demonstrate appends
    f.append("Line 1").unwrap();
    f.append("Line 2").unwrap();

    // Replace a specific line
    f.write_line(2, "Updated line 2").unwrap();

    // Insert a new line at the top
    f.insert_line(1, "Inserted line 1").unwrap();

    // Insert multiple lines at position 3
    f.insert_lines(3, ["A", "B", "C"]).unwrap();

    // Remove line 5
    f.remove_line(5).unwrap();

    // Find & replace across the whole file
    f.find_replace("Line", "Ln").unwrap();

    // Show a range of lines
    let range = f.read_range(1, 4).unwrap();
    println!("First 4 lines: {:?}", range);

    // Count and display all lines
    println!("File has {} lines", f.count_lines().unwrap());

    println!("\nFile content:");
    for line in f.read_lines().unwrap() {
        println!("{}", line);
    }
}