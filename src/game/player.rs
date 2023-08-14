use itertools::Itertools;
use std::fmt;

use cards::{Card, Deck, Hand};
use game::Controller;

/// The representation of a player with a [`Hand`], a discarded pile, a [`Controller`], and points.
///
/// The discarded pile is needed because during play, you discard to the stack. Meaning that
/// discarded [`Card`]s cannot be from the [`Hand`], but they eventually go back into the
/// [`Hand`] after play is done. This is also a [`Vec`] because the discarded pile doesn't
/// need the funcationality of a [`Hand`].
///
/// The [`Controller`] is used to grab the indices of the cards to select for discarding
/// during play.
///
/// Points is self explainitory.
#[derive(Debug, PartialEq)]
pub struct Player<C>
where
    C: Controller,
{
    controller: C,
    discarded: Vec<Card>,
    pub hand: Hand,
    pub points: u32,
}

impl<C> Player<C>
where
    C: Controller,
{
    /// Creates a new [`Player`] with a given [`Controller`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let player = Player::new(controller);
    /// ```
    pub fn new(controller: C) -> Player<C> {
        Player {
            controller,
            discarded: Vec::new(),
            hand: Hand::new(),
            points: 0,
        }
    }

    /// Creates a new [`Player`] with a given [`Controller`] and [`Card`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let cards = vec![Card::new(Rank::Ace, Suit::Hearts), Card::new(Rank::Ace, Suit::Clubs)];
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let player = Player::new_with_cards(controller, cards);
    /// ```
    pub fn new_with_cards(controller: C, cards: Vec<Card>) -> Player<C> {
        Player {
            controller,
            hand: Hand::from(cards),
            discarded: Vec::new(),
            points: 0,
        }
    }

    /// Add a [`Card`] to [`Player::hand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let mut player = Player::new(controller);
    ///
    /// player.add_card(Card::new(Rank::Ace, Suit::Spades));
    /// ```
    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    /// Indicates that the [`Player`] has [`Card`]s in [`Player::hand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let mut player = Player::new(controller);
    ///
    /// assert!(!player.has_cards());
    ///
    /// player.add_card(Card::new(Rank::Ace, Suit::Spades));
    ///
    /// assert!(player.has_cards());
    /// ```
    pub fn has_cards(&self) -> bool {
        !self.hand.is_empty()
    }

    /// Chooses [`Card`] for the cut from given [`Deck`], which is removed from the [`Deck`].
    ///
    /// This [`Card`] is determined by the [`Player::controller`] and is
    /// added to [`Player::discarded`].
    ///
    /// # Panics
    ///
    /// If the [`Player::controller`] returns an index that is out of bounds of the
    /// [`Deck`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Deck, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let mut deck = Deck::new();
    ///
    /// // Cut the 13th card from the deck (12 is the index from 0).
    /// let controller = PredeterminedController::from(vec![12]);
    ///
    /// let mut player = Player::new(controller);
    ///
    /// let result = player.choose_card_for_cut(&mut deck);
    ///
    /// assert_eq!(result, Some(Card::new(Rank::King, Suit::Clubs)));
    /// assert_eq!(deck.as_vec().len(), 51);
    /// ```
    #[must_use]
    pub fn choose_card_for_cut(&mut self, deck: &mut Deck) -> Option<Card> {
        let possible_card = self
            .controller
            .get_card_index(deck.as_vec())
            .map(|index| deck.remove(index).unwrap());

        if let Some(card) = possible_card.clone() {
            self.discarded.push(card);
        }

        possible_card
    }

    /// Discards, and returns, a [`Card`] from [`Player::hand`] if there are cards to remove.
    ///
    /// This [`Card`] is determined by the [`Player::controller`] and is
    /// added to [`Player::discarded`].
    ///
    /// # Panics
    ///
    /// If the [`Player::controller`] returns an index that is out of bounds of the
    /// [`Player::hand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let cards = vec![
    ///     // Index 0 -> removed after first discard.
    ///     Card::new(Rank::Ace, Suit::Hearts),
    ///     // Index 1 -> 0 after first discard -> 0 on second discard -> removed on third discard.
    ///     Card::new(Rank::Ace, Suit::Spades),
    ///     // Index 2 -> 1 after first discard -> removed on second discard.
    ///     Card::new(Rank::Ace, Suit::Clubs),
    /// ];
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 0]);
    ///
    /// let mut player = Player::new_with_cards(controller, cards.clone());
    ///
    /// let result_1 = player.discard();
    /// let result_2 = player.discard();
    /// let result_3 = player.discard();
    ///
    /// assert_eq!(result_1, Some(cards[0].clone()));
    /// assert_eq!(result_2, Some(cards[2].clone()));
    /// assert_eq!(result_3, Some(cards[1].clone()));
    /// ```
    #[must_use]
    pub fn discard(&mut self) -> Option<Card> {
        let possible_card = self
            .controller
            .get_card_index(self.hand.as_vec())
            .map(|index| self.hand.discard(index).unwrap());

        if let Some(card) = possible_card.clone() {
            self.discarded.push(card);
        }

        possible_card
    }

    /// Removes, and returns, a [`Card`] from [`Player::hand`] if there are cards to remove.
    ///
    /// This [`Card`] is determined by the [`Player::controller`].
    ///
    /// Unlike [`Player::discard`], this method does not add to [`Player::discarded`].
    ///
    /// # Panics
    ///
    /// If the [`Player::controller`] returns an index that is out of bounds of the
    /// [`Player::hand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    ///
    /// let controller = PredeterminedController::from(vec![0]);
    ///
    /// let mut player = Player::new(controller);
    ///
    /// player.add_card(card.clone());
    ///
    /// let result = player.remove_card();
    ///
    /// assert_eq!(result, Some(card));
    /// ```
    #[must_use]
    pub fn remove_card(&mut self) -> Option<Card> {
        self.controller
            .get_card_index(self.hand.as_vec())
            .map(|index| self.hand.discard(index).unwrap())
    }

    /// Adds all the [`Card`]s in [`Player::discarded`] to the [`Player::hand`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    ///
    /// let controller = PredeterminedController::from(vec![0]);
    ///
    /// let mut player = Player::new(controller);
    ///
    /// player.add_card(card.clone());
    ///
    /// assert!(player.has_cards());
    ///
    /// let _ = player.discard();
    ///
    /// assert!(!player.has_cards());
    ///
    /// player.gather_discarded();
    ///
    /// assert!(player.has_cards());
    /// ```
    pub fn gather_discarded(&mut self) {
        self.hand = Hand::from(self.discarded.clone());
        self.discarded = Vec::new();
    }

    /// Indicats if [`Player`] has a [`Card`] whose [`Card::score`] is less than the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let card = Card::new(Rank::Six, Suit::Spades);
    ///
    /// let controller = PredeterminedController::from(vec![0]);
    ///
    /// let mut player = Player::new(controller);
    ///
    /// player.add_card(card);
    ///
    /// assert!(!player.has_card_with_score_at_most(5));
    /// assert!(player.has_card_with_score_at_most(6));
    /// assert!(player.has_card_with_score_at_most(7));
    /// ```
    #[must_use]
    pub fn has_card_with_score_at_most(&self, value: u32) -> bool {
        self.hand.as_vec().iter().any(|card| card.score() <= value)
    }
}

