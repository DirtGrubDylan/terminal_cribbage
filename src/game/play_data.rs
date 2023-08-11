//! This module is for the Play (pegging) part of the game.
//!
//! It's not called pegging because I am immature.

use crate::cards::Card;

/// Simple struct to keep track of the played stack of [`Card`]s and their running raw total score.
///
/// The stack and stack score are public for display purposes. Having getters and setters is dumb
/// when Rust natively handles mutablility.
#[derive(Debug, PartialEq)]
pub struct PlayData {
    pub stack: Vec<Card>,
    pub stack_score: u32,
}

impl PlayData {
    /// Creates a new [`PlayData`] with an empty stack and a `0` stack_score.
    pub fn new() -> PlayData {
        PlayData {
            stack: Vec::new(),
            stack_score: 0,
        }
    }

    /// Adds a [`Card`] to the stack and updates the stack score.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::PlayData;
    ///
    /// let card1 = Card::new(Rank::Ace, Suit::Hearts);
    /// let card2 = Card::new(Rank::Queen, Suit::Hearts);
    ///
    /// let expected = PlayData {
    ///     stack: vec![card1.clone(), card2.clone()],
    ///     stack_score: 11,
    /// };
    ///
    /// let mut data = PlayData::new();
    ///
    /// data.add_card(card1);
    /// data.add_card(card2);
    ///
    /// assert_eq!(data, expected);
    /// ```
    pub fn add_card(&mut self, card: Card) {
        self.stack_score += card.score();
        self.stack.push(card);
    }
}

impl From<Vec<Card>> for PlayData {
    /// Convert from [`Vec`] of [`Card`]s.
    ///
    /// Mainly used for testing. This updates the stack and the stack score.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::PlayData;
    ///
    /// let cards = vec![Card::new(Rank::Ace, Suit::Hearts), Card::new(Rank::Ace, Suit::Clubs)];
    ///
    /// PlayData::from(cards);
    /// ```
    fn from(input: Vec<Card>) -> PlayData {
        let mut data = PlayData::new();

        input.into_iter().for_each(|card| {
            data.add_card(card);
        });

        data
    }
}
