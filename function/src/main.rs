fn main() {
    if_return();
    explicit_return();
    never_return();
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

fn if_return() {
    let x = plus_or_minus(6);
    println!("The value of x is: {}", x);
}

fn explicit_return() -> () {
    println!("hello,world");
}

use std::thread;
use std::time;

fn never_return() -> ! {
    // panic!("I return nothing")
    // unimplemented!();
    // todo!();
    loop {
        println!("I return nothing");
        thread::sleep(time::Duration::from_secs(1));
    }
}
