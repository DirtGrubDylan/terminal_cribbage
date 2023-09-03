//! Handles the display of the game.

#[cfg(doc)]
use crate::cards::Hand;

#[cfg(doc)]
use crate::game::Game;

use std::{thread, time};

use itertools::Itertools;

use crate::cards::Card;
use crate::game::{Controller, Display, PlayData, Player};

/// A struct for displaying the [`Game`] that uses the [`Display`] trait.
#[derive(Debug, PartialEq, Clone)]
pub struct UiDisplay {
    pub joiner: String,
    should_print: bool,
    post_print_delay_millis: time::Duration,
}

impl UiDisplay {
    /// Creates a new [`UiDisplay`] struct.
    #[must_use]
    pub fn new() -> UiDisplay {
        UiDisplay {
            joiner: String::from("\n"),
            should_print: false,
            post_print_delay_millis: time::Duration::from_millis(500),
        }
    }

    /// Turns on printing for [`Diplay`].
    pub fn turn_on_printing(&mut self, should_print: bool) {
        self.should_print = should_print;
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

impl Display for UiDisplay {
    /// Print message to `std::out` without a spacer or a delay.
    fn println_no_spacer_no_delay(&self, message: &str) {
        if self.should_print {
            println!("{message}");

            thread::sleep(self.post_print_delay_millis);
        }
    }

    /// Print message with spacer to `std::out` using [`thread::sleep`] with a delay after printing.
    fn println(&self, message: &str) {
        if self.should_print {
            println!("\n{}", Self::spacer());
            println!("{message}");

            thread::sleep(self.post_print_delay_millis);
        }
    }

    /// The [`String`] display for both [`Player`]s [`Card`]s cut from the [`Deck`].
    #[must_use]
    fn game_after_cut_message(
        &self,
        player_cut: &Card,
        opponent_cut: &Card,
        player_won: bool,
    ) -> String {
        let mut result = Vec::new();

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

        result.join(&self.joiner)
    }

    /// The [`String`] display for both [`Player`]s and the starter [`Card`] before play.
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib. If starter is [`None`], then `"[?]"`. The player's crib will only be displayed if they
    /// have one.
    #[must_use]
    fn game_before_play_message<C>(
        &self,
        starter: Option<&Card>,
        player: &Player<C>,
        opponent: &Player<C>,
    ) -> String
    where
        C: Controller,
    {
        let mut result = Vec::new();

        result.push(format!(
            "Player Points: {} | Opponent Points: {}",
            player.points, opponent.points
        ));
        result.push(format!("Starter: {}", Self::card_string(starter)));
        result.push(format!("Player Hand: {}", player.hand));

        if player.has_crib() {
            result.push(format!("Player Crib: {}", player.crib));
        }

        result.join(&self.joiner)
    }

    /// The [`String`] display for both [`Player`]s, the starter [`Card`], and [`PlayData`] during play.
    ///
    /// This will show the opponent's and player's points, but only show the player's [`Hand`] and
    /// crib. The player's crib will only be displayed if they have one.
    #[must_use]
    fn game_during_play_message<C>(
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

        result.join(&self.joiner)
    }

    /// The [`String`] display for both [`Player`]s and the starter [`Card`] during counting.
    ///
    /// This will show the opponent's and player's points, [`Hand`]s and cribs.
    #[must_use]
    fn game_during_counting_message<C>(
        &self,
        starter: &Card,
        player: &Player<C>,
        opponent: &Player<C>,
    ) -> String
    where
        C: Controller,
    {
        let mut result = Vec::new();

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

        result.join(&self.joiner)
    }

    /// The [`String`] display for game over.
    #[must_use]
    fn game_over_message(&self, player_won: bool) -> String {
        let mut result = Vec::new();

        if player_won {
            result.push("You Won!".to_string());
        } else {
            result.push("You Lost!".to_string());
        }

        result.join(&self.joiner)
    }
}

impl Default for UiDisplay {
    fn default() -> Self {
        UiDisplay::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cards::{Card, Rank, Suit};
    use crate::game::{PlayData, Player, PredeterminedController};

    #[test]
    fn test_game_after_cut_message_player_won() {
        let display = UiDisplay::new();

        let player_cut = Card::new(Rank::King, Suit::Clubs);
        let opponent_cut = Card::new(Rank::Eight, Suit::Spades);

        let expected = "Player Cut: [K♣]\nOpponent Cut: [8♠]\nPlayer Won Cut";

        let result =
            display.game_after_cut_message(&player_cut, &opponent_cut, /*player_won=*/ true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_after_cut_message_opponent_won() {
        let display = UiDisplay::new();

        let player_cut = Card::new(Rank::Eight, Suit::Spades);
        let opponent_cut = Card::new(Rank::King, Suit::Clubs);

        let expected = "Player Cut: [8♠]\nOpponent Cut: [K♣]\nOpponent Won Cut";

        let result =
            display.game_after_cut_message(&player_cut, &opponent_cut, /*player_won=*/ false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_before_play_message_discard_to_crib_no_starter() {
        let display = UiDisplay::new();

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
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [?]\n"
            + "Player Hand: [ [8♠],[K♣],[2♠],[6♦],[5♦],[5♣] ]";

        let result = display.game_before_play_message(starter, &player_1, &player_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_before_play_message_discard_to_crib_with_starter() {
        let display = UiDisplay::new();

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
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[2♠],[6♦],[5♦],[5♣] ]";

        let result = display.game_before_play_message(Some(&starter), &player_1, &player_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_before_play_message_with_crib_with_starter() {
        let display = UiDisplay::new();

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
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[2♠],[6♦] ]\n"
            + "Player Crib: [ [A♣],[2♣],[5♦],[5♣] ]";

        let result = display.game_before_play_message(Some(&starter), &player_1, &player_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_during_play_message_with_crib() {
        let display = UiDisplay::new();

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
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[6♦] ]\n"
            + "Player Crib: [ [A♣],[2♣],[5♦],[5♣] ]\n"
            + "Opponent Hand Size: 3\n"
            + "Opponent Last Played: [8♣]\n"
            + "Play Stack: [ [A♦],[8♣] ]";

        let result = display.game_during_play_message(&starter, &player_1, &player_2, &play_data);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_during_play_message_without_crib() {
        let display = UiDisplay::new();

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
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[6♦] ]\n"
            + "Opponent Hand Size: 3\n"
            + "Opponent Last Played: [8♣]\n"
            + "Play Stack: [ [A♦],[8♣] ]";

        let result = display.game_during_play_message(&starter, &player_1, &player_2, &play_data);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_during_counting_message_with_crib() {
        let display = UiDisplay::new();

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
            + "Player Points: 0 | Opponent Points: 0\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[A♦],[6♦] ]\n"
            + "Player Crib: [ [A♣],[2♣],[5♦],[5♣] ]\n"
            + "Opponent Hand: [ [8♦],[K♦],[6♣],[8♣] ]\n"
            + "Opponent Hand Score: 2\n"
            + "Hand Score: 4\n"
            + "Crib Score: 4";

        let result = display.game_during_counting_message(&starter, &player_1, &player_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_during_counting_message_opponent_crib() {
        let display = UiDisplay::new();

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
            + "Player Points: 8 | Opponent Points: 2\n"
            + "Starter: [4♦]\n"
            + "Player Hand: [ [8♠],[K♣],[A♦],[6♦] ]\n"
            + "Opponent Hand: [ [8♦],[K♦],[6♣],[8♣] ]\n"
            + "Opponent Crib: [ [A♣],[2♣],[5♦],[5♣] ]\n"
            + "Opponent Hand Score: 2\n"
            + "Opponent Crib Score: 4\n"
            + "Hand Score: 4";

        let result = display.game_during_counting_message(&starter, &player_1, &player_2);

        assert_eq!(result, expected);
    }
}
