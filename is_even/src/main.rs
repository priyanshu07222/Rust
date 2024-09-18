use std::io;
fn main() {
    let x = is_even(4);
    println!("{}", fib(8));

    println!("{}", x);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    };

    if num == 1 {
        return 1;
    }

    for _ in 1..num - 1 {
        let temp = second;
        second = temp + first;
        first = temp;
    }

    return second;
}
