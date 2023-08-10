//! The trait and structs for controlling how players choose their [`Card`]s from their [`Hand`].

/// The `trait` for controlling how players choose their [`Card`]s from their [`Hand`].
trait Controller {

    /// Get an index for a [`Card`] from a given array of [`Card`]s.
    fn get_card_index(available_cards: &[Card]) -> usize;
}
