use rand::{rngs::ThreadRng, Rng};

use cards::Card;
use game::Controller;

/// A controller that gets all of it's moves from an RNG.
///
/// This is a very dumb AI, but it's a good first start.
#[derive(Debug, Clone)]
pub struct RngController {
    rng: ThreadRng,
}

impl RngController {
    /// Creates a new [`RngController`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::game::RngController;
    ///
    /// let controller = RngController::new();
    /// ```
    #[must_use]
    pub fn new() -> RngController {
        RngController {
            rng: rand::thread_rng(),
        }
    }
}

impl Controller for RngController {
    /// Returns a possible index for a [`Card`] for a given array of [`Card`]s.
    ///
    /// The index is randomly chosen within the range of the given array of [`Card`]s.
    ///
    /// # Panics
    ///
    /// If the index is out of bounds for the `available_cards`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Controller, RngController};
    ///
    /// let no_cards = vec![];
    /// let available_cards = vec![
    ///     Card::new(Rank::Queen, Suit::Hearts),
    ///     Card::new(Rank::King, Suit::Clubs),
    /// ];
    ///
    /// let mut controller = RngController::new();
    ///
    /// assert!(controller.get_card_index(&no_cards).is_none());
    /// assert!(controller.get_card_index(&available_cards).is_some());
    /// ```
    #[must_use]
    fn get_card_index(&mut self, available_cards: &[Card]) -> Option<usize> {
        if available_cards.is_empty() {
            None
        } else {
            Some(self.rng.gen_range(0..available_cards.len()))
        }
    }
}

impl Default for RngController {
    fn default() -> Self {
        Self::new()
    }
}
