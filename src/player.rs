use crate::card::Card;

pub struct Player<'a> {
    pub name: String,
    pub hand: Vec<&'a Card>,
}

impl<'a> Player<'a> {
    pub fn new(name: String) -> Self {
        return Player { name, hand: vec![] };
    }

    pub fn add_card_to_hand(&mut self, card: &'a Card) {
        self.hand.push(card);
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }

    pub fn remove_card_from_hand(&mut self, card: &Card) {
        self.hand.retain(|&e| !std::ptr::eq(card, e))
    }
}
