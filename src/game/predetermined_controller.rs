use std::collections::VecDeque;

use cards::Card;
use game::{Controller, Display, NoOpDisplay};

/// A "predetermined" controller, who implements [`Controller`].
///
/// It is predetermined because instead of querying an AI or prompting a user, this
/// controller uses a given [`VecDeque`] of card indices to make decisions for the player.
///
/// This is strictly used for testing purposes, since AI or user prompts is how card play
/// is normally determined.
#[derive(Debug, PartialEq, Clone)]
pub struct PredeterminedController<D>
where
    D: Display,
{
    /// The indicies for choosing [`Card`]s for a player.
    card_indices: VecDeque<usize>,
    display: D,
}

impl<D: Display> PredeterminedController<D> {
    /// Creates a new [`PredeterminedController`] with a given array and [`NoOpDisplay`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// use libterminal_cribbage::game::{NoOpDisplay, PredeterminedController};
    ///
    /// let controller = PredeterminedController::new(vec![1, 2, 3], NoOpDisplay::new());
    /// ```
    pub fn new(card_indices: Vec<usize>, display: D) -> PredeterminedController<D> {
        PredeterminedController {
            card_indices: VecDeque::from(card_indices),
            display,
        }
    }
}

impl<D: Display> Controller for PredeterminedController<D> {
    /// Returns a possible index for a [`Card`] for a given array of [`Card`]s.
    ///
    /// The value is the result of [`VecDeque::pop_front`] from the internal
    /// [`PredeterminedController::card_indices`].
    ///
    /// # Panics
    ///
    /// If the index at the front of [`PredeterminedController::card_indices`] is
    /// out of bounds for the `available_cards`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Controller, PredeterminedController};
    ///
    /// let available_cards = vec![
    ///     Card::new(Rank::Queen, Suit::Hearts),
    ///     Card::new(Rank::King, Suit::Clubs),
    /// ];
    ///
    /// let mut controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// assert_eq!(controller.get_card_index(&available_cards), Some(0));
    /// assert_eq!(controller.get_card_index(&available_cards), Some(1));
    /// assert_eq!(controller.get_card_index(&available_cards), Some(2));
    /// assert_eq!(controller.get_card_index(&available_cards), None);
    /// ```
    fn get_card_index(&mut self, available_cards: &[Card]) -> Option<usize> {
        let result = self.card_indices.pop_front();

        let number_of_cards = available_cards.len();

        let message = format!(
            "Choose Card to Discard (1 to {number_of_cards}): {:?}",
            result.map(|index| index + 1)
        );

        self.display.println_no_spacer_no_delay(&message);

        result
    }
}

impl From<Vec<usize>> for PredeterminedController<NoOpDisplay> {
    /// Converts a [`Vec<usize>`] to a [`PredeterminedController`].
    ///
    /// The [`Display`] is set to a [`NoOpDisplay`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::game::PredeterminedController;
    /// use std::collections::VecDeque;
    ///
    /// let result = PredeterminedController::from(vec![1, 2, 3]);
    /// ```
    fn from(vec: Vec<usize>) -> Self {
        PredeterminedController::new(vec, NoOpDisplay::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_from_vec() {
        let expected = PredeterminedController {
            card_indices: VecDeque::from([1, 2, 3]),
            display: NoOpDisplay::new(),
        };

        let result = PredeterminedController::from(vec![1, 2, 3]);

        assert_eq!(result, expected);
    }
}
