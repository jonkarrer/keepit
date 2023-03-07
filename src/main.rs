use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn main() {
    // Capture args from the cli input and collect them into a vector
    let args: Vec<String> = env::args().collect();
    let link = &args[1];
    let tag = &args[2];

    // Prepare the link for markdown format
    let formatted_link = format!("- [{}]({})\n", tag, link);

    // Open file and prep it for writing, appending, and creation if not existant.
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("/Users/jkarrer/dev/projects/personal/rust/keepit/README.md")
        .expect("Failed to open file");

    // Write the cli arguments to the file
    file.write_all(formatted_link.as_bytes()).expect("Failed to write to file");

    // Run git commands to commit to a repo.
    Command::new("git")
        .arg("add")
        .arg(".")
        .status()
        .expect("Failed git add");
    
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("added new link")
        .status()
        .expect("Failed to commit");
    
    Command::new("git")
        .arg("push")
        .status()
        .expect("Failed to push");
}

