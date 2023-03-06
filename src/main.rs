
// A command line app that will store my snippets, gists, and links into a repo.
// This will
// - Take an argument for the type of thing to store
// - Take an argument for the item, file, or text
// - Take an argument of some type of organization or group

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let link = &args[1];
    let tag = &args[2];
    dbg!(link, tag);

    let formatted_link = format!("[{}]({})", tag, link);
    fs::write("../README.md", formatted_link).expect("Failed to write to file");
}

