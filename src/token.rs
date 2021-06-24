use regex::Regex;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
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
	pub fn find(expression: &String) -> Option<Token> {
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

			return Option::Some(Token{
				class: rule.0,
				text: (&expression[0..text_end]).to_string()
			});
		}

		return Option::None;
	}

	//pub fn tokenize(expression: String) -> Option<Token> {}
}
