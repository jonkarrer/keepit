
// A command line app that will store my snippets, gists, and links into a repo.
// This will
// - Take an argument for the type of thing to store
// - Take an argument for the item, file, or text
// - Take an argument of some type of organization or group

use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let link = &args[1];
    let tag = &args[2];
    dbg!(link, tag);

    let formatted_link = format!("[{}]({})\n", tag, link);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("/Users/jkarrer/dev/projects/personal/rust/keepit/README.md")
        .expect("Failed to open file");


    file.write_all(formatted_link.as_bytes()).expect("Failed to write to file");
}

