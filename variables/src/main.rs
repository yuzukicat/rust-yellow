fn main() {
    variable_deconstruction();
    destructive_assignment();
    variable_shadowing();
    variable_shadowing_2();
    variable_scope();
    variable_scope_2();
    variable_scope_3();
}

fn variable_deconstruction() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

fn destructive_assignment() {
    struct Struct {
        e: i32,
    }

    let (mut a, b, c, d, e);
    (a, b) = (1, 2);
    a += 2;
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([3, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn variable_shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn variable_shadowing_2() {
    let spaces = "    ";
    println!("spaces_str: {}", spaces);
    let spaces = spaces.len();
    println!("spaces_num: {}", spaces);
}

fn variable_scope() {
    let x: i32 = 10;
    let y: i32 = 20;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

fn variable_scope_2() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}

fn variable_scope_3() {
    let x = define_x_2();
    println!("{:?}, world", x);
}

fn define_x_2() -> &'static str {
    let x = "hello";
    x
}
