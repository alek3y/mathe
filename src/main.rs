use std::io;
use std::io::Write;
use mathe::evaluate;

fn main() -> io::Result<()> {
	loop {
		print!("> ");
		io::stdout().flush()?;

		let mut expression = String::new();
		io::stdin().read_line(&mut expression)?;

		println!("{}", evaluate(&expression));
	}
}
