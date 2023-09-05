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
    string_add();
    string_format();
    string_loop_chars();
    string_loop_bytes();
    string_escaping();
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

fn string_add() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("string add + -> {}", result);
}

fn string_format() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}", s1, s2);
    println!("{}", s);
}

fn string_loop_chars() {
    for c in "japanese".chars() {
        println!("{}", c);
    }
}

fn string_loop_bytes() {
    for b in "japanese".bytes() {
        println!("{}", b);
    }
}

fn string_escaping() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("WHat are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE_STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
can span multiple lines.
The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
