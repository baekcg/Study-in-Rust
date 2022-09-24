use std::io;

fn main() {
    println!("== Guess A Number ==");
	println!("Input your number: ");

	let mut number = String::new();
	io::stdin().read_line(&mut number)
		.expect("Failed to read line");

	println!("Your Number: {}", number);
}
