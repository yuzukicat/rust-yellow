fn main() {
    ret_unit_type();
    implicitly_return_2();
    explicitly_return();
}

fn implicitly_return() {
    let x = 1;
    let _y = if x % 2 == 1 { "odd" } else { "even" };
}

fn ret_unit_type() {
    assert_eq!(implicitly_return(), ())
}

fn implicitly_return_2() {
    let v = {
        let mut _x = 1;
        _x += 2
    };
    assert_eq!(v, ());
}

fn explicitly_return() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
    assert_eq!(v, 3);
}
