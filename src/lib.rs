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
	for line in search(&config.query, &content) {
		println!("{}", line);
	}
	Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut result = Vec::new();

	for line in content.lines() {
		if line.contains(query) {
			result.push(line);
		}
	}
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let content = "\
Roses are red, violets are blue
This is only for testing purposes
But so are you";
		let query = "for";

		assert_eq!(
			search(query, content),
			vec!["This is only for testing purposes"]
		);
	}

	#[test]
	fn two_results() {
		let content = "\
Roses are red, violets are blue
This is only for testing purposes
But so are you";
		let query = "oses";

		assert_eq!(
			search(query, content),
			vec![
				"Roses are red, violets are blue",
				"This is only for testing purposes"
			]
		);
	}
}
