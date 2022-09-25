extern crate module_exam;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use TrafficLight::*; //전부 가져오기(glob)

use a::series::of; //of::nested_modules()
use a::series::of::nested_modules;

fn main() {
    println!("fn main()");
    module_exam::client::connect();
    a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();

    let red: TrafficLight = Red;
    let yellow: TrafficLight = Yellow;
    let green: TrafficLight = TrafficLight::Green;
}
