use std::io;
use std::io::Write;
use mathe::evaluate;

fn main() {
	loop {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut expression = String::new();
		io::stdin().read_line(&mut expression).unwrap();

		let result = evaluate(&expression);
		if result.is_err() {
			println!("{}", result.err().unwrap());
			continue;
		}

		println!("{}", result.unwrap());
	}
}
