use rustyline::error::ReadlineError;
use std::fs;

mod clauses;
mod parser;
mod types;

use crate::clauses::cnf::Clausifier;
use crate::parser::parse_gic_file;

use std::env;

fn main() {
	let mut rl = rustyline::DefaultEditor::new().unwrap();
	let mut clausifier = Clausifier::new();

	let cwd = env::current_dir().unwrap_or_else(|_| {
		eprintln!("Error getting current directory, using default.");
		env::current_dir().unwrap()
	});
	println!("Welcome to the IGIC REPL! Type 'exit' or 'quit' to leave.");
	println!("Current directory: {:?}", cwd);
	loop {
		let readline = rl.readline("igic> ");
		match readline {
			Ok(line) => {
				let input = line.trim();

				if input.is_empty() {
					continue;
				}

				//rl.add_history_entry(input);

				if input.eq_ignore_ascii_case("exit()") || input.eq_ignore_ascii_case("quit()") {
					break;
				}

				let mut parts = input.split_whitespace();
				let command = parts.next().unwrap_or("");

				match command {
					"consult" => {
						let filename = cwd.join(parts.next().unwrap_or(""));

						// Check if the filename ends with .gic
						if filename.extension().is_none() || filename.extension().unwrap() != "gic"
						{
							eprintln!("Error: File must have a .gic extension.");
							continue;
						}

						match fs::read_to_string(&filename) {
							Ok(content) => match parse_gic_file(&content) {
								Ok(expressions) => {
									for (i, expr) in expressions.iter().enumerate() {
										println!("--- Expression {} ---", i + 1);
										println!("{}", expr);
									}
									for expr in expressions {
										if let Err(e) = clausifier.clausify(expr) {
											eprintln!("Error clausifying: {}", e);
										}
									}
									println!("Current CNF Program:");
									println!("{}", clausifier.get_program());
								},
								Err(e) => eprintln!("Parse error: {}", e),
							},
							Err(e) => {
								eprintln!(
									"Error reading file '{}': {}",
									filename.to_string_lossy(),
									e
								)
							},
						}
					},
					_ => println!(
						"Unknown command: '{}'. Use 'consult -p <file>', 'exit', or 'quit'.",
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
}
