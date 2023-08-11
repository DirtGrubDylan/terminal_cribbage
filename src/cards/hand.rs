use std::fmt;

use cards::score;
use cards::Card;

#[cfg(doc)]
use cards::card::Rank;

/// The [`Hand`] struct is a wrapper for a vector of [`Card`]s.
///
/// This wrapper is so the vector can be treated like an actual hand of [`Card`]s
#[derive(Debug, PartialEq)]
pub struct Hand(Vec<Card>);

impl Hand {
    /// Constructs a new [`Hand`].
    ///
    /// The [`Hand`] is constructed with the internal vector being new.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::Hand;
    ///
    /// let hand = Hand::new();
    /// ```
    #[must_use]
    pub fn new() -> Hand {
        let card_vector: Vec<Card> = Vec::new();

        Hand(card_vector)
    }

    /// Adds a [`Card`] to [`Hand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Hand, Card, Rank, Suit};
    ///
    /// let mut hand = Hand::new();
    ///
    /// hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
    /// hand.add_card(Card::new(Rank::Four, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Three, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::Two, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Queen, Suit::Diamonds));
    /// hand.add_card(Card::new(Rank::Two, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::Ten, Suit::Clubs));
    /// ```
    pub fn add_card(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Discard a [`Card`] from [`Hand`] by index. Returns [`Err`] if the index is out of bounds.
    ///
    /// # Errors
    ///
    /// If the index is out of bounds or the [`Hand`] has no cards.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Hand, Card, Rank, Suit};
    ///
    /// let mut hand = Hand::new();
    ///
    /// hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
    /// hand.add_card(Card::new(Rank::Four, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Three, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::Two, Suit::Spades));
    ///
    /// let mut card = hand.discard(2);
    ///
    /// assert_eq!(card, Ok(Card::new(Rank::Three, Suit::Hearts)));
    /// ```
    pub fn discard(&mut self, index_of_card: usize) -> Result<Card, String> {
        if self.0.len() <= index_of_card {
            return Err("Out of Bounds!".to_string());
        }

        Ok(self.0.remove(index_of_card))
    }

    /// Discard a [`Card`] from [`Hand`] that matching the given [`Card`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Hand, Card, Rank, Suit};
    ///
    /// let card1 = Card::new(Rank::Ace, Suit::Clubs);
    /// let card2 = Card::new(Rank::Ace, Suit::Diamonds);
    ///
    /// let mut hand = Hand::new();
    ///
    /// hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
    /// hand.add_card(Card::new(Rank::Four, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Three, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::Two, Suit::Spades));
    ///
    /// let discard1 = hand.discard_matching(&card1);
    /// let discard2 = hand.discard_matching(&card2);
    ///
    /// assert_eq!(discard1, Some(card1));
    /// assert_eq!(discard2, None);
    /// ```
    pub fn discard_matching(&mut self, matching_card: &Card) -> Option<Card> {
        self.0
            .iter()
            .position(|card| card == matching_card)
            .map(|index| self.0.remove(index))
    }

    /// Returns a `&`[`Vec`]`<`[`Card`]`>` as a representation of the [`Hand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Hand, Card, Rank, Suit};
    ///
    /// let expected = vec![
    ///     Card::new(Rank::Ace, Suit::Clubs),
    ///     Card::new(Rank::Two, Suit::Spades),
    ///     Card::new(Rank::Three, Suit::Hearts),
    ///     Card::new(Rank::Four, Suit::Spades),
    /// ];
    ///
    /// let mut hand = Hand::new();
    ///
    /// hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
    /// hand.add_card(Card::new(Rank::Two, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Three, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::Four, Suit::Spades));
    ///
    /// let hand_as_vec = hand.as_vec();
    ///
    /// assert_eq!(hand_as_vec, &expected);
    /// ```
    #[must_use]
    pub fn as_vec(&self) -> &Vec<Card> {
        &self.0
    }

    /// Returns the score of the [`Hand`].
    ///
    /// # Panics
    ///
    /// Panics if:
    ///   * This method finds more combinations adding to `15` then can fit into a [`u32`].
    ///   * This method finds more matching pairs then can fit into a [`u32`].
    ///   * There is a [`Rank`] variant who's enum value is greater than `12`.///
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Hand, Rank, Suit};
    ///
    /// let cards = vec![
    ///     Card::new(Rank::Jack, Suit::Clubs),
    ///     Card::new(Rank::Five, Suit::Diamonds),
    ///     Card::new(Rank::Five, Suit::Hearts),
    ///     Card::new(Rank::Five, Suit::Spades),
    /// ];
    ///
    /// let starter = Card::new(Rank::Five, Suit::Clubs);
    ///
    /// // Highest scoring hand in cribbage by the way!
    /// let hand = Hand::from(cards);
    ///
    /// let score = hand.total(&starter, /*is_crib=*/ false);
    ///
    /// assert_eq!(score, 29);
    /// ```
    #[must_use]
    pub fn total(&self, starter: &Card, is_crib: bool) -> u32 {
        score::total(self, starter, is_crib)
    }

