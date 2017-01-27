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
//! use libterminal_cribbage::cards::{Card, Rank, Suit};
//!
//! let playing_card = Card::new(Rank::Ace, Suit::Clubs);
//! let other_playing_card = Card::new(Rank::Eight, Suit::Clubs);
//!
//! assert_eq!(playing_card.suit, other_playing_card.suit);
//!
//! let score = match playing_card.rank {
//!     Rank::Ace => 1,
//!     Rank::Two => 2,
//!     Rank::Three => 3,
//!     Rank::Four => 4,
//!     Rank::Five => 5,
//!     Rank::Six => 6,
//!     Rank::Seven => 7,
//!     Rank::Eight => 8,
//!     Rank::Nine => 9,
//!     Rank::Ten => 10,
//!     Rank::Jack => 10,
//!     Rank::Queen => 10,
//!     Rank::King => 10,
//! };
//!
//! assert_eq!(score, 1);
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
//! use libterminal_cribbage::cards::{Deck, Hand};
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
//! use libterminal_cribbage::cards::{Deck, Card, Hand, Rank, Suit};
//!
//! let mut hand = Hand::new();
//!
//! hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
//! hand.add_card(Card::new(Rank::Four, Suit::Spades));
//! hand.add_card(Card::new(Rank::Three, Suit::Hearts));
//! hand.add_card(Card::new(Rank::Two, Suit::Spades));
//! hand.add_card(Card::new(Rank::Queen, Suit::Diamonds));
//! hand.add_card(Card::new(Rank::Two, Suit::Hearts));
//! hand.add_card(Card::new(Rank::Ten, Suit::Clubs));
//!
//! assert_eq!(
//!     hand.0,
//!     vec![Card::new(Rank::Ace, Suit::Clubs),
//!          Card::new(Rank::Two, Suit::Hearts),
//!          Card::new(Rank::Two, Suit::Spades),
//!          Card::new(Rank::Three, Suit::Hearts),
//!          Card::new(Rank::Four, Suit::Spades),
//!          Card::new(Rank::Ten, Suit::Clubs),
//!          Card::new(Rank::Queen, Suit::Diamonds)]);
//!
//! assert_eq!(hand.discard(4), Ok(Card::new(Rank::Four, Suit::Spades)));
//! assert_eq!(hand.discard(4), Ok(Card::new(Rank::Ten, Suit::Clubs)));
//! ```
//!
//! ## Dealing and Discarding
//!
//! Since Cribbage involves a lot of dealing and discarding, you may see this functions appear all
//! over the crate. For example, after dealing out a hand, a player must discard two [`Card`]s:
//!
//! ```
//! use libterminal_cribbage::cards::{Deck, Hand};
//!
//! let mut deck = Deck::new();
//! let mut hand = Hand::new();
//!
//! deck.shuffle();

//! for _ in 0..6 {
//!     hand.add_card(deck.deal().unwrap());
//! }
//!
//! println!("Hand of cards: {}", hand);
//! println!("Discarded Card: {}", hand.discard(0).unwrap());
//!
//! ```
//!
//! [`Card`]: struct.Card.html
//! [`Deck`]: struct.Deck.html
//! [`Rank`]: enum.Rank.html
//! [`Suit`]: enum.Suit.html

pub use self::deck::Deck;
pub use self::hand::Hand;
pub use self::card::{Card, Rank, Suit};

mod card;
mod deck;
mod hand;