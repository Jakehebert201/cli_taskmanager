use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, stdin, Read, Write};
use std::path::Path;

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

pub fn get_csv(filepath: &str) {
    println!("Filepath: {}", filepath);

    // Check if the file already exists and has content
    let file_exists = Path::new(filepath).exists();
    let file_is_empty = if file_exists {
        match std::fs::metadata(filepath) {
            Ok(metadata) => metadata.len() == 0,
            Err(_) => true, // Assume empty if there's an error getting metadata
        }
    } else {
        true // File doesn't exist, so it's "empty"
    };

    // Open the file for appending (or create it if it doesn't exist)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filepath)
        .expect("Failed to open file for appending");

    // If the file is new or empty, write the header
    if file_is_empty {
        let header = "Task,Description,Due Date\n";
        file.write_all(header.as_bytes())
            .expect("Failed to write header to file");
    }

    // The rest of your function remains the same
    let mut contents = Vec::new();
    println!("What task would you like to do?");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    contents.push(input.trim().to_string());
    input.clear();

    println!("What is the task description?");
    stdin().read_line(&mut input).expect("Failed to read line");
    contents.push(input.trim().to_string());
    input.clear();

    println!("What is the due date?");
    stdin().read_line(&mut input).expect("Failed to read line");
    contents.push(input.trim().to_string());

    // Format the contents as a CSV row and write to the file
    let contents = contents.join(",") + "\n"; // Add a newline at the end for CSV formatting
    file.write_all(contents.as_bytes())
        .expect("Failed to write to file");

    println!("Successfully wrote to {}", filepath);
}
