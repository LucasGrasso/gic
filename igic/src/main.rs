use std::env;
use std::fs;

use crate::clauses::cnf::Clausifier;
use crate::parser::{parse_formula, parse_gic_file};
use rustyline::error::ReadlineError;

mod clauses;
mod mgu;
mod parser;
mod types;

fn main() {
	let mut rl = rustyline::DefaultEditor::new().unwrap();
	let mut clausifier = Clausifier::new();

	let cwd = env::current_dir().unwrap_or_else(|_| {
		eprintln!("Error getting current directory, using default.");
		env::current_dir().unwrap()
	});
	println!("Welcome to the IGIC REPL! Type 'exit' or 'quit' to leave.");
	println!("Current directory: {:?}", cwd);
	let history_path = "igic_history.txt";
	if rl.load_history(history_path).is_err() {
		println!("No previous history.");
	}
	loop {
		let readline = rl.readline("igic> ");
		match readline {
			Ok(line) => {
				let input = line.trim();

				if input.is_empty() {
					continue;
				}

				let _ = rl.add_history_entry(input);

				if input.eq_ignore_ascii_case("exit()") || input.eq_ignore_ascii_case("quit()") {
					break;
				}

				let mut parts = input.split_whitespace();
				let command = parts.next().unwrap_or("");

				match command {
					"consult" => consult_cmd(&mut clausifier, &cwd, parts.next().unwrap_or("")),
					"query" => {
						use regex::Regex;
						let rest_of_line = parts.collect::<Vec<&str>>().join(" ");

						let re = Regex::new(r#""(.*?)""#).unwrap();
						if let Some(caps) = re.captures(&rest_of_line) {
							let query_input = caps.get(1).unwrap().as_str();
							println!("Processing query: {}", query_input);
							query_cmd(&mut clausifier, query_input);
						} else {
							eprintln!("Error: Query must be wrapped in double quotes, like: query \"<formula>\"");
						}
					},
					_ => println!(
						"Unknown command: '{}'. Use 'consult <file>', 'exit', or 'quit'.",
						command
					),
				}
			},
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C pressed, exiting.");
				break;
			},
			Err(ReadlineError::Eof) => {
				println!("CTRL-D pressed, exiting.");
				break;
			},
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			},
		}
	}
	rl.save_history(history_path).unwrap();
}

fn consult_cmd(clausifier: &mut Clausifier, cwd: &std::path::Path, input: &str) {
	let filename = cwd.join(input);

	// Check if the filename ends with .gic
	if filename.extension().is_none() || filename.extension().unwrap() != "gic" {
		eprintln!("Error: File must have a .gic extension.");
		return;
	}

	match fs::read_to_string(&filename) {
		Ok(content) => match parse_gic_file(&content) {
			Ok(expressions) => {
				for (i, expr) in expressions.iter().enumerate() {
					println!("--- Expression {} ---", i + 1);
					println!("{}", expr);
				}
				for expr in expressions {
					if let Err(e) = clausifier.add_to_progam(expr) {
						eprintln!("Error clausifying: {}", e);
					}
				}
				println!("Current CNF Program:");
				println!("{}", clausifier.get_program());
			},
			Err(e) => eprintln!("Parse error: {}", e),
		},
		Err(e) => {
			eprintln!("Error reading file '{}': {}", filename.to_string_lossy(), e)
		},
	}
}

fn query_cmd(clausifier: &mut Clausifier, input: &str) {
	let query = input.trim();

	if query.is_empty() {
		eprintln!("Error: Query cannot be empty.");
		return;
	}

	let mut formula = query.to_string();
	formula.push(';');

	match parse_formula(&formula) {
		Ok(expr) => match clausifier.clausify(types::ast::Expression::Not((Box::new(expr)))) {
			Ok(goal) => {
				let refutable =
					mgu::resolution::sld_resolution(clausifier.get_program().clone(), goal);
				if refutable {
					println!("The query is refutable.");
				} else {
					println!("The query is not refutable.");
				}
			},
			Err(e) => eprintln!("Error clausifying query: {}", e),
		},
		Err(e) => {
			eprintln!("Parse error: {}", e);
		},
	}
}
