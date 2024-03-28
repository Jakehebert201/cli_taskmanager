use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn cli_args(args: Vec<String>) {
    // Check if no arguments are provided
    if args.is_empty() {
        println!("No arguments provided");
        return;
    }

    let filename = &args[0];
    let path = Path::new(filename);
    let display = path.display();

    // Check if file exists, if it doesn't, create and write to it
    if std::fs::metadata(&path).is_ok() {
        println!("File exists");
    } else {
        println!("Creating file: {}", display);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match file.write_all(b"Hello, world!") {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("Successfully wrote to {}", display),
        }
    }
}
