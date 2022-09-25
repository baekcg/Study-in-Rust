enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Point(i32, i32, i32);
enum Test {
    Z(Point), // struct도 들어감
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Luck Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m: Message = Message::Write(String::from("Hello"));
    m.call();

    // 값이 있거나 없을 수 있는 상황 Option<T>
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    let some_none: Option<i32> = None;

    let coin: Coin = Coin::Penny;
    let cents: u32 = value_in_cents(coin);
    println!("{}", cents);
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value: u8 = 1;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _는 switch default 느낌
        _ => (),
    }

    let some_test = Some(3);
    if let Some(3) = some_test {
        println!("three");
    }
}
