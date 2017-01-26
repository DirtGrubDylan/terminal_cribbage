use std::fmt;

use rand::{self, Rng};

use cards::{Card, Rank, Suit};

/// The `Deck` struct is a wrapper for a vector of [`Card`]s.
///
/// This wrapper is so the vector can be treated like an actual deck of [`Card`]s
///
/// [`Card`]: struct.Card.html
#[derive(Debug)]
pub struct Deck(Vec<Card>);


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
    /// use libterminal_cribbage::cards::Deck;
    ///
    /// let mut deck = Deck::new();
    ///
    /// let dealt_card = deck.deal().unwrap();
    ///
    /// println!("Dealt card: {}", dealt_card);
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