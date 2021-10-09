use rust_grep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(error) = rust_grep::run(config) {
        println!("Error reading file: {}", error);
        process::exit(1);
    }
}
