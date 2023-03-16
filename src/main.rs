use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

// Test ubuntu push
fn main() {
    // Capture args from the cli input and collect them into a vector
    let args: Vec<String> = env::args().collect();
    let link = &args[1];
    let tag = &args[2];
    let home_path = env::var("HOME").expect("$HOME Not Found");
    let file_path = format!("{}/devjon/projects/rust/keepit/LINKS.md", home_path);
    // Prepare the link for markdown format
    let readme_formatted_link = format!("- [{}]({})\n", tag, link);

    // Open file and prep it for writing, appending, and creation if not existant.
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file");

    // Write the cli arguments to the file
    file.write_all(readme_formatted_link.as_bytes()).expect("Failed to write to file");
}

