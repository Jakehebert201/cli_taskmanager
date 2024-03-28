mod cli_args;
mod file_operations;
use cli_args::cli_args;
use file_operations::{get_csv, read_from_file, write_to_file};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    // This now directly uses the filename returned by cli_args
    let filename = cli_args(&args.get(0).cloned());

    loop {
        println!("Enter 1 to write to file, 2 to read from file, or 3 to exit: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match input {
            1 => get_csv(&filename),
            2 => {
                if let Err(e) = read_from_file(&filename) {
                    println!("An error occurred while reading from the file: {}", e);
                }
            }
            3 => break,
            _ => println!("Invalid input. Please enter a number between 1 and 3."),
        }
    }
}
