mod cli_args;
use cli_args::cli_args;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filepath = args.get(0).cloned(); // Gets the first argument if any, otherwise None

    cli_args(filepath);
}
