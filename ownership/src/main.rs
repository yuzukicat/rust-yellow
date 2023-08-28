fn main() {
    copy_of_reference();
    clone();
    copy();
    copy_of_reference2();
    as_str();
    move_or_copy();
    move_or_copy2();
    box_new();
    partial_move();
}

fn copy_of_reference() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}

fn clone() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}", x, y);
}

fn copy() {
    let x = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}

fn copy_of_reference2() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}", x, y);
}

fn as_str() {
    let x = String::from("hello, world");
    let y = x.as_str();
    println!("{},{}", x, y);
}

fn move_or_copy() {
    let s = String::from("hello, world");
    print_str(s.clone());
    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s);
}

fn move_or_copy2() {
    let s = String::from("hello, world");
    print_str2(&s);
    println!("{}", s);
}

fn print_str2(s: &String) {
    println!("{}", s);
}

fn box_new() {
    let x = Box::new(5);
    let mut y = Box::new(3);
    *y = 4;
    assert_eq!(*x, 5);
}

fn partial_move() {
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // let age = & person.age;
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // println!("The person struct is {:?}", person);

    println!("The person's age from the person struct is {}", person.age);
}
