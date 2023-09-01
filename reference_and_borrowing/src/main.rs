fn main() {
    reference_and_dereference();
    immutable_reference();
    variable_reference();
    memory_address();
    ref_keyword();
    borrowing_immutable_from_mutable_objects();
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

fn memory_address() {
    let x = 5;
    let p = &x;
    println!("the memory address of x is {:p}", p);
}

fn ref_keyword() {
    let c = 'z';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));
}

// get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn borrowing_immutable_from_mutable_objects() {
    let mut s = String::from("hello, ");
    borrow_object(&s);
    s.push_str("world");
}

fn borrow_object(s: &String) {}
