use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
	pub filename: String,
	pub query: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			return Err("Not enough arguments");
		}

		let query = args[1].clone();
		let filename = args[2].clone();

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config {
			filename,
			query,
			case_sensitive,
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.filename)?;

	let result = if config.case_sensitive {
		search_case_sensitive(&config.query, &content)
	} else {
		search_case_insensitive(&config.query, &content)
	};

	for line in result {
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

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut result = Vec::new();

	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
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

	#[test]
	fn case_insensitive() {
		let query = "oses";
		let content = "\
Roses are red, violets are blue
This is only for testing purpOsEs
But so are you";

		assert_eq!(
			search_case_insensitive(query, content),
			vec![
				"Roses are red, violets are blue",
				"This is only for testing purpOsEs"
			]
		);
	}
}
