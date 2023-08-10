fn main() {
    integer_overflow();
    compare_floating_point_type();
    mathematically_undefined_result();
    numerical_operation();
    range_num();
    range_char();
    type_as_str();
    create_range();
    quotient();
}

fn integer_overflow() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("b: {}", b);
}

fn compare_floating_point_type() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", (abc.2).to_bits());
    println!();
    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", (xyz.2).to_bits());

    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert_ne!((xyz.0 + xyz.1), xyz.2);
    assert!((xyz.0 + xyz.1 - xyz.2).abs() < 0.001)
}

fn mathematically_undefined_result() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("mathematically_undefined_result")
    }
}

fn numerical_operation() {
    let forty_twos = [42.0, 42f32, 42.0_f32];
    println!("{:.2}", forty_twos[0]);
}

fn range_num() {
    for i in 1..5 {
        println!("{}", i);
    }
}

fn range_char() {
    for i in 'a'..='z' {
        println!("{}", i as u8);
    }
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn type_as_str() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

use std::ops::{Range, RangeInclusive};

fn create_range() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

fn quotient() {
    let quotient = 9.6 / 3.2;
    assert!((quotient - 3.0_f64).abs() < 0.001);
}
