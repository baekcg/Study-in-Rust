fn main() {
    let s1: String = String::from("hello");
	let s2: String = s1; // s1 무력화 얕은복사로 포인터를 복사하는게 아니라 걍 다 s1이 s2로 이동함

	println!("{} World !", s2);

	// 깊은 복사라고 생각하면 될듯? 
	let s1: String = String::from("hello");
	let s2: String = s1.clone();

	println!("s1 = {}, s2 = {}", s1, s2);

}

fn teakes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}
