mod cli_args;
mod file_operations;
use cli_args::cli_args;
use file_operations::read_from_file;
use file_operations::write_to_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filepath = args.get(0).cloned(); // Gets the first argument if any, otherwise None
    cli_args(filepath);
}
