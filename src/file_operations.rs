use csv::Reader;
use csv::{Error, ReaderBuilder, WriterBuilder};
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
        let header = "Task Number,Task,Description,Due Date\n";
        file.write_all(header.as_bytes())
            .expect("Failed to write header to file");
    }
    let mut contents = Vec::new();
    //read first field to get the number of tasks, then increment by 1
    let csv_contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let mut task_count = 0;
    for line in csv_contents.lines() {
        task_count += 1;
    }
    contents.push(task_count.to_string());

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

pub fn delete_task(filepath: &str, task_number_to_delete: u32) -> Result<(), Error> {
    let mut rdr = ReaderBuilder::new().from_path(filepath)?;
    let records: Result<Vec<_>, _> = rdr.records().collect();
    let mut records = records?;

    // Convert to a more manageable structure
    let mut data: Vec<Vec<String>> = records
        .iter()
        .map(|r| r.iter().map(|s| s.to_string()).collect())
        .collect();

    // Filter and adjust task numbers
    let mut found = false;
    let mut data_filtered: Vec<Vec<String>> = Vec::new();
    let mut current_num = 1;

    for row in data.into_iter() {
        if let Ok(num) = row[0].parse::<u32>() {
            if num != task_number_to_delete {
                let mut new_row = row;
                new_row[0] = current_num.to_string(); // Assign the current task number
                data_filtered.push(new_row);
                current_num += 1;
            } else {
                found = true; // Skip the task to delete, but mark as found
            }
        } else {
            // If the row doesn't start with a valid number, include it as is
            data_filtered.push(row);
        }
    }

    // Rewrite the CSV file with the updated records
    if found {
        let mut wtr = WriterBuilder::new().from_writer(File::create(filepath)?);
        for record in data_filtered {
            wtr.write_record(&record)?;
        }
        println!("Task {} deleted successfully.", task_number_to_delete);
    } else {
        println!("Task number {} not found.", task_number_to_delete);
    }

    Ok(())
}

pub fn delete_task_interaction(filepath: &str) -> io::Result<()> {
    if let Err(e) = read_from_file(filepath) {
        println!("An error occurred while reading from the file: {}", e);
    }
    println!("Enter the number of the task you want to delete:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let task_number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return Ok(()); // Early return if invalid input
        }
    };

    // Now call the delete_task function with the user-provided task number
    if let Err(e) = delete_task(filepath, task_number) {
        println!("An error occurred: {}", e);
    }

    Ok(())
}

// Ensure you have the `delete_task` function defined as shown previously or appropriately imported
