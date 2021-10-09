use std::error::Error;
use std::fs;

pub struct Config {
	pub filename: String,
	pub query: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			return Err("Not enough arguments");
		}

		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Config { filename, query })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.filename)?;
	println!("File read:\n{}", content);
	Ok(())
}
