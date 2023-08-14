use std::fmt;

use itertools::Itertools;
use rand::seq::SliceRandom;

use cards::{Card, Rank, Suit};

/// The [`Deck`] struct is a wrapper for a vector of [`Card`]s.
///
/// This wrapper is so the vector can be treated like an actual deck of [`Card`]s
#[derive(Debug, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    /// Constructs a new `Deck`.
    ///
    /// The `Deck` is constructed the same way every time. Starting with [`Suit::Clubs`] through
    /// [`Suit::Spades`], it loops through [`Rank::Ace`] to [`Rank::King`] to build a deck in order.
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
    #[must_use]
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        let ranks: Vec<Rank> = vec![
            Rank::Ace,
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
            Rank::King,
        ];
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        for suit in suits {
            for &rank in &ranks {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck(cards)
    }

    /// Shuffles the [`Card`]s in a [`Deck`] in place.
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

        self.0.shuffle(&mut rng);
    }

    /// Deals a [`Card`] from the back of the [`Deck`].
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

    /// Removes a [`Card`] from the [`Deck`].
    ///
    /// # Errors
    ///
    /// If the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Deck, Card, Rank, Suit};
    ///
    /// let mut deck = Deck::new();
    ///
    /// // Removes the 13th card from deck (12 is the index from 0).
    /// let result = deck.remove(12);
    ///
    /// assert_eq!(result, Ok(Card::new(Rank::King, Suit::Clubs)));
    /// assert_eq!(deck.as_vec().len(), 51);
    /// ```
    pub fn remove(&mut self, index_of_card: usize) -> Result<Card, String> {
        if self.0.len() <= index_of_card {
            return Err("Out of Bounds!".to_string());
        }

        Ok(self.0.remove(index_of_card))
    }

    /// Returns [`Vec`] representation of the [`Deck`]
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Deck, Card, Rank, Suit};
    ///
    /// let deck = Deck::new();
    ///
    /// let deck_vec = deck.as_vec();
    /// ```
    pub fn as_vec(&self) -> &Vec<Card> {
        &self.0
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let cards_str_joined = self.0.iter().map(|card| card.to_string()).join(",");

        write!(formatter, "[ {cards_str_joined} ]")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cards::{Card, Rank, Suit};

    #[test]
    fn test_new() {
        let test_deck = Deck::new();

        let ranks: Vec<Rank> = vec![
            Rank::Ace,
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
            Rank::King,
        ];
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        for suit in suits {
            for &rank in &ranks {
                assert!(test_deck.0.contains(&Card::new(rank, suit)));
            }
        }

        assert!(test_deck
            .0
            .starts_with(&[Card::new(Rank::Ace, Suit::Clubs)]));
        assert!(test_deck
            .0
            .ends_with(&[Card::new(Rank::King, Suit::Spades)]));
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

        assert_ne!(test_deck, other_test_deck);

        other_test_deck.shuffle();

        assert_ne!(test_deck, other_test_deck);
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
