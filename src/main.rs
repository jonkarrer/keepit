use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let link = &args[1];
    let tag = &args[2];

    let formatted_link = format!("[{}]({})\n", tag, link);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("/Users/jkarrer/dev/projects/personal/rust/keepit/README.md")
        .expect("Failed to open file");


    file.write_all(formatted_link.as_bytes()).expect("Failed to write to file");
}

