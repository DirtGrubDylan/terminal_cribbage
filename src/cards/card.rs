use std::fmt;

/// `Rank` is a type the represents the rank of a playing card.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rank {
    Ace,
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
}


/// `Suit` is a type the represents the suit of a playing card.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}


/// `Card` is a struct that holds the [`Rank`] and [`Suit`] type of a playing card.
///
/// [`Rank`]: enum.Rank.html
/// [`Suit`]: enum.Suit.html
#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Constructs a new `Card`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    ///
    /// let playing_card = Card::new(Rank::Ace, Suit::Spades);
    ///
    /// println!("Played card: {}", playing_card);
    /// ```
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank: rank, suit:suit }
    }
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