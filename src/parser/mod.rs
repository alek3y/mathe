pub mod token;
use token::*;

#[derive(Debug)]
pub struct Tree {
	data: Token,
	left: Option<Box<Tree>>,
	right: Option<Box<Tree>>
}

impl Tree {
	pub fn new(tokens: &Vec<Token>) -> Box<Tree> {
		let tokens = Tree::sanitize(tokens);
		if tokens.len() == 1 {
			return Box::new(Tree{
				data: tokens[0].clone(),		// TODO: Check how this runs performance-wise
				left: Option::None,
				right: Option::None
			});
		}

		// TODO:
		// Handle brackets and functions (shouldn't do anything
		// for the latter)

		let root_index = Tree::find_lightest_operator(&tokens);
		let root = tokens[root_index].clone();
		let left = tokens[..root_index].to_vec();
		let right = tokens[root_index+1..].to_vec();
		return Box::new(Tree{
			data: root,
			left: Option::Some(Tree::new(&left)),
			right: Option::Some(Tree::new(&right))
		});
	}

	fn sanitize(tokens: &Vec<Token>) -> Vec<Token> {
		let mut sanitized_tokens = Vec::new();

		for token in tokens.iter() {
			if token.class != Type::Illegal {
				sanitized_tokens.push(token.clone());
			}
		}

		return sanitized_tokens;
	}

	fn find_lightest_operator(tokens: &Vec<Token>) -> usize {
		let mut lightest_index = 0usize;
		let mut lightest_weight = 0u8;

		let mut parenthesis = 0usize;
		let mut found_one = false;
		for (i, token) in tokens.iter().enumerate() {
			if token.class == Type::Bracket {
				match token.text.as_str() {
					"(" => parenthesis += 1,
					")" => parenthesis -= 1,
					_ => panic!()
				}
			} else if token.class == Type::Function {
				parenthesis += 1;
			}

			// Skip subexpressions
			if parenthesis > 0 {
				continue;
			}

			// Ignore non operators
			let weight = token.weight();
			if weight.is_none() {
				continue;
			}

			let weight = weight.unwrap();
			if weight <= lightest_weight || !found_one {
				lightest_index = i;
				lightest_weight = weight;

				found_one = true;
			}
		}

		return lightest_index;
	}

	pub fn get_left(&self) -> &Box<Tree> {
		return &(&self.left).as_ref().unwrap();
	}

	pub fn get_right(&self) -> &Box<Tree> {
		return &(&self.right).as_ref().unwrap();
	}
}
