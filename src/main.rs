mod card;
mod player;

use card::Card;
use card::CardSuit;

fn main() {
    let p = Card::new(CardSuit::Black, 12);
    let q = Card::new(CardSuit::Black, 11);
    let mut x = player::Player::new(String::from("Flavio"));

    x.add_card_to_hand(&p);
    x.add_card_to_hand(&q);

    println!("{:?}", x.hand);

    x.remove_card_from_hand(&p);
    println!("{:?}", x.hand);

    println!("rank: {}", p.get_rank(CardSuit::Black));
}
