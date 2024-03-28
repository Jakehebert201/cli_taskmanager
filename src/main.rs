use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod cli_args;
use cli_args::cli_args;

fn main() {
    // Take arguments and store them in a vector, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // Call the cli_args function with the arguments
    cli_args(args);
}