impl<C> fmt::Display for Player<C>
where
    C: Controller,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let discarded_str_joined = self.discarded.iter().map(|card| card.to_string()).join(",");

        write!(
            f,
            "Player: {{ Hand: {0}, Points: {1}, Discarded: [ {2} ] }}",
            self.hand, self.points, discarded_str_joined
        )
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use cards::{Card, Hand, Rank, Suit};
    use game::PredeterminedController;

    #[test]
    fn test_new() {
        let expected = Player {
            controller: PredeterminedController::from(vec![0, 1, 2]),
            hand: Hand::new(),
            discarded: Vec::new(),
            points: 0,
        };

        let controller = PredeterminedController::from(vec![0, 1, 2]);

        let result = Player::new(controller);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_with_cards() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
        ];

        let expected = Player {
            controller: PredeterminedController::from(vec![0, 1, 2]),
            hand: Hand::from(cards.clone()),
            discarded: Vec::new(),
            points: 0,
        };

        let controller = PredeterminedController::from(vec![0, 1, 2]);

        let result = Player::new_with_cards(controller, cards);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_card() {
        let card = Card::new(Rank::Ace, Suit::Spades);

        let expected = Player {
            controller: PredeterminedController::from(vec![0, 1, 2]),
            hand: Hand::from(vec![card.clone()]),
            discarded: Vec::new(),
            points: 0,
        };

        let controller = PredeterminedController::from(vec![0, 1, 2]);

        let mut player = Player::new(controller);

        player.add_card(card);

        assert_eq!(player, expected);
    }

    #[test]
    fn test_has_cards_false() {
        let controller = PredeterminedController::from(vec![0, 1, 2]);

        let player = Player::new(controller);

        assert!(!player.has_cards());
    }

    #[test]
    fn test_has_cards_true() {
        let card = Card::new(Rank::Ace, Suit::Spades);

        let controller = PredeterminedController::from(vec![0, 1, 2]);

        let mut player = Player::new(controller);

        player.add_card(card);

        assert!(player.has_cards());
    }

    #[test]
    #[should_panic]
    fn test_discard_controller_index_oob_panics() {
        let card = Card::new(Rank::Ace, Suit::Spades);

        let controller = PredeterminedController::from(vec![1, 0, 2]);

        let mut player = Player::new(controller);

        player.add_card(card.clone());

        let _ = player.discard();
    }

    #[test]
    fn test_discard() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];

        let controller = PredeterminedController::from(vec![0, 1, 0]);

        let expected_discarded = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let expected_player = Player {
            controller: PredeterminedController::from(Vec::new()),
            hand: Hand::new(),
            discarded: expected_discarded.clone(),
            points: 0,
        };

        let mut player = Player::new_with_cards(controller, cards);

        let result: Vec<Card> = (0..=2).map(|_| player.discard().unwrap()).collect();

        assert_eq!(result, expected_discarded);
        assert_eq!(player, expected_player);
    }

    #[test]
    #[should_panic]
    fn test_remove_card_controller_index_oob_panics() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];

        let controller = PredeterminedController::from(vec![0, 3, 1]);

        let mut player = Player::new_with_cards(controller, cards);

        let _result: Vec<_> = (0..=2).map(|_| player.remove_card()).collect();
    }

    #[test]
    fn test_remove_card() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];

        let controller = PredeterminedController::from(vec![0, 1, 0]);

        let expected_removed = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let expected_player = Player {
            controller: PredeterminedController::from(Vec::new()),
            hand: Hand::new(),
            discarded: Vec::new(),
            points: 0,
        };

        let mut player = Player::new_with_cards(controller, cards);

        let result: Vec<Card> = (0..=2).map(|_| player.remove_card().unwrap()).collect();

        assert_eq!(result, expected_removed);
        assert_eq!(player, expected_player);
    }

    #[test]
    fn test_gather_discarded() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];

        let controller = PredeterminedController::from(vec![0, 0, 0]);

        let expected = Player {
            controller: PredeterminedController::from(Vec::new()),
            hand: Hand::from(cards.clone()),
            discarded: Vec::new(),
            points: 0,
        };

        let mut player = Player::new_with_cards(controller, cards);

        let _discards: Vec<Card> = (0..=2).map(|_| player.discard().unwrap()).collect();

        assert!(!player.has_cards());

        player.gather_discarded();

        assert!(player.has_cards());
        assert_eq!(player, expected);
    }
}
