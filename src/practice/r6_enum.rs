#[derive(Debug)]
enum PockSuit {
    Clubs,
    Spades,
    Diamonds,
    Heart,
}

#[test]
fn test_enum_1() {
    let clubs = PockSuit::Clubs;
    let diamonds = PockSuit::Diamonds;
    print_suit(clubs);
    print_suit(diamonds);
}

fn print_suit(suit: PockSuit) {
    println!("suit is {:?}", suit);
}