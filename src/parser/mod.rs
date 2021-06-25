pub mod token;
use token::*;

pub struct Tree {
	data: Token,
	left: Box<Tree>,
	right: Option<Box<Tree>>		// Optional with function tokens
}

impl Tree {
	//pub fn new(tokens: &Vec<Token>) -> Box<Tree> {}

	pub fn sanitize(string: &String) -> String {
		// TODO
	}

	pub fn find_lightest_operator(tokens: &Vec<Token>) -> usize {
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
		return &self.left;
	}

	pub fn get_right(&self) -> &Box<Tree> {
		return &(&self.right).as_ref().unwrap();
	}
}