    /// Indicates if the [`Hand`] is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Hand, Rank, Suit};
    ///
    /// let cards = vec![
    ///     Card::new(Rank::Jack, Suit::Clubs),
    ///     Card::new(Rank::Five, Suit::Diamonds),
    ///     Card::new(Rank::Five, Suit::Hearts),
    ///     Card::new(Rank::Five, Suit::Spades),
    /// ];
    ///
    /// // Highest scoring hand in cribbage by the way!
    /// let hand = Hand::from(cards);
    ///
    /// assert!(!hand.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.0 {
            write!(formatter, "\n{card}")?;
        }

        write!(formatter, "")
    }
}

/// Converts a [`Vec<Card>`] to a [`Hand`].
///
/// # Examples
///
/// ```
/// use libterminal_cribbage::cards::{Hand, Card, Rank, Suit};
///
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Clubs),
///     Card::new(Rank::Two, Suit::Spades),
///     Card::new(Rank::Three, Suit::Hearts),
///     Card::new(Rank::Four, Suit::Spades),
/// ];
///
/// let hand = Hand::from(cards.clone());
///
/// let hand_as_vec = hand.as_vec();
///
/// assert_eq!(hand_as_vec, &cards);
/// ```
impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        let mut hand = Hand::new();

        for card in cards {
            hand.add_card(card);
        }

        hand
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cards::card::{Card, Rank, Suit};

    #[test]
    fn test_new() {
        let test_hand = Hand::new();

        assert!(test_hand.0.is_empty());
        assert_eq!(test_hand, Hand::new());
    }

    #[test]
    fn test_add_card() {
        let mut hand = Hand::new();

        hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
        hand.add_card(Card::new(Rank::Four, Suit::Spades));
        hand.add_card(Card::new(Rank::Three, Suit::Hearts));
        hand.add_card(Card::new(Rank::Two, Suit::Spades));
        hand.add_card(Card::new(Rank::Queen, Suit::Diamonds));
        hand.add_card(Card::new(Rank::Two, Suit::Hearts));
        hand.add_card(Card::new(Rank::Ten, Suit::Clubs));

        assert_eq!(hand.0.len(), 7);
        assert_eq!(
            hand.0,
            vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Four, Suit::Spades),
                Card::new(Rank::Three, Suit::Hearts),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Queen, Suit::Diamonds),
                Card::new(Rank::Two, Suit::Hearts),
                Card::new(Rank::Ten, Suit::Clubs),
            ]
        );
    }

    #[test]
    fn test_discard() {
        let mut hand = Hand::new();

        hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
        hand.add_card(Card::new(Rank::Four, Suit::Spades));
        hand.add_card(Card::new(Rank::Three, Suit::Hearts));
        hand.add_card(Card::new(Rank::Two, Suit::Spades));

        let mut card = hand.discard(2);

        assert_eq!(card, Ok(Card::new(Rank::Three, Suit::Hearts)));

        for _ in 0..3 {
            card = hand.discard(0);
        }

        assert_eq!(card, Ok(Card::new(Rank::Two, Suit::Spades)));

        card = hand.discard(0);

        assert!(card.is_err());
    }

    #[test]
    fn test_as_vec() {
        let expected = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let mut hand = Hand::new();

        hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
        hand.add_card(Card::new(Rank::Two, Suit::Spades));
        hand.add_card(Card::new(Rank::Three, Suit::Hearts));
        hand.add_card(Card::new(Rank::Four, Suit::Spades));

        let hand_as_vec = hand.as_vec();

        assert_eq!(hand_as_vec, &expected);
    }

    #[test]
    fn discard_matching() {
        let card1 = Card::new(Rank::Ace, Suit::Clubs);
        let card2 = Card::new(Rank::Ace, Suit::Diamonds);

        let mut hand = Hand::new();

        hand.add_card(card1.clone());
        hand.add_card(Card::new(Rank::Four, Suit::Spades));
        hand.add_card(Card::new(Rank::Three, Suit::Hearts));
        hand.add_card(Card::new(Rank::Two, Suit::Spades));

        let discard1 = hand.discard_matching(&card1);
        let discard2 = hand.discard_matching(&card2);

        assert_eq!(discard1, Some(card1));
        assert_eq!(discard2, None);
    }

    #[test]
    fn from_vec() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let hand = Hand::from(cards.clone());

        let hand_as_vec = hand.as_vec();

        assert_eq!(hand_as_vec, &cards);
    }
}
