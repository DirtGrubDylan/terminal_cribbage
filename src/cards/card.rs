use std::fmt;


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}


#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}


impl fmt::Display for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };

        let suit_char = match self.suit {
            Suit::Hearts => '\u{2665}',
            Suit::Clubs => '\u{2663}',
            Suit::Diamonds => '\u{2666}',
            Suit::Spades => '\u{2660}',
        };

        write!(formatter, "[{}{}]", rank_str, suit_char)
    }
}