// The suit represents also the power of the card
// Since colors all are graded the same, we keep a distance of "100"
// between each suit, so that by floor(suit/100) we can still get the proper value

#[derive(PartialEq, Debug)]
pub enum CardSuit {
    PirateOrWhiteFlag = -1,
    WhiteFlag = 101,
    PirateOrWhiteFlagThenWhiteFlag = 102,
    Red = 201,
    Green = 202,
    Blue = 203,
    Yellow = 204,
    /* RGBYWhenSameTable 301 */
    Black = 401,
    Mermaid = 601,
    Pirate = 701,
    PirateOrWhiteFlagThenPirate = 702,
    King = 801,
}

#[derive(Debug)]
pub struct Card {
    pub suit: CardSuit,
    pub value: u8,
}

impl Card {
    pub fn new(suit: CardSuit, value: u8) -> Self {
        if (suit == CardSuit::Green
            || suit == CardSuit::Yellow
            || suit == CardSuit::Red
            || suit == CardSuit::Blue
            || suit == CardSuit::Black)
            && (value <= 0)
        {
            panic!("Invalid color card without a positive value");
        } else if (suit == CardSuit::Pirate
            || suit == CardSuit::King
            || suit == CardSuit::Mermaid
            || suit == CardSuit::WhiteFlag
            || suit == CardSuit::PirateOrWhiteFlag)
            && value != 0
        {
            panic!("Invalid Special card without a Special value");
        }

        return Card { suit, value };
    }
    pub fn is_color_card(&self, include_black: bool) -> bool {
        if include_black && self.suit == CardSuit::Black {
            return true;
        }
        return self.suit == CardSuit::Green
            || self.suit == CardSuit::Red
            || self.suit == CardSuit::Blue
            || self.suit == CardSuit::Yellow;
    }
    pub fn get_rank(&self, table_color: CardSuit) -> u32 {
        let add_score: u16 = if self.suit == table_color { 100 } else { 0 };
        let score: u16 = (self.suit as u16) + add_score;
        let rank: f32 = (score as f32) / (100 as f32);
        return rank as u32;
    }
}
