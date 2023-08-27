use std::fmt;

/// [`Rank`] is a type the represents the rank of a playing card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

/// [`Suit`] is a type the represents the suit of a playing card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

/// [`Card`] is a struct that holds the [`Rank`] and [`Suit`] type of a playing card.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Constructs a new [`Card`].
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
    #[must_use]
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    /// Gets the score of a [`Card`].
    ///
    /// All scores match the rank, where the [`Rank::Jack`], [`Rank::Queen`], and [`Rank::King`]
    /// cards are all worth 10 and the [`Rank::Ace`] is worth 1.
    ///
    /// # Exampless
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    ///
    /// let playing_card_1 = Card::new(Rank::Ace, Suit::Spades);
    /// let playing_card_2 = Card::new(Rank::Queen, Suit::Hearts);
    ///
    /// assert_eq!(playing_card_1.score(), 1);
    /// assert_eq!(playing_card_2.score(), 10);
    /// ```
    #[must_use]
    pub fn score(&self) -> u32 {
        match self.rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
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

        write!(formatter, "[{rank_str}{suit_char}]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let test_card = Card::new(Rank::Ace, Suit::Clubs);

        assert_eq!(test_card.rank, Rank::Ace);
        assert_eq!(test_card.suit, Suit::Clubs);
    }

    #[test]
    fn test_score() {
        let playing_card_1 = Card::new(Rank::Ace, Suit::Spades);
        let playing_card_2 = Card::new(Rank::Queen, Suit::Hearts);

        assert_eq!(playing_card_1.score(), 1);
        assert_eq!(playing_card_2.score(), 10);
    }
}
