fn main() {
    reference_and_dereference();
    immutable_reference();
    variable_reference();
}

fn reference_and_dereference() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn immutable_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    print!("The length of {} is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn variable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
