mod parser;
mod types;

use crate::parser::parse_gic_file;

fn main() {
	let gic_code = r#"
        //forall T1.forall T2.forall M.forall N.(Tipo(M, arrow(T1,T2)) and Tipo(N, T1)) impl Tipo(app(M, N), T2);

		exists M.Tipo(M, arrow(a,arrow(b,c)));

		exists M.Tipo(M, arrow(a,b));

		exists M.Tipo(M, a);
    "#;

	match parse_gic_file(gic_code) {
		Ok(expressions) => {
			println!("Successfully parsed {} expressions:", expressions.len());
			for (i, expr) in expressions.iter().enumerate() {
				println!("--- Expression {} ---", i + 1);
				println!("{}", expr);
			}
		},
		Err(e) => {
			eprintln!("Error parsing GIC file: {}", e);
		},
	}
}
