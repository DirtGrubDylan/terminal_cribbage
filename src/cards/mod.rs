//! Traits, helpers, and type definitions for Card functionality.
//!
//! The `libterminal_cribbage::cards` module contains a number of common things you will need when
//! using cards during the game.
//!
//! ## Card, Rank, and Suit
//!
//! With all card games, using the standard playing cards, each [`Card`] references a [`Rank`] and a 
//! [`Suit`]. `libterminal_cribbage::cards` provides the struct [`Card`], and two enums, [`Rank`] 
//! and [`Suit`], in order to ease the use of organization and comparrisons.
//!
//! For example, in Cribbage, when scoring, each [`Card`] must be evaluated based on it's [`Rank`] 
//! and [`Suit`]:
//!
//! ```
//! ```
//!
//! ## Deck and Hand
//!
//! In Cribbage there are two different types of [`Card`] piles. These are used through out the 
//! game, and to help with this `libterminal_cribbage::cards` comes with two handy structs, [`Deck`]
//! and `Hand`, which wrap these piles. The wrappers provide a way to reduce the number of calls
//! and collections for compiling and manipulating [`Card`] piles.
//!
//! For example, [`Deck`] provides a way to create a new deck of [`Card`]s, and shuffle them:
//!
//! ```
//! use libterminal_cribbage::cards::Deck;
//!
//! let mut deck = Deck::new();
//!
//! println!("Unshuffled deck of cards: {}", deck);
//!
//! deck.shuffle();
//!
//! println!("Shuffled deck of cards: {}", deck);
//! ```
//!
//! `Hand` provides an easy way to keep the cards in order, and discard:
//! `Hand` example:
//!
//! ```
//! ```
//!
//! ## Dealing and Discarding
//!
//! Since Cribbage involves a lot of dealing and discarding, you may see this functions appear all 
//! over the crate. For example, after dealing out a hand, a player must discard two [`Card`]s:
//!
//! ```
//! ```
//!
//! [`Card`]: struct.Card.html
//! [`Deck`]: struct.Deck.html
//! [`Rank`]: enum.Rank.html
//! [`Suit`]: enum.Suit.html

pub use self::deck::Deck;
pub use self::card::{Card, Rank, Suit};

mod card;
mod deck;
mod hand;