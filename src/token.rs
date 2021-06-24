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

#[derive(Debug, Clone)]
pub struct Token {
	pub class: Type,
	pub text: String
}

impl Token {
	pub fn find(expression: &String) -> Token {
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

			let mut text_end = found.end();
			if rule.0 == Type::Function {
				text_end -= 1;		// Remove bracket
			}

			return Token{
				class: rule.0,
				text: (&expression[0..text_end]).to_string()
			};
		}

		return Token{
			class: Type::Illegal,
			text: expression.chars().nth(0).unwrap().to_string()
		};
	}

	pub fn tokenize(mut expression: String) -> Vec<Token> {
		let mut tokens = Vec::new();

		while expression.len() > 0 {
			let token_next = Token::find(&expression);
			tokens.push(token_next.clone());

			expression = (&expression[token_next.text.len()..]).to_string();
		}

		return tokens;
	}
}
