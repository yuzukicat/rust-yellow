fn main() {
    mem_size_char();
    mem_size_unit();
    println_return_type();
}

fn mem_size_char() {
    let x = 'a';
    println!(
        "The word 'a' takes up {} bytes of memory size",
        std::mem::size_of_val(&x)
    );
}

fn mem_size_unit() {
    let unit: () = ();
    assert_eq!(std::mem::size_of_val(&unit), 0);
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

fn println_return_type() {
    let v0: () = ();
    assert_eq!(v0, implicitly_ret_unit())
}
