fn main() {
	let mut x: i32 = 5;
	println!("The Value of x is {}", x);
	x = 6;
	println!("The Value of x is {}", x);

	let spaces: &str = "    ";
	let spaces: usize = spaces.len();

	println!("{}", spaces);

	// integer
	let guess: u32 = "42".parse().expect("not a  numer!");
	println!("{}", guess);

	// float	
	let y: f32 = 3.20;
	println!("{}", y);

	//boolean
	let b: bool = true;
	if b {
		println!("true");
	}

	//char - unicode
	let z: char = 'z';
	let z: char = 'ℤ';
	let z: char = '😻';
	println!("{}", z);

	// tuple
	let tuple: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z): (i32, f64, u8) = tuple;
	println!("{}, {}, {}", x, y, z);
	println!("{}, {}, {}", tuple.0, tuple.1, tuple.2);

	let arr: [i32; 5] = [1,2,3,4,5];
	let first: i32 = arr[0];
	let second: i32 = arr[1];
	println!("first: {}, second: {}", first, second);

	another_function(3);

	let f: i32 = {
		let tmp: i32 = 2;
		tmp + 3
	};
	println!("The value of f: {}", f);

	let five: i32 = returnFive();
	println!("The value of five: {}", five);

	let sum_result: i32 = sumNum(10, 2);
	println!("Sum Result: {}", sum_result);

	let condition: bool = true;
	let number: i32 = if condition {
		10
	} else {
		0
	};
	println!("The value of number: {}", number);

	//loop {
	//	println!("loop infinitly");
	//}

	let mut number: i32 = 5;
	while number > 0 {
		println!("{}", number);
		number = number - 1;
	}
	println!("LIFTOFF!");

	let arr: [i32; 5] = [10,20,30,40,50];
	for element in arr.iter() {
		println!("The value is {}", element);
	}

	arr.iter().for_each(|element: &i32| {
		println!("The value is {}", element);
	});

	for i in 0..5 {
		println!("{}!", i);
	}

	for i in (0..5).rev() {
		println!("{}", i);
	}
	

}

fn another_function(x: i32) {
	println!("The value of x is : {}", x)
}

fn returnFive() -> i32 {
	5
}

fn sumNum(x: i32, y: i32) -> i32 {
	x + y
}