//! Handles the display of the game.

#[cfg(doc)]
use crate::cards::Hand;

use std::{thread, time};

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
/// Show cuts from deck;
/// *************************************
/// Player Cut: [K♣]
/// Opponenet Cut: [8♠]
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
/// Player Points: j | Opponent Points: 3
/// Starter: [4♦]
/// Player Hand: [8♠],[K♣],[A♠],[6♦]
/// Player Crib: [A♣],[2♣],[5♦],[5♣]
/// Opponenet Hand: [8♦],[K♦],[6♣],[8♣]
/// Opponenet hand score: 2
/// Hand score: 4
/// Crib score: 4
/// *************************************
///
#[derive(Debug, PartialEq, Clone)]
pub struct Display {
    pub joiner: String,
    should_print: bool,
    post_print_delay_millis: time::Duration,
}

impl Display {
    /// Creates a new [`Display`] struct.
    #[must_use]
    pub fn new() -> Display {
        Display {
            joiner: String::from("\n"),
            should_print: false,
            post_print_delay_millis: time::Duration::from_millis(500),
        }
    }

    /// Turns on printing for [`Diplay`].
    pub fn turn_on_printing(&mut self, should_print: bool) {
        self.should_print = should_print;
    }

    /// Print given message to `std::out` using [`thread::sleep`] with a delay after printing.
    pub fn println(&self, message: String) {
        if self.should_print {
            println!("\n{message}");

            thread::sleep(self.post_print_delay_millis);
        }
    }

    /// The [`String`] display for both [`Player`]s [`Card`]s cut from the [`Deck`].
    pub fn game_after_cut_to_string(
        &self,
        player_cut: &Card,
        opponent_cut: &Card,
        player_won: bool,
    ) -> String {
        let mut result = Vec::new();

        result.push(Self::spacer());

        result.push(format!(
            "Player Cut: {}",
            Self::card_string(Some(player_cut))
        ));
        result.push(format!(
            "Opponent Cut: {}",
            Self::card_string(Some(opponent_cut))
        ));

        if player_won {
            result.push("Player Won Cut".to_string());
        } else {
            result.push("Opponent Won Cut".to_string());
        }

        result.push(Self::spacer());

        result.join(&self.joiner)
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
        let mut result = Vec::new();

        result.push(Self::spacer());

        result.push(format!(
            "Player Points: {} | Opponent Points: {}",
            player.points, opponent.points
        ));
        result.push(format!("Starter: {starter}"));
        result.push(format!("Player Hand: {}", player.hand));

        if player.has_crib() {
            result.push(format!("Player Crib: {}", player.crib));
        }

        result.push(format!("Opponent Hand Size: {}", opponent.hand.len()));

        let opponent_last_played = opponent
            .last_discarded()
            .map_or(String::new(), std::string::ToString::to_string);

        result.push(format!("Opponent Last Played: {opponent_last_played}"));

        let play_stack_str = play_data
            .stack
            .iter()
            .map(std::string::ToString::to_string)
            .join(",");

        result.push(format!("Play Stack: [ {play_stack_str} ]"));

        result.push(Self::spacer());

        result.join(&self.joiner)
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
        let mut result = Vec::new();

        result.push(Self::spacer());

        result.push(format!(
            "Player Points: {} | Opponent Points: {}",
            player.points, opponent.points
        ));
        result.push(format!("Starter: {starter}"));

        result.push(format!("Player Hand: {}", player.hand));

        if player.has_crib() {
            result.push(format!("Player Crib: {}", player.crib));
        }

        result.push(format!("Opponent Hand: {}", opponent.hand));

        if opponent.has_crib() {
            result.push(format!("Opponent Crib: {}", opponent.crib));
        }

        result.push(format!(
            "Opponent Hand Score: {}",
            opponent.hand.total(starter, /*is_crib=*/ false)
        ));

        if opponent.has_crib() {
            result.push(format!(
                "Opponent Crib Score: {}",
                opponent.crib.total(starter, /*is_crib=*/ true)
            ));
        }

        result.push(format!(
            "Hand Score: {}",
            player.hand.total(starter, /*is_crib=*/ false)
        ));

        if player.has_crib() {
            result.push(format!(
                "Crib Score: {}",
                player.crib.total(starter, /*is_crib=*/ true)
            ));
        }

        result.push(Self::spacer());

        result.join(&self.joiner)
    }

