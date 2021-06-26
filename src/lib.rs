pub mod parser;
use parser::*;
use token::*;

pub fn evaluate(expression: &str) -> f64 {
	let tokens = Token::tokenize(expression);
	let tree = Tree::new(&tokens);
	let mut stack = Vec::new();
	evaluate_on_stack(&tree, &mut stack);
	return stack.pop().unwrap();
}

fn evaluate_on_stack(tree: &Box<Tree>, stack: &mut Vec<f64>) {
	let token = &tree.data;

	if tree.left.is_none() && tree.right.is_none() {
		if token.class != Type::Number {
			panic!();
		}

		stack.push(token.text.parse().unwrap());
		return;
	}

	if tree.left.is_some() {		// Will be None with functions
		evaluate_on_stack(&tree.left.as_ref().unwrap(), stack);
	}
	evaluate_on_stack(&tree.right.as_ref().unwrap(), stack);

	let result: f64;
	match token.class {
		Type::Function => {
			let right = stack.pop().unwrap();

			match token.text.as_str() {
				"sqrt" => result = right.sqrt(),
				_ => panic!()
			}
		},
		Type::Operator => {
			let right = stack.pop().unwrap();
			let left = stack.pop().unwrap();

			match token.text.as_str() {
				"+" => result = left + right,
				"-" => result = left - right,
				"*" => result = left * right,
				"/" => result = left / right,
				"%" => result = left % right,
				"^" => result = left.powf(right),
				_ => panic!()
			}
		},
		_ => panic!()
	}

	stack.push(result);
}
