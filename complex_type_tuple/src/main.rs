fn main() {
    deconstruct_tuple();
    access_tuple();
    return_multiple_values_tuple();
}

fn deconstruct_tuple() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is {}", y);
}

fn access_tuple() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let one = x.2;
    println!("The value of one is {}", one);
}

fn return_multiple_values_tuple() {
    let s1 = String::from("hello");
    let (s2, length) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, length);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
