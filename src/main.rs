use std::io;
fn main() {
    //take arguments and store them in a vector
    let _args: Vec<_> = std::env::args().collect();

    for arg in _args.iter() {
        println!("{}", arg);
    }
}
