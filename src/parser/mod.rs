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
		let first_token = &tokens[0];

		// Remove opening and closing brackets
		if first_token.class == Type::Bracket {
			return Tree::new(&tokens[1..tokens.len()-1].to_vec());
		}

		let mut root_index = Tree::find_lightest_operator(&tokens);
		if first_token.class == Type::Function {
			root_index = 0;		// Function token as root
		}

		let root = tokens[root_index].clone();
		let left = tokens[..root_index].to_vec();
		let right = tokens[root_index+1..].to_vec();
		return Box::new(Tree{
			data: root,
			left: if left.len() > 0 {Option::Some(Tree::new(&left))} else {Option::None},
			right: if right.len() > 0 {Option::Some(Tree::new(&right))} else {Option::None}
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
