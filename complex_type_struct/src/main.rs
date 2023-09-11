fn main() {
    create_access_struct_instance();
    update_struct();
    tuple_struct();
    dbg_print_move_struct();
    let u = Unit;
    do_something_with_unit(u);
    move_reference_struct();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn create_access_struct_instance() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("changedsomeone@example.com");
    println!("user1.email: {}", user1.email);
    println!("user1.username: {}", user1.username);
    println!("user1.active: {}", user1.active);
    println!("user1.sign_in_count: {}", user1.sign_in_count);
}

fn update_struct() {
    let user2 = construct_struct(
        String::from("anotherone@example.com"),
        String::from("anotherusername123"),
    );
    let user3 = User {
        email: String::from("yetanotherone@example.com"),
        ..user2
    };
    println!("user3.email: {}", user3.email);
    println!("user3.username: {}", user3.username);
    println!("user3.active: {}", user3.active);
    println!("user3.sign_in_count: {}", user3.sign_in_count);
    println!("user3.email: {}", user2.email);
    // println!("user2.email: {}", user2.username);
    println!("user2.active: {}", user2.active);
    println!("user2.sign_in_count: {}", user2.sign_in_count);
}

fn construct_struct(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);
fn tuple_struct() {
    let black = Color(0, 0, 0);
    println!("black is {:#?}", black);
}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}
fn dbg_print_move_struct() {
    let scale = 2;
    let rect1 = Rectangle {
        _width: dbg!(30 * scale),
        _height: 50,
    };
    dbg!(&rect1);
}

#[derive(Debug)]
struct Unit;
trait SomeTrait {}
impl SomeTrait for Unit {}
fn do_something_with_unit(u: Unit) {
    println!("{:?}", u);
}
// trait Zero {
//     const ZERO: Self;
//     fn is_zero(&self) -> bool;
// }

// impl Zero for i32 {
//     const ZERO: Self = 0;

//     fn is_zero(&self) -> bool {
//         *self == Self::ZERO
//     }
// }

fn move_reference_struct() {
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    println!("The person's age from person struct is {}", person.age);
}
