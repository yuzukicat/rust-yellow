fn main() {
    if_return();
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
