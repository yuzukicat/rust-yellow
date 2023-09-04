fn main() {
    string_push();
    string_insert();
    string_replace();
    string_replacen();
    string_replace_range();
    string_pop();
    string_remove();
    string_truncate();
    string_clear();
}

fn string_push() {
    let mut s = String::from("Hello ");

    s.push_str("rust");
    println!("push_str() -> {}", s);

    s.push('!');
    println!("push() -> {}", s);
}

fn string_insert() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("insert_str() -> {}", s);
}

fn string_replace() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
}

fn string_replacen() {
    let string_replacen = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replacen = string_replacen.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
}

fn string_replace_range() {
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}

fn string_pop() {
    let mut string_pop = String::from("rust pop!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn string_remove() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // string_remove.remove(0);
    string_remove.remove(3);
    dbg!(string_remove);
}

fn string_truncate() {
    let mut string_truncate = String::from("测试truncate方法");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}

fn string_clear() {
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}
