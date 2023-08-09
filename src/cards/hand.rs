use std::fmt;

use cards::card::Card;

#[cfg(doc)]
use cards::card::Rank;

/// The [`Hand`] struct is a wrapper for a vector of [`Card`]s.
///
/// This wrapper is so the vector can be treated like an actual hand of [`Card`]s
#[derive(Debug, PartialEq)]
pub struct Hand(pub Vec<Card>);

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
    ///
    /// assert!(hand.0.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Hand {
        let card_vector: Vec<Card> = Vec::new();

        Hand(card_vector)
    }

    /// Adds a [`Card`] to [`Hand`].
    ///
    /// In addition to adding the [`Card`] to the [`Hand`], the internal vector is also sorted by the
    /// [`Card`]'s [`Rank`].
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
    ///
    /// assert_eq!(
    ///     hand.0,
    ///     vec![Card::new(Rank::Ace, Suit::Clubs),
    ///          Card::new(Rank::Two, Suit::Hearts),
    ///          Card::new(Rank::Two, Suit::Spades),
    ///          Card::new(Rank::Three, Suit::Hearts),
    ///          Card::new(Rank::Four, Suit::Spades),
    ///          Card::new(Rank::Ten, Suit::Clubs),
    ///          Card::new(Rank::Queen, Suit::Diamonds)]);
    /// ```
    pub fn add_card(&mut self, card: Card) {
        self.0.push(card);
        self.0.sort();
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
        if index_of_card >= self.0.len() {
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

    /// Returns a `&[Vec]<[Card]>` as a representation of the [`Hand`].
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
    /// hand.add_card(Card::new(Rank::Four, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Three, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::Two, Suit::Spades));
    ///
    /// let hand_as_vec = hand.as_vec();
    ///
    /// assert_eq!(hand_as_vec, &expected);
    /// ```
    #[must_use]
    pub fn as_vec(&self) -> &Vec<Card> {
        &self.0
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
                Card::new(Rank::Two, Suit::Hearts),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Three, Suit::Hearts),
                Card::new(Rank::Four, Suit::Spades),
                Card::new(Rank::Ten, Suit::Clubs),
                Card::new(Rank::Queen, Suit::Diamonds)
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

        assert_eq!(card, Ok(Card::new(Rank::Four, Suit::Spades)));

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
        hand.add_card(Card::new(Rank::Four, Suit::Spades));
        hand.add_card(Card::new(Rank::Three, Suit::Hearts));
        hand.add_card(Card::new(Rank::Two, Suit::Spades));

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
