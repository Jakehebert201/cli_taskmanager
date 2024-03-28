use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

pub fn cli_args(args: &Option<String>) -> String {
    // Determine the filename: use the provided argument or a default value
    let filename = args
        .as_ref()
        .unwrap_or(&"taskmanager.csv".to_string())
        .clone();
    let path = Path::new(&filename);
    let display = path.display();

    // Check if the file exists, and create it if it doesn't
    if fs::metadata(&path).is_ok() {
        println!("Opening file: {}", display);
        // Add logic here if you need to do anything specific with the existing file
    } else {
        println!("File does not exist, creating {}", display);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
    }

    // Return the determined filename
    filename
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
