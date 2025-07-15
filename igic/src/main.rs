use colored::*;
use std::env;
use std::fs;

use crate::clauses::cnf::Clausifier;
use crate::parser::{parse_formula, parse_gic_file};
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;

mod clauses;
mod libraries;
mod mgu;
mod parser;
mod resolution;
mod types;

fn main() {
	let mut rl = rustyline::DefaultEditor::new().unwrap();
	let mut clausifier = Clausifier::new();

	let cwd = env::current_dir().unwrap_or_else(|_| {
		eprintln!("Error getting current directory, using default.");
		env::current_dir().unwrap()
	});

	let progam_index = load_common_libraries(&mut clausifier, &cwd);

	println!("Welcome to the IGIC REPL! Type 'exit' or 'quit' to leave.");
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
					"load" => load_cmd(&mut clausifier, &cwd, parts.next().unwrap_or("")),
					"program" => {
						if clausifier.get_progam_length() - progam_index >= 1 {
							println!("{}", clausifier.to_str_from(progam_index));
						} else {
							eprint!("{}", "Error: ".red());
							eprintln!("No program loaded. Please load a .gic file first.");
						}
					},
					"query" => {
						use regex::Regex;
						let rest_of_line = parts.collect::<Vec<&str>>().join(" ");

						let re = Regex::new(r#""(.*?)""#).unwrap();
						if let Some(caps) = re.captures(&rest_of_line) {
							let query_input = caps.get(1).unwrap().as_str();
							query_cmd(&mut clausifier, query_input, &mut rl);
						} else {
							eprint!("{}", "Error: ".red());
							eprintln!(
								"Query must be wrapped in double quotes, like: query \"<formula>\""
							);
						}
					},
					_ => {
						eprint!("{}", "Error: ".red());
						eprintln!(
						"Unknown command: '{}'. Use 'load <file>', 'query \"<expr>\"', program, 'exit', or 'quit'.",
						command
					)
					},
				}
			},
			Err(ReadlineError::Interrupted) => {
				break;
			},
			Err(ReadlineError::Eof) => {
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

fn load_common_libraries(clausifier: &mut Clausifier, cwd: &std::path::Path) -> usize {
	let libraries = ["src\\libraries\\lists\\lists.gic"];
	for lib in libraries {
		let file = cwd.join(lib);
		match fs::read_to_string(file) {
			Ok(content) => match parse_gic_file(&content) {
				Ok(expressions) => {
					for expr in expressions {
						if let Err(e) = clausifier.add_to_program(expr) {
							eprintln!("{}", format!("Error loading library {}: {}", lib, e).red());
						}
					}
				},
				Err(e) => eprintln!("{}", format!("Parse error in library {}: {}", lib, e).red()),
			},
			Err(e) => eprintln!("Error reading library {}: {}", lib, e),
		}
	}
	clausifier.get_progam_length()
}

fn load_cmd(clausifier: &mut Clausifier, cwd: &std::path::Path, input: &str) {
	let filename = cwd.join(input);

	// Check if the filename ends with .gic
	if filename.extension().is_none() || filename.extension().unwrap() != "gic" {
		eprintln!("{}", "Error: File must have a .gic extension.".red());
		return;
	}

	match fs::read_to_string(&filename) {
		Ok(content) => match parse_gic_file(&content) {
			Ok(expressions) => {
				for expr in expressions {
					if let Err(e) = clausifier.add_to_program(expr) {
						eprintln!("{}", format!("Error clausifying: {}", e).red());
					}
				}
				println!("{}", "loaded.".green());
			},
			Err(e) => eprintln!("{}", format!("Parse error: {}", e).red()),
		},
		Err(e) => {
			eprintln!("Error reading file '{}': {}", filename.to_string_lossy(), e)
		},
	}
}

fn query_cmd(clausifier: &mut Clausifier, input: &str, rl: &mut Editor<(), FileHistory>) {
	let query = input.trim();

	if query.is_empty() {
		eprintln!("Error: Query cannot be empty.");
		return;
	}

	let mut formula = query.to_string();
	formula.push(';');

	match parse_formula(&formula) {
		Ok(expr) => match clausifier.clausify(types::ast::Expression::Not(Box::new(expr))) {
			Ok(goal_program) => match goal_program.get_clause(0) {
				Some(goal_clause) => {
					resolution::resolution::sld_resolution(
						&clausifier.get_program(),
						goal_clause,
						rl,
					);
				},
				None => eprintln!("No clauses found in the goal program."),
			},
			Err(e) => eprintln!("Error clausifying query: {}", e),
		},
		Err(e) => {
			eprintln!("Parse error: {}", e);
		},
	}
}
