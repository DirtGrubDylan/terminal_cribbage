//! Trait for displaying the game.
use std::io::{self, Write};

#[cfg(doc)]
use crate::cards::Hand;

use crate::cards::Card;
use crate::game::{Controller, PlayData, Player};

/// The `trait` for controlling how the game is displayed.
pub trait Display {
    fn flush_stdout(&self) -> io::Result<()> {
        io::stdout().flush()
    }

    /// Print message to `std::out` without a new line, a spacer, or a delay.
    fn print_no_spacer_no_delay(&self, message: &str);

    /// Print message to `std::out` without a spacer or a delay.
    fn println_no_spacer_no_delay(&self, message: &str);

    /// Print message with spacer to `std::out` using [`thread::sleep`] with a delay after printing.
    fn println(&self, message: &str);

    /// The [`String`] display for both [`Player`]s [`Card`]s cut from the [`Deck`].
    #[must_use]
    fn game_after_cut_message(
        &self,
        player_cut: &Card,
        opponent_cut: &Card,
        player_won: bool,
    ) -> String;

    /// The [`String`] display for both [`Player`]s and the starter [`Card`] before play.
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib. If starter is [`None`], then `"[?]"`. The player's crib will only be displayed if they
    /// have one.
    #[must_use]
    fn game_before_play_message<C1, C2>(
        &self,
        starter: Option<&Card>,
        player: &Player<C1>,
        opponent: &Player<C2>,
    ) -> String
    where
        C1: Controller,
        C2: Controller;

    /// The [`String`] display for both [`Player`]s, the starter [`Card`], and [`PlayData`] during play.
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib. The player's crib will only be displayed if they have one.
    #[must_use]
    fn game_during_play_message<C1, C2>(
        &self,
        starter: &Card,
        player: &Player<C1>,
        opponent: &Player<C2>,
        play_data: &PlayData,
    ) -> String
    where
        C1: Controller,
        C2: Controller;

    /// The [`String`] display for both [`Player`]s and the starter [`Card`] during counting.
    ///
    /// This will show the opponent's and player's points, [`Hand`]s and cribs.
    #[must_use]
    fn game_during_counting_message<C1, C2>(
        &self,
        starter: &Card,
        player: &Player<C1>,
        opponent: &Player<C2>,
    ) -> String
    where
        C1: Controller,
        C2: Controller;

    /// The [`String`] display for game over.
    #[must_use]
    fn game_over_message(&self, player_won: bool) -> String;
}
