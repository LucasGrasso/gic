pub mod common;
pub mod integers;
pub mod lists;

use lists::lists_builtin::is_list_pred;

use crate::libraries::common::*;
use crate::libraries::integers::arithmetic::*;
use crate::libraries::integers::comparation::*;
use crate::libraries::lists::lists_builtin::*;

use crate::mgu::mgu::Substitution;
use crate::types::clause::{Clause, Literal};

pub fn built_in_preds<'a>(
	goal: &'a Clause,
	lit: &'a Literal,
	sub: &'a Substitution,
) -> Option<Box<dyn Iterator<Item = (Clause, Substitution)> + 'a>> {
	if let Literal::Not(p) = lit {
		match (p.name.as_str(), p.terms.len()) {
			("Eq", 2) => return eq_pred(sub, p, goal),
			("Diff", 2) => return diff_pred(sub, p, goal),
			("Var", 1) => return var_pred(sub, p, goal),
			("Add", 3) => return arithmetic_op_pred(goal, p, sub, |a, b| a + b),
			("Sub", 3) => return arithmetic_op_pred(goal, p, sub, |a, b| a - b),
			("Mul", 3) => return arithmetic_op_pred(goal, p, sub, |a, b| a * b),
			("Div", 3) => {
				return arithmetic_op_pred(goal, p, sub, |a, b| {
					if b == 0 {
						panic!("Division by zero");
					}
					a / b
				});
			},
			("Mod", 3) => return arithmetic_op_pred(goal, p, sub, |a, b| a % b),
			("Lt", 2) => return compare_pred(goal, p, sub, |a, b| a < b),
			("Lt_eq", 2) => return compare_pred(goal, p, sub, |a, b| a <= b),
			("Gt", 2) => return compare_pred(goal, p, sub, |a, b| a > b),
			("Gt_eq", 2) => return compare_pred(goal, p, sub, |a, b| a >= b),
			("Eq_int", 2) => return compare_pred(goal, p, sub, |a, b| a == b),
			("Diff_int", 2) => return compare_pred(goal, p, sub, |a, b| a != b),
			("Between", 3) => return between_pred(goal, p, sub),
			("Is_list", 1) => return is_list_pred(goal, p, sub),
			("Length", 2) => return length_pred(goal, p, sub),
			_ => return None,
		}
	}
	None
}
