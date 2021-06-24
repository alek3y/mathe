use regex::Regex;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
	Illegal,
	Number,
	Function,
	Bracket,
	Operator,
	Constant
}

const RULES: [(Type, &str); 5] = [
	(Type::Number, r"\d+(\.\d+|)(e(\-|\+)\d+|)"),
	(Type::Function, r"[a-zA-Z]+\("),		// Won't contain the bracket
	(Type::Bracket, r"[\(\)]"),
	(Type::Operator, r"[\+\-\*/\^%]"),
	(Type::Constant, r"([a-zA-Z]|_)+")
];

#[derive(Debug)]
pub struct Token {
	pub class: Type,
	pub text: String
}

impl Token {
	pub fn find(expression: &str) -> Token {
		for rule in RULES.iter() {
			let regex = Regex::new(rule.1).unwrap();

			let try_match = regex.find(expression);
			if try_match.is_none() {
				continue;
			}

			let found = try_match.unwrap();
			if found.start() != 0 {
				continue;
			}

			let mut text_end = found.end();
			if rule.0 == Type::Function {
				text_end -= 1;		// Remove bracket
			}

			return Token{
				class: rule.0,
				text: expression[0..text_end].to_string()
			};
		}

		return Token{
			class: Type::Illegal,
			text: expression.chars().nth(0).unwrap().to_string()
		};
	}

	pub fn tokenize(expression: &str) -> Vec<Token> {
		let mut tokens = Vec::new();

		let mut expression = expression.to_string();
		while expression.len() > 0 {
			let token_next = Token::find(expression.as_str());
			let token_len = token_next.text.len();
			tokens.push(token_next);

			expression = (&expression[token_len..]).to_string();
		}

		return tokens;
	}
}
