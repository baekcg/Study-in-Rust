use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
	println!("== Guess A Number ==");
	println!("Input your number: ");

	let mut rng = rand::thread_rng();
	let secret_number = rng.gen_range(1..=101);
	// let secret_string: String = secret_number.to_string();

	loop {
		let mut input_string = String::new();
		io::stdin().read_line(&mut input_string)
			.expect("Failed to read line");
		
		// let input_number: i32 = input_string.trim().parse()
		// 	.expect("Please type a number!");
		let input_number: i32 = match input_string.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please Input A Number!!!");
				println!("");
				continue;
			},
		};
		
		println!("Random Number: {}", secret_number);
		println!("Your Input Number: {}", input_number);

		match input_number.cmp(&secret_number) {
			Ordering::Less => println!("Too Small!"),
			Ordering::Greater => println!("Too Big!"),
			Ordering::Equal => {
				println!("Correct!");
				break;
			},
		}
	}
	// match input_string.cmp(&secret_string) {
	// 	Ordering::Less => println!("Too Small!"),
	// 	Ordering::Greater => println!("Too Big!"),
	// 	Ordering::Equal => println!("Correct!"),
	// }
}
