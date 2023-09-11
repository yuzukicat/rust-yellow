fn main() {
    enumerated_value();
    enumerate_with_value();
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
