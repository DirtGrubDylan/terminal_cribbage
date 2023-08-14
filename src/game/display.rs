//! Handles the display of the game.

#[cfg(doc)]
use crate::cards::Hand;

use itertools::Itertools;

use crate::cards::Card;
use crate::game::{Controller, PlayData, Player};

/// A struct for just displaying the game.
///
/// Not sure how this will work yet.
///
/// Ideal display would be:
///
///
/// Player cuts deck for dealer;
/// *************************************
/// Choose Card to Cut (0 to 52): _
/// *************************************
///
/// Player has to discard to crib.
/// *************************************
/// Player Points: 0 | Opponent Points: 0
/// Starter: [?]
/// Player Hand: [8♠],[K♣],[2♠],[6♦],[5♦],[5♣]
/// Choose Card to Discard (0 to 5): _
/// *************************************
///
/// Player has to discard to crib, but starter card is not flipped, chooses wrong index;
/// *************************************
/// Player Points: 0 | Opponent Points: 0
/// Starter: [?]
/// Player Hand: [8♠],[K♣],[2♠],[6♦],[5♦],[5♣]
/// Choose Card to Discard (0 to 5): q
/// Card "q" is not valid!
/// Choose Card to Discard (0 to 5): _
/// *************************************
///
/// Player does not have a crib and starter card is flipped, but before play:
/// *************************************
/// Player Points: 0 | Opponent Points: 0
/// Starter: [4♦]
/// Player Hand: [8♠],[K♣],[2♠],[6♦]
/// *************************************
///
/// Player has crib and starter card is flipped for his-heels, but before play:
/// *************************************
/// Player Points: 2 | Opponent Points: 0
/// Starter: [J♦]
/// Player Hand: [8♠],[K♣],[2♠],[6♦]
/// Player Crib: [A♣],[2♣],[3♣],[4♣]
/// His-heels for 2pts
/// *************************************
///
/// Player has crib and starter card is flipped not his-heels, but before play:
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
pub struct Display {
    pub joiner: String,
}

impl Display {
    /// Creates a new [`Display`] struct.
    pub fn new() -> Display {
        Display {
            joiner: String::from("\n"),
        }
    }

    /// The [`String`] display for both [`Player`]s and the starter [`Card`] before play.
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib. If starter is [`None`], then `"[?]"`. The player's crib will only be displayed if they
    /// have one.
    pub fn game_before_play_to_string<C>(
        &self,
        starter: Option<&Card>,
        player: &Player<C>,
        opponent: &Player<C>,
    ) -> String
    where
        C: Controller,
    {
        let mut result = Vec::new();

        result.push(Self::spacer());

        result.push(format!(
            "Player Points: {} | Opponent Points: {}",
            player.points, opponent.points
        ));
        result.push(format!("Starter: {}", Self::card_string(starter)));
        result.push(format!("Player Hand: {}", player.hand));

        if player.has_crib() {
            result.push(format!("Player Crib: {}", player.crib));
        }

        result.push(Self::spacer());

        result.join(&self.joiner)
    }

    /// The [`String`] display for both [`Player`]s, the starter [`Card`], and [`PlayData`] during play.
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib. The player's crib will only be displayed if they have one.
    pub fn game_during_play_to_string<C>(
        &self,
        starter: &Card,
        player: &Player<C>,
        opponent: &Player<C>,
        play_data: &PlayData,
    ) -> String
    where
        C: Controller,
    {
        unimplemented!()
    }

    /// The [`String`] display for both [`Player`]s and the starter [`Card`] during counting.
    ///
    /// This will show the opponent's and player's points, [`Hand`]s and cribs.
    pub fn game_during_counting_to_string<C>(
        &self,
        starter: &Card,
        player: &Player<C>,
        opponent: &Player<C>,
    ) -> String
    where
        C: Controller,
    {
        unimplemented!()
    }

    /// The display [`String`] representation of a [`Option<&Card>`].
    fn card_string(possible_card: Option<&Card>) -> String {
        match possible_card {
            Some(card) => card.to_string(),
            None => "[?]".to_string(),
        }
    }

    /// The display [`String`] spacer before and after every display.
    fn spacer() -> String {
        String::from("******************************************")
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

    use crate::cards::{Card, Rank, Suit};
    use crate::game::{Player, PredeterminedController};

    #[test]
    fn test_game_before_play_to_string_discard_to_crib_no_starter() {
        let display = Display::new();

        let starter = None;
        let controller = PredeterminedController::from(vec![]);

        let player_1_cards = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);

        let player_2 = Player::new(controller.clone());

        let expected = String::new()
            + "******************************************\n"
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [?]\n"
            + "Player Hand: [ [8♠],[K♣],[2♠],[6♦],[5♦],[5♣] ]\n"
            + "******************************************";

        let result = display.game_before_play_to_string(starter, &player_1, &player_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_before_play_to_string_discard_to_crib_with_starter() {
        let display = Display::new();

        let starter = Card::new(Rank::Four, Suit::Diamonds);
        let controller = PredeterminedController::from(vec![]);

        let player_1_cards = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);

        let player_2 = Player::new(controller.clone());

        let expected = String::new()
            + "******************************************\n"
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[2♠],[6♦],[5♦],[5♣] ]\n"
            + "******************************************";

        let result = display.game_before_play_to_string(Some(&starter), &player_1, &player_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_before_play_to_string_with_crib_with_starter() {
        let display = Display::new();

        let starter = Card::new(Rank::Four, Suit::Diamonds);
        let controller = PredeterminedController::from(vec![]);

        let hand = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Six, Suit::Diamonds),
        ];
        let crib = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let player_1 = Player::new_with_cards_and_crib(controller.clone(), hand, crib);

        let player_2 = Player::new(controller.clone());

        let expected = String::new()
            + "******************************************\n"
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[2♠],[6♦] ]\n"
            + "Player Crib: [ [A♣],[2♣],[5♦],[5♣] ]\n"
            + "******************************************";

        let result = display.game_before_play_to_string(Some(&starter), &player_1, &player_2);

        assert_eq!(result, expected);
    }


}
