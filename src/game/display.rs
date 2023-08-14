//! Handles the display of the game.

#[cfg(doc)]
use crate::cards::Hand;

use crate::cards::Card;
use crate::game::{Controller, Player};

/// A struct for just displaying the game.
///
/// Not sure how this will work yet.
///
/// Ideal display would be:
///
/// Player has crib, but starter card is not flipped;
/// *************************************
/// Player Points: 0 | Opponent Points: 0
/// Starter: [?]
/// Player Hand: [8♠],[K♣],[2♠],[6♦],[5♦],[5♣]
/// Player Crib:
/// Choose Card to Discard (0 to 5): _
/// *************************************
///
/// Player has crib, but starter card is not flipped, chooses wrong index;
/// *************************************
/// Player Points: 0 | Opponent Points: 0
/// Starter: [?]
/// Player Hand: [8♠],[K♣],[2♠],[6♦],[5♦],[5♣]
/// Player Crib:
/// Choose Card to Discard (0 to 5): q
/// Card "q" is not valid!
/// Choose Card to Discard (0 to 5): _
/// *************************************
///
/// Player has crib and starter card is flipped, but before play:
/// *************************************
/// Player Points: 0 | Opponent Points: 0
/// Starter: [4♦]
/// Player Hand: [8♠],[K♣],[2♠],[6♦]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// *************************************
///
/// Player has crib and starter card is flipped during play and played once:
/// *************************************
/// Player Points: 0 | Opponent Points: 0
/// Starter: [4♦]
/// Player Hand: [8♠],[K♣],[6♦]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// Opponent Hand Size: 3
/// Opponent Last Played: [8♣]
/// Play Stack: [A♦],[8♣]
/// Choose Card to Play (0 to 2): _
/// *************************************
///
/// Player has crib and starter card is flipped during play and played twice:
/// *************************************
/// Player Points: 2 | Opponent Points: 3
/// Starter: [4♦]
/// Player Hand: [8♠],[K♣]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// Opponent Hand Size: 2
/// Opponent Last Played: [7♣]
/// Play Stack: [A♦],[8♣],[6♦],[7♣]
/// Choose Card to Play (0 to 1): _
/// *************************************
///
/// Player has crib and starter card is flipped during play, but player chose illegal card:
/// *************************************
/// Player Points: 2 | Opponent Points: 3
/// Starter: [4♦]
/// Player Hand: [8♠],[K♣]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// Opponent Hand Size: 2
/// Opponent Last Played: [7♣]
/// Play Stack: [A♦],[8♣],[6♦],[7♣]
/// Choose Card to Play (0 to 1): _
/// Card at 1 is [K♣], which is not a legal play!
/// *************************************
///
/// Player has crib and starter card is flipped during play and play is reset:
/// *************************************
/// Player Points: 4 | Opponent Points: 3
/// Starter: [4♦]
/// Player Hand: [K♣]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// Opponent Hand Size: 2
/// Opponent Last Played: [7♣]
/// Play Stack: [A♦],[8♣],[6♦],[7♣],[8♠]
/// Choose Card to Play (0 to 1): 0
/// Stack is 30 for a GO point!
///
/// Player has crib and starter card is flipped during play and play is reset:
/// *************************************
/// Player Points: 4 | Opponent Points: 3
/// Starter: [4♦]
/// Player Hand: [K♣]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// Opponent Hand Size: 2
/// Opponent Last Played:
/// Play Stack:
/// Choose Card to Play (0 to 0): _
/// *************************************
///
/// Player has crib and starter card is flipped, hand and crib are counted.
/// *************************************
/// Player Points: 8 | Opponent Points: 3
/// Starter: [4♦]
/// Player Hand: [8♠],[K♣],[2♠],[6♦]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// Oppenent Hand: [2♠],[Q♣],[3♠],[K♦]
/// Opponenet hand score: 4
/// Hand score: 0
/// Crib score: 4
/// *************************************
///
pub struct Display {}

impl Display {
    /// Creates a new [`Display`] struct.
    pub fn new() -> Display {
        Display {}
    }

    /// The string display for the starter [`Card`].
    ///
    /// If starter is [`None`], then `"[?]"`.
    pub fn starter_to_string(starter: Option<&Card>) -> String {
        unimplemented!()
    }

    /// The string display for both [`Player`]s' cut [`Card`].
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib.
    pub fn cut_cards_to_string(player_cut_card: &Card, opponent_cut_card: &Card) -> String {
        unimplemented!()
    }

    /// The string display for both [`Player`]s.
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib.
    pub fn players_to_string<C>(player: &Player<C>, opponent: &Player<C>) -> String
    where
        C: Controller,
    {
        unimplemented!()
    }

    /// The string display for both [`Player`]s and the starter [`Card`].
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib. If starter is [`None`], then `"[?]"`.
    pub fn total_game_to_string<C>(
        starter: Option<&Card>,
        player: &Player<C>,
        opponent: &Player<C>,
    ) -> String
    where
        C: Controller,
    {
        unimplemented!()
    }
}

impl Default for Display {
    fn default() -> Self {
        Display::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        unimplemented!()
    }
}
