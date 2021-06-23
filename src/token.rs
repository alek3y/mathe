use regex::Regex;

#[derive(PartialEq, Copy, Clone)]
pub enum Type {
	Number,
	Function,
	Operator,
	Constant
}

const RULES: [(Type, &str); 4] = [
	(Type::Number, r"\A\d+(\.\d+|)(e(\-|\+)\d+|)"),
	(Type::Function, r"\A[a-zA-Z]+\("),
	(Type::Operator, r"\A[\+\-\*/\^%]"),
	(Type::Constant, r"\A([a-zA-Z]|_)+")
];

pub struct Token {
	pub class: Type,
	pub label: Option<String>,
	pub body: String
}

impl Token {
	pub fn find(expression: String) -> Option<Token> {
		for rule in RULES.iter() {
			let regex = Regex::new(rule.1).unwrap();

			let try_match = regex.find(expression.as_str());
			if try_match.is_none() {
				continue;
			}

			let found = try_match.unwrap();
			if found.start() != 0 {
				continue;
			}

			if rule.0 == Type::Function {
				let label = (&expression[..found.end()-1]).to_string();

				let mut body_end: usize = 0;
				let mut parenthesis: usize = 1;		// Count for the first matched
				for (i, letter) in (&expression[found.end()..expression.len()]).chars().enumerate() {
					match letter.to_string().as_str() {
						"(" => parenthesis += 1,
						")" => parenthesis -= 1,
						_ => {}
					}

					if parenthesis == 0 {
						body_end = found.end() + i;
						break;
					}
				}

				// NOTE: Panics when parenthesis aren't closed
				let body = (&expression[found.end()..body_end]).to_string();

				return Option::Some(Token{
					class: rule.0,
					label: Option::Some(label),
					body: body
				});
			}

			return Option::Some(Token{
				class: rule.0,
				label: Option::None,
				body: (&expression[0..found.end()]).to_string(),
			});
		}

		panic!("couldn't match a valid token for '{}'", expression);
	}

	//pub fn tokenize(expression: String) -> Option<Token> {}
}
