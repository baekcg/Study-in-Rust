struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	activie: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 구조체의 body를 출락하려면 #[derive(debug)] 필요
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.height > other.height && self.width > other.width
	}

	fn sqaure(size: u32) -> Rectangle {
		Rectangle { width:size, height: size }
	}
}

fn bind_user(username: String, email: String) -> User {
	// 매개변수랑 이름같으면 생략가능
	User {
		username,
		email,
		sign_in_count: 1,
		activie: true,
	}
}

fn main() {
	let mut user1: User = User { username: String::from("baekcg"), email: String::from("doci1803@naver.com"), sign_in_count: 1, activie: true };
	println!("{}", user1.email);

	user1.email = String::from("doci1803@gmail.com");
	println!("{}", user1.email);

	let user2: User = bind_user(String::from("gimoring"), String::from("gimoring@gmail.com"));

	// 남은 필드값 가져옴 ..{}
	let user3: User = User {
		username: String::from("gimorr"),
		..user2
	};
	println!("{}", user3.email);

	let blackColor: Color = Color(0, 0, 0);

	let rect1: Rectangle = Rectangle {
		width: 50, height: 30,
	};
	// 구조체의 body를 출력하려면 {:?}
	println!("rect1 is {:?}", rect1);

	// println!("The area of the rectangle {} sqaure pixels.", area(&rect1));
	println!("{}, {}", rect1.width, rect1.height);

}

// fn area(rect: &Rectangle) -> u32 { // &Rectangle로 하는이유는 소유권이 넘겨지면 main의 rect1이 여기서 소유권 해제되기 때문에 못써서 그럼
// 	rect.height * rect.width
// }