use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

pub fn cli_args(args: Option<String>) {
    // Determine the filename: use the provided argument or a default value
    let filename = args.unwrap_or_else(|| "default.txt".to_string());
    let path = Path::new(&filename);
    let display = path.display();

    // Check if the file exists, and create it if it doesn't
    if fs::metadata(&path).is_ok() {
        println!("Opening file: {}", display);
        // Add logic here if you need to do anything specific with the existing file
    } else {
        println!("File does not exist, creating {}", display);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", why, display),
            Ok(file) => file,
        };
        // Example: Write some default content to the file
        match file.write_all(b"Hello, world!") {
            Err(why) => panic!("couldn't write to {}: {}", why, display),
            Ok(_) => println!("Successfully wrote to {}", display),
        }
    }
}
