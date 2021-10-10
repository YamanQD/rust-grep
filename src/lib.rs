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
	for line in search_case_sensitive(&config.query, &content) {
		println!("{}", line);
	}
	Ok(())
}

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
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
	fn case_sensitive() {
		let query = "oses";

		let content = "\
Roses are red, violets are blue
This is only for testing purposes
But so are you";

		assert_eq!(
			search_case_sensitive(query, content),
			vec![
				"Roses are red, violets are blue",
				"This is only for testing purposes"
			]
		);

		let content = "\
Roses are red, violets are blue
This is only for testing purpOsEs
But so are you";

		assert_eq!(
			search_case_sensitive(query, content),
			vec!["Roses are red, violets are blue"]
		);
	}
}
