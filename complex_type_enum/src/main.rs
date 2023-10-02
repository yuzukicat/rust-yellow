fn main() {
    enumerated_value();
    enumerate_with_value();
    match_option();
    explicit_set_value();
    enum_array();
    match_linked_list();
}

#[derive(Debug)]
enum PokeSuit {
    Clubs,
    _Spades,
    Diamonds,
    Hearts,
}

struct PokeCard {
    suit: PokeSuit,
    value: u8,
}

fn enumerated_value() {
    let heart = PokeSuit::Hearts;
    let diamond = PokeSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond);

    let c1 = PokeCard {
        suit: PokeSuit::Clubs,
        value: 12,
    };
    println!("c1.Pokesuit: {:?}", c1.suit);
    println!("c1.value: {}", c1.value);
}

fn print_suit(card: PokeSuit) {
    println!("{:?}", card);
}

#[derive(Debug)]
enum PokerCard {
    _Clubs(u8),
    Spades(u8),
    Diamonds(char),
    _Hearts(char),
}

fn enumerate_with_value() {
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds('A');
    println!("c1: {:?}", c1);
    println!("c2: {:?}", c2);
}

fn match_option() {
    let five = Some(5);
    let six = plus_one(five);
    println!("Six: {:?}", six);
    let none = plus_one(None);
    println!("None: {:?}", none);

    if let Some(n) = six {
        println!("{}", n);
        return;
    }

    panic!("NEVER LET THIS RUN!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn explicit_set_value() {
    println!("One as u8: {:?}", Number::One as u8);
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
}

enum Number {
    _Zero,
    One,
    _Two,
}

enum Number1 {
    _Zero = 0,
    One,
    _Two,
}

enum Number2 {
    _Zero = 0,
    One = 1,
    _Two = 2,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    _Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_array() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}

use crate::List::*;

enum List {
    // Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // A node that signifies the end of the linked list
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn match_linked_list() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());

    println!("{}", list.stringify());
}
