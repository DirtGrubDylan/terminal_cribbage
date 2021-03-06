use std::fmt;

use rand::{self, Rng};

use cards::{Card, Rank, Suit};

/// The `Deck` struct is a wrapper for a vector of [`Card`]s.
///
/// This wrapper is so the vector can be treated like an actual deck of [`Card`]s
///
/// [`Card`]: struct.Card.html
#[derive(Debug, PartialEq)]
pub struct Deck(pub Vec<Card>);


impl Deck {
    /// Constructs a new `Deck`.
    ///
    /// The `Deck` is constructed the same way every time. Starting with [`Suit::Clubs`] through
    /// [`Suit::Spades`], it loops through [`Rank::Ace`] to [`Rank::King`] to build a deck in order.
    ///
    /// [`Suit::Clubs`]: enum.Suit.html
    /// [`Suit::Spades`]: enum.Suit.html
    /// [`Rank::Ace`]: enum.Rank.html
    /// [`Rank::King`]: enum.Rank.html
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::Deck;
    ///
    /// let deck = Deck::new();
    ///
    /// println!("Unshuffled deck of cards: {}", deck);
    /// ```
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        let ranks: Vec<Rank> = vec![Rank::Ace,
                                    Rank::Two,
                                    Rank::Three,
                                    Rank::Four,
                                    Rank::Five,
                                    Rank::Six,
                                    Rank::Seven,
                                    Rank::Eight,
                                    Rank::Nine,
                                    Rank::Ten,
                                    Rank::Jack,
                                    Rank::Queen,
                                    Rank::King];
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        for suit in suits.into_iter() {
            for &rank in ranks.iter() {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck(cards)
    }


    /// Shuffles the [`Card`]s in a `Deck` in place.
    ///
    /// [`Card`]: struct.Card.html
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::Deck;
    ///
    /// let mut deck = Deck::new();
    ///
    /// deck.shuffle();
    ///
    /// println!("Shuffled deck of cards: {}", deck);
    /// ```
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

        rng.shuffle(&mut self.0);
    }


    /// Deals a [`Card`] from the back of the `Deck`.
    ///
    /// [`Card`]: struct.Card.html
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Deck, Card, Rank, Suit};
    ///
    /// let mut deck = Deck::new();
    ///
    /// let dealt_card = deck.deal();
    ///
    /// assert_eq!(dealt_card, Some(Card::new(Rank::King, Suit::Spades)));
    /// ```
    pub fn deal(&mut self) -> Option<Card> {
        self.0.pop()
    }
}


impl fmt::Display for Deck {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for card in self.0.iter() {
            write!(formatter, "\n{}", card)?;
        }

        write!(formatter, "")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use cards::{Card, Rank, Suit};

    #[test]
    fn test_new() {
        let test_deck = Deck::new();

        let ranks: Vec<Rank> = vec![Rank::Ace,
                                    Rank::Two,
                                    Rank::Three,
                                    Rank::Four,
                                    Rank::Five,
                                    Rank::Six,
                                    Rank::Seven,
                                    Rank::Eight,
                                    Rank::Nine,
                                    Rank::Ten,
                                    Rank::Jack,
                                    Rank::Queen,
                                    Rank::King];
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        for suit in suits.into_iter() {
            for &rank in ranks.iter() {
                assert!(test_deck.0.contains(&Card::new(rank, suit)));
            }
        }

        assert!(test_deck.0.starts_with(&[Card::new(Rank::Ace, Suit::Clubs)]));
        assert!(test_deck.0.ends_with(&[Card::new(Rank::King, Suit::Spades)]));
        assert_eq!(test_deck.0.len(), 52);
    }


    #[test]
    fn test_eq() {
        let test_deck = Deck::new();
        let other_test_deck = Deck::new();

        assert_eq!(test_deck, other_test_deck);
    }


    #[test]
    fn test_shuffle() {
        let mut test_deck = Deck::new();
        let mut other_test_deck = Deck::new();

        test_deck.shuffle();

        assert_eq!(test_deck == other_test_deck, false);

        other_test_deck.shuffle();

        assert_eq!(test_deck == other_test_deck, false);
    }


    #[test]
    fn test_deal() {
        let mut test_deck = Deck::new();

        let mut dealt_card = test_deck.deal();

        assert_eq!(test_deck.0.len(), 51);
        assert_eq!(dealt_card, Some(Card::new(Rank::King, Suit::Spades)));

        for _ in 0..51 {
            dealt_card = test_deck.deal();
        }

        assert!(test_deck.0.is_empty());
        assert_eq!(dealt_card, Some(Card::new(Rank::Ace, Suit::Clubs)));

        dealt_card = test_deck.deal();

        assert_eq!(dealt_card, None);
    }
}