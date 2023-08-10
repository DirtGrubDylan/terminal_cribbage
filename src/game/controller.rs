//! The trait and structs for controlling how players choose their [`Card`]s from their [`Hand`].

#[cfg(doc)]
use cards::Hand;

use cards::Card;

/// The `trait` for controlling how players choose their [`Card`]s from their [`Hand`].
pub trait Controller {

    /// Get a possible index for a [`Card`] from a given array of [`Card`]s.
    ///
    /// This required `&mut self` because it is assumed that some internal
    /// state of the implementors needs to change to determine the indices.
    fn get_card_index(&mut self, available_cards: &[Card]) -> Option<usize>;
}
