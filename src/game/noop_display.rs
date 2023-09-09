//! A No-Op display for the Game.
//!
//! This is mainly used for testing, but also used for the NPCs.

use crate::cards::Card;
use crate::game::{Controller, Display, PlayData, Player};

/// A struct for displaying (or not in this case) the [`Game`] that uses the [`Display`] trait.
#[derive(Debug, PartialEq, Clone)]
pub struct NoOpDisplay {}

impl NoOpDisplay {
    /// Creates a new [`NoOpDisplay`] struct.
    #[must_use]
    pub fn new() -> NoOpDisplay {
        NoOpDisplay {}
    }
}

impl Display for NoOpDisplay {
    /// Does nothing.
    fn println_no_spacer_no_delay(&self, _message: &str) {}

    /// Does nothing.
    fn println(&self, _message: &str) {}

    /// Returns an empty [`String`].
    #[must_use]
    fn game_after_cut_message(
        &self,
        _player_cut: &Card,
        _opponent_cut: &Card,
        _player_won: bool,
    ) -> String {
        String::new()
    }

    /// Returns an empty [`String`].
    #[must_use]
    fn game_before_play_message<C1, C2>(
        &self,
        _starter: Option<&Card>,
        _player: &Player<C1>,
        _opponent: &Player<C2>,
    ) -> String
    where
        C1: Controller,
        C2: Controller,
    {
        String::new()
    }

    /// Returns an empty [`String`].
    #[must_use]
    fn game_during_play_message<C1, C2>(
        &self,
        _starter: &Card,
        _player: &Player<C1>,
        _opponent: &Player<C2>,
        _play_data: &PlayData,
    ) -> String
    where
        C1: Controller,
        C2: Controller,
    {
        String::new()
    }

    /// The [`String`] display for both [`Player`]s and the starter [`Card`] during counting.
    ///
    /// This will show the opponent's and player's points, [`Hand`]s and cribs.
    #[must_use]
    fn game_during_counting_message<C1, C2>(
        &self,
        _starter: &Card,
        _player: &Player<C1>,
        _opponent: &Player<C2>,
    ) -> String
    where
        C1: Controller,
        C2: Controller,
    {
        String::new()
    }

    /// The [`String`] display for game over.
    #[must_use]
    fn game_over_message(&self, _player_won: bool) -> String {
        String::new()
    }
}

impl Default for NoOpDisplay {
    fn default() -> Self {
        NoOpDisplay::new()
    }
}
