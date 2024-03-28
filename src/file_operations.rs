use std::fs::File;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Read;
use std::io::{self, Write};
use std::path::Path; // Import the Read trait

pub fn write_to_file(filepath: &str) {
    println!("Filepath: {}", filepath);

    // Open the file in append mode. If the file does not exist, it will be created.
    let mut file = match OpenOptions::new()
        .append(true) // Set the option to append
        .create(true) // This option means the file will be created if it does not exist
        .open(filepath)
    {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(file) => file,
    };

    // Prompt the user for input
    println!("Enter text to write to file: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Write the user input to the file
    match file.write_all(input.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", filepath, why),
        Ok(_) => println!("Successfully wrote to {}", filepath),
    }
}

pub fn read_from_file(filepath: &str) -> Result<(), String> {
    let contents = fs::read_to_string(filepath)
        .map_err(|why| format!("couldn't read {}: {}", filepath, why))?;
    println!("Contents of {}: \n{}", filepath, contents);
    Ok(())
}
