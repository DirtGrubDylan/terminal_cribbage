use cards::{Card, Hand};
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
    pub controller: C,
    pub hand: Hand,
    pub discarded: Vec<Card>,
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
    /// use libterminal_cribbage::cards::{Card, Hand};
    /// use libterminal_cribbage::game::{Player, PredeterminedController};
    ///
    /// let expected = Player {
    ///     controller: PredeterminedController::from(vec![0, 1, 2]),
    ///     hand: Hand::new(),
    ///     discarded: Vec::new(),
    ///     points: 0,
    /// };
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let result = Player::new(controller);
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn new(controller: C) -> Player<C> {
        Player {
            controller,
            hand: Hand::new(),
            discarded: Vec::new(),
            points: 0,
        }
    }

    /// Add a [`Card`] to [`Player::hand`].
    pub fn add_card(&mut self, card: Card) {
        unimplemented!()
    }

    /// Add points to a [`Player`].
    pub fn add_points(&mut self, points: u32) {
        unimplemented!()
    }

    /// Indicates that the [`Player`] has [`Card`]s in [`Player::hand`].
    pub fn has_cards(&mut self, card: Card) {
        unimplemented!()
    }

    /// Discards, and returns, a [`Card`] from [`Player::hand`].
    ///
    /// This [`Card`] is determined by the [`Player::controller`] and is
    /// added to [`Player::discarded`].
    pub fn discard(&mut self) -> &Card {
        unimplemented!()
    }

    /// Removes, and returns, a [`Card`] from [`Player::hand`].
    ///
    /// This [`Card`] is determined by the [`Player::controller`].
    ///
    /// Unlike [`discard`], this method does not add to [`Player::discarded`].
    pub fn remove_card(&mut self, card: Card) -> Card {
        unimplemented!()
    }

    /// Adds all the [`Card`]s in [`Player::discarded`] to the [`Player::hand`].
    pub fn gather_discarded(&mut self, card: Card) {
        unimplemented!()
    }
}