    /// The [`String`] display for game over.
    pub fn game_over_to_string(&self, player_won: bool) -> String {
        let mut result = Vec::new();

        result.push(Self::spacer());

        if player_won {
            result.push("You Won!".to_string());
        } else {
            result.push("You Lost!".to_string());
        }

        result.push(Self::spacer());

        result.join(&self.joiner)
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
    use crate::game::{PlayData, Player, PredeterminedController};

    #[test]
    fn test_game_after_cut_to_string_player_won() {
        let display = Display::new();

        let player_cut = Card::new(Rank::King, Suit::Clubs);
        let opponent_cut = Card::new(Rank::Eight, Suit::Spades);

        let expected = String::new()
            + "******************************************\n"
            + "Player Cut: [K♣]\n"
            + "Opponent Cut: [8♠]\n"
            + "Player Won Cut\n"
            + "******************************************";

        let result =
            display.game_after_cut_to_string(&player_cut, &opponent_cut, /*player_won=*/ true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_after_cut_to_string_opponent_won() {
        let display = Display::new();

        let player_cut = Card::new(Rank::Eight, Suit::Spades);
        let opponent_cut = Card::new(Rank::King, Suit::Clubs);

        let expected = String::new()
            + "******************************************\n"
            + "Player Cut: [8♠]\n"
            + "Opponent Cut: [K♣]\n"
            + "Opponent Won Cut\n"
            + "******************************************";

        let result = display.game_after_cut_to_string(
            &player_cut,
            &opponent_cut,
            /*player_won=*/ false,
        );

        assert_eq!(result, expected);
    }

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

        let player_2 = Player::new(controller);

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

        let player_2 = Player::new(controller);

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

        let player_2 = Player::new(controller);

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

    #[test]
    fn test_game_during_play_to_string_with_crib() {
        let display = Display::new();

        let starter = Card::new(Rank::Four, Suit::Diamonds);
        let controller = PredeterminedController::from(vec![3]);

        let player_1_hand = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
        ];
        let crib = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let player_1 = Player::new_with_cards_and_crib(controller.clone(), player_1_hand, crib);

        let player_2_hand = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let mut player_2 = Player::new_with_cards(controller, player_2_hand);

        let stack = vec![Card::new(Rank::Ace, Suit::Diamonds)];
        let mut play_data = PlayData::from(stack);

        play_data.play_once(&mut player_2, &player_1);

        let expected = String::new()
            + "******************************************\n"
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[6♦] ]\n"
            + "Player Crib: [ [A♣],[2♣],[5♦],[5♣] ]\n"
            + "Opponent Hand Size: 3\n"
            + "Opponent Last Played: [8♣]\n"
            + "Play Stack: [ [A♦],[8♣] ]\n"
            + "******************************************";

        let result = display.game_during_play_to_string(&starter, &player_1, &player_2, &play_data);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_during_play_to_string_without_crib() {
        let display = Display::new();

        let starter = Card::new(Rank::Four, Suit::Diamonds);
        let controller = PredeterminedController::from(vec![3]);

        let player_1_hand = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
        ];
        let player_1 = Player::new_with_cards(controller.clone(), player_1_hand);

        let player_2_hand = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let mut player_2 = Player::new_with_cards(controller, player_2_hand);

        let stack = vec![Card::new(Rank::Ace, Suit::Diamonds)];
        let mut play_data = PlayData::from(stack);

        play_data.play_once(&mut player_2, &player_1);

        let expected = String::new()
            + "******************************************\n"
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[6♦] ]\n"
            + "Opponent Hand Size: 3\n"
            + "Opponent Last Played: [8♣]\n"
            + "Play Stack: [ [A♦],[8♣] ]\n"
            + "******************************************";

        let result = display.game_during_play_to_string(&starter, &player_1, &player_2, &play_data);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_during_counting_to_string_with_crib() {
        let display = Display::new();

        let starter = Card::new(Rank::Four, Suit::Diamonds);
        let controller = PredeterminedController::from(vec![3]);

        let player_1_hand = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
        ];
        let crib = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let player_1 = Player::new_with_cards_and_crib(controller.clone(), player_1_hand, crib);

        let player_2_hand = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let player_2 = Player::new_with_cards(controller, player_2_hand);

        let expected = String::new()
            + "******************************************\n"
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[A♦],[6♦] ]\n"
            + "Player Crib: [ [A♣],[2♣],[5♦],[5♣] ]\n"
            + "Opponent Hand: [ [8♦],[K♦],[6♣],[8♣] ]\n"
            + "Opponent Hand Score: 2\n"
            + "Hand Score: 4\n"
            + "Crib Score: 4\n"
            + "******************************************";

        let result = display.game_during_counting_to_string(&starter, &player_1, &player_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_during_counting_to_string_opponent_crib() {
        let display = Display::new();

        let starter = Card::new(Rank::Four, Suit::Diamonds);
        let controller = PredeterminedController::from(vec![3]);

        let player_1_hand = vec![
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
        ];
        let mut player_1 = Player::new_with_cards(controller.clone(), player_1_hand);

        let player_2_hand = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let crib = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let mut player_2 = Player::new_with_cards_and_crib(controller, player_2_hand, crib);

        player_1.points += 8;
        player_2.points += 2;

        let expected = String::new()
            + "******************************************\n"
            + "Player Points: 8 | Opponent Points: 2\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[A♦],[6♦] ]\n"
            + "Opponent Hand: [ [8♦],[K♦],[6♣],[8♣] ]\n"
            + "Opponent Crib: [ [A♣],[2♣],[5♦],[5♣] ]\n"
            + "Opponent Hand Score: 2\n"
            + "Opponent Crib Score: 4\n"
            + "Hand Score: 4\n"
            + "******************************************";

        let result = display.game_during_counting_to_string(&starter, &player_1, &player_2);

        assert_eq!(result, expected);
    }
}
