use core::time::Duration;
use std::thread;
fn main() {
    let clo = | name: String | -> String {
        return name;
    };

    let expensive_closure = | num: u32 | -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("the exp clo {} ",expensive_closure(4));
    println!("the clos mclosure {}", clo(String::from("neha")));
}
