fn main() {
    type_derivation_and_labeling();
    type_derivation_and_labeling_2();
}

fn type_derivation_and_labeling() {
    let guess: i32 = "42".parse().expect("Not a number");
    println!("guess: {}", guess);
}

fn type_derivation_and_labeling_2() {
    let guess = "42".parse::<i32>().expect("Not a number");
    println!("guess: {}", guess);
}
