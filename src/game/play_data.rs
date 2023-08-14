//! This module is for the Play (pegging) part of the game.
//!
//! It's not called pegging because I am immature.

#[cfg(doc)]
use cards::{Rank, Suit};

use cards::Card;
use game::{Controller, Player};

/// Simple struct to keep track of the played stack of [`Card`]s and their running raw total score.
///
/// The stack and stack score are public for display purposes. Having getters and setters is dumb
/// when Rust natively handles mutablility.
#[derive(Debug, PartialEq)]
pub struct PlayData {
    pub stack: Vec<Card>,
    pub stack_score: u32,
}

impl PlayData {
    /// Creates a new [`PlayData`] with an empty stack and a `0` stack score.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::game::PlayData;
    ///
    /// let data = PlayData::new();
    /// ```
    #[must_use]
    pub fn new() -> PlayData {
        PlayData {
            stack: Vec::new(),
            stack_score: 0,
        }
    }

    /// Adds a [`Card`] to the stack and updates the stack score.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::PlayData;
    ///
    /// let card1 = Card::new(Rank::Ace, Suit::Hearts);
    /// let card2 = Card::new(Rank::Queen, Suit::Hearts);
    ///
    /// let expected = PlayData {
    ///     stack: vec![card1.clone(), card2.clone()],
    ///     stack_score: 11,
    /// };
    ///
    /// let mut data = PlayData::new();
    ///
    /// data.add_card(card1);
    /// data.add_card(card2);
    ///
    /// assert_eq!(data, expected);
    /// ```
    pub fn add_card(&mut self, card: Card) {
        self.stack_score += card.score();
        self.stack.push(card);
    }

    /// Resets [`PlayData`] if necessary.
    ///
    /// This is only needed if no [`Player`] can play **OR** the stack score is `31`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{PredeterminedController, PlayData, Player};
    ///
    /// // Going to discard the Two.
    /// let controller = PredeterminedController::from(vec![0]);
    ///
    /// let player_1_cards = vec![
    ///     Card::new(Rank::Five, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Spades),
    /// ];
    /// let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);
    ///
    /// let player_2_cards = vec![
    ///     Card::new(Rank::Six, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Diamonds),
    /// ];
    /// let player_2 = Player::new_with_cards(controller.clone(), player_2_cards);
    ///
    /// let stack = vec![
    ///     Card::new(Rank::King, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Hearts),
    ///     Card::new(Rank::Seven, Suit::Diamonds),
    /// ];
    /// let mut data = PlayData::from(stack);
    ///
    /// // Stack score isn't 31, but neither player can play.
    /// data.reset_if_needed(&player_1, &player_2);
    ///
    /// assert_eq!(data.stack, Vec::new());
    /// assert_eq!(data.stack_score, 0);
    /// ```
    pub fn reset_if_needed<C>(&mut self, player_1: &Player<C>, player_2: &Player<C>)
    where
        C: Controller,
    {
        if !self.any_can_play(player_1, player_2) {
            self.stack = Vec::new();
            self.stack_score = 0;
        }
    }

    /// Indicates if [`Player`] has a [`Card`] to make a play.
    ///
    /// A play is only possible if the [`Player`] has a [`Card`] whose score summed with the stack
    /// score is less than, or equal to, `31`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{PredeterminedController, PlayData, Player};
    ///
    /// // Going to discard the Two.
    /// let controller = PredeterminedController::from(vec![0]);
    ///
    /// let player_cards = vec![
    ///     Card::new(Rank::Two, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Spades),
    /// ];
    /// let mut player = Player::new_with_cards(controller, player_cards);
    ///
    /// let stack = vec![
    ///     Card::new(Rank::King, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Hearts),
    ///     Card::new(Rank::Ace, Suit::Diamonds),
    /// ];
    /// let mut data = PlayData::from(stack);
    ///
    /// // Can play either the 2 or the King.
    /// let could_play_before_discard = data.can_play(&player);
    ///
    /// let card_from_player = player.discard().unwrap();
    ///
    ///
    /// data.add_card(card_from_player.clone());
    ///
    /// assert_eq!(card_from_player, Card::new(Rank::Two, Suit::Clubs));
    /// assert!(could_play_before_discard);
    /// assert!(!data.can_play(&player));
    /// ```
    pub fn can_play<C>(&self, player: &Player<C>) -> bool
    where
        C: Controller,
    {
        let highest_possible_card_score = 31_u32.saturating_sub(self.stack_score);

        player.has_card_with_score_at_most(highest_possible_card_score)
    }

    /// Indicates if any [`Player`] has a [`Card`] to make a play.
    ///
    /// A play is only possible if the [`Player`] has a [`Card`] whose score summed with the stack
    /// score is less than, or equal to, `31`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{PredeterminedController, PlayData, Player};
    ///
    /// // Going to discard the Two.
    /// let controller = PredeterminedController::from(vec![0]);
    ///
    /// let player_1_cards = vec![
    ///     Card::new(Rank::Five, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Spades),
    /// ];
    /// let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);
    ///
    /// let player_2_cards = vec![
    ///     Card::new(Rank::Six, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Diamonds),
    /// ];
    /// let player_2 = Player::new_with_cards(controller.clone(), player_2_cards);
    ///
    /// let stack = vec![
    ///     Card::new(Rank::King, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Hearts),
    ///     Card::new(Rank::Seven, Suit::Diamonds),
    /// ];
    /// let data = PlayData::from(stack);
    ///
    /// // Stack score isn't 31, but neither player can play.
    /// assert!(!data.any_can_play(&player_1, &player_2));
    /// ```
    pub fn any_can_play<C>(&self, player_1: &Player<C>, player_2: &Player<C>) -> bool
    where
        C: Controller,
    {
        self.can_play(player_1) || self.can_play(player_2)
    }

    /// Plays a single round of play for a [`Player`].
    ///
    /// If the [`Player`] can play:
    /// * The [`Player`] discards a [`Card`], which is placed on the top of the stack.
    ///     * Using [`Player::discard`].
    /// * The stack total is increased by the score of that [`Card`].
    /// * The stack's points are totalled and added to the [`Player`].
    ///    * Using [`PlayData::current_points`].
    ///
    /// If the [`Player`] cannot play, they GO (pass their turn).
    ///
    /// # Panics
    ///
    /// * If the index at the front of [`Player::controller`] returns an index that is out of bounds
    /// for the [`Player::hand`].
    /// * If there is a [`Rank`] variant who's enum value is greater than `12`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{PredeterminedController, PlayData, Player};
    ///
    /// // Going to discard the Queen for 1 point from a "GO".
    /// let controller_1 = PredeterminedController::from(vec![0]);
    /// let player_1_cards = vec![
    ///     Card::new(Rank::Two, Suit::Clubs),
    /// ];
    /// let mut player_1 = Player::new_with_cards(controller_1, player_1_cards);
    ///
    /// // Going to discard the Queen for 31 points, but won't have a chance after Player 1 plays.
    /// let controller_2 = PredeterminedController::from(vec![0]);
    /// // Cannot play at all.
    /// let player_2_cards = vec![
    ///     Card::new(Rank::Queen, Suit::Clubs),
    /// ];
    /// let mut player_2 = Player::new_with_cards(controller_2, player_2_cards);
    ///
    /// let stack = vec![
    ///     Card::new(Rank::King, Suit::Clubs),
    ///     Card::new(Rank::King, Suit::Hearts),
    ///     Card::new(Rank::Ace, Suit::Diamonds),
    /// ];
    /// let mut data = PlayData::from(stack);
    ///
    /// data.play_once(&mut player_1, &player_2);
    /// data.play_once(&mut player_2, &player_1);
    ///
    /// // Player 1 got 1 points for a GO and has 1 less card in their hand.
    /// assert_eq!(player_1.points, 1);
    /// assert!(!player_1.has_cards());
    /// // Player 2 cannot play after Player 1 not get to play
    /// assert_eq!(player_2.points, 0);
    /// assert!(player_2.has_cards());
    /// ```
    pub fn play_once<C>(&mut self, player: &mut Player<C>, opponent: &Player<C>)
    where
        C: Controller,
    {
        if self.can_play(player) {
            let card_from_players_hand = player.discard().unwrap();

            self.add_card(card_from_players_hand);

            player.points += self.current_points();

            player.points += self.go_point(player, opponent);
        }
    }

    /// Calculates the current points of the stack.
    ///
    /// Possible opimization is to calculate points in [`PlayData::add_card`].
    ///
    /// Points are calculated as such:
    /// * Runs can last as long as possible in play - 1pt per card in run
    ///     * Runs can go backwards or forwards and are not necessarily sequential
    ///     * 5 -> 4 -> 7 -> 6 is a four card run
    ///     * A -> 5 -> 3 -> 4 -> 6 -> 2 -> 7 is a seven card run
    ///     * 3-7 card runs are worth 3-7pts respectively
    ///         * player 1 does a 3 card run and gets 3pts
    ///         * player 2 does a 4 card run and gets 4pts
    ///         * player 1 does a 5 card run and gets 5pts
    /// * Pairs are counted as:
    ///     * player 1 does a pair and gets 2pts
    ///     * player 2 does a three-of-a-kind and gets 6pts
    ///     * player 1 does a four-of-a-kind and gets 12pts
    /// * 15 (stack score is `15`) - 2pts
    /// * 31 (stack score is `31`) - 2pts
    /// * Go (played last card) (not counted here) - 1pt
    /// * Flushes and Nobs count do not count.
    ///
    /// # Panics
    ///
    /// If there is a [`Rank`] variant who's enum value is greater than `12`.
    fn current_points(&self) -> u32 {
        self.largest_run_points()
            + self.pairs_points()
            + self.fifteen_points()
            + self.thirty_one_points()
    }

    /// Returns `0` or `1` if neither [`Player`] can play.
    ///
    /// It's important to note, this is calculated **AFTER** a [`Player`] has played. Thus, the GO
    /// point is added to that [`Player`].
    ///
    /// Uses [`PlayData::any_can_play`].
    ///
    /// # Panics
    ///
    /// If, for some reason, a [`bool`] cannot be converted to a [`u32`].
    fn go_point<C>(&self, player_1: &Player<C>, player_2: &Player<C>) -> u32
    where
        C: Controller,
    {
        u32::from(!self.any_can_play(player_1, player_2) && (self.stack_score != 31))
    }

    /// Counts the largest sequential run from the [`Card`] at the top of the stack
    ///
    /// Runs can last as long as possible in play - 1pt per card in run:
    /// * Runs can go backwards or forwards and are not necessarily sequential
    /// * 5 -> 4 -> 7 -> 6 is a four card run
    /// * A -> 5 -> 3 -> 4 -> 6 -> 2 -> 7 is a seven card run
    /// * 3-7 card runs are worth 3-7pts respectively
    ///     * player 1 does a 3 card run and gets 3pts
    ///     * player 2 does a 4 card run and gets 4pts
    ///     * player 1 does a 5 card run and gets 5pts
    ///
    /// # Panics
    ///
    /// Panics if there is a [`Rank`] variant who's enum value is greater than `12`.
    fn largest_run_points(&self) -> u32 {
        if self.stack.len() < 3 {
            return 0;
        }

        // This is a way to keep track of which ranks we have found using the enum to usize
        // conversion.
        // Rank::Ace is mapped to index 0 and Rank::King is mapped to index 12
        let mut seven_run = [0; 13];
        let mut six_run = [0; 13];
        let mut five_run = [0; 13];
        let mut four_run = [0; 13];
        let mut three_run = [0; 13];

        let top_card_index = self.stack.len() - 1;
        let top_card = self.stack.last().unwrap();

        for (index, card) in self.stack.iter().enumerate() {
            if Self::can_make_run_of(index, card, top_card_index, top_card, /*run_size=*/ 7) {
                Self::add_rank_to_array(&mut seven_run, card);
            }

            if Self::can_make_run_of(index, card, top_card_index, top_card, /*run_size=*/ 6) {
                Self::add_rank_to_array(&mut six_run, card);
            }

            if Self::can_make_run_of(index, card, top_card_index, top_card, /*run_size=*/ 5) {
                Self::add_rank_to_array(&mut five_run, card);
            }

            if Self::can_make_run_of(index, card, top_card_index, top_card, /*run_size=*/ 4) {
                Self::add_rank_to_array(&mut four_run, card);
            }

            if Self::can_make_run_of(index, card, top_card_index, top_card, /*run_size=*/ 3) {
                Self::add_rank_to_array(&mut three_run, card);
            }
        }

        if Self::is_run_of(&seven_run, 7) {
            7
        } else if Self::is_run_of(&six_run, 6) {
            6
        } else if Self::is_run_of(&five_run, 5) {
            5
        } else if Self::is_run_of(&four_run, 4) {
            4
        } else if Self::is_run_of(&three_run, 3) {
            3
        } else {
            0
        }
    }

    /// Helper method for [`largest_run_points`] to check if a card can be in run of given size.
    fn can_make_run_of(
        card_index: usize,
        card: &Card,
        last_card_index: usize,
        last_card: &Card,
        run_size: usize,
    ) -> bool {
        let card_rank_value = card.rank as usize;
        let last_card_rank_value = last_card.rank as usize;

        let index_diff = last_card_index.abs_diff(card_index);
        let rank_value_diff = last_card_rank_value.abs_diff(card_rank_value);

        (index_diff < run_size) && (rank_value_diff < run_size)
    }

    /// Helper method for [`largest_run_points`] to check if an array can score the given points.
    fn is_run_of(rank_array: &[u32], points: u32) -> bool {
        let mut current_run = 0;

        for rank_count in rank_array {
            if 0 < *rank_count {
                current_run += 1;
            } else if 0 < current_run {
                break;
            }
        }

        current_run == points
    }

    /// Helper method for [`largest_run_points`] to add [`Card`] [`Rank`] to counting array.
    ///
    /// # Panics
    ///
    /// Panics if there is a [`Rank`] variant who's enum value is greater than `12`.
    fn add_rank_to_array(rank_array: &mut [u32], card: &Card) {
        match rank_array.get_mut(card.rank as usize) {
            Some(count) => *count += 1,
            None => panic!("Rank {:?} not handled", card.rank),
        }
    }

    /// Returns `0` or `2` if the stack score is `15`.
    fn fifteen_points(&self) -> u32 {
        if self.stack_score == 15 {
            2
        } else {
            0
        }
    }

    /// Returns `0` or `2` if the stack score is `31`.
    fn thirty_one_points(&self) -> u32 {
        if self.stack_score == 31 {
            2
        } else {
            0
        }
    }

    /// Returns `0`, `2`, `6`, or `12` depending on the [`Rank`] matching of the top 2-4 [`Card`]s.
    ///
    /// Pairs are counted as:
    ///     * player 1 does a pair and gets 2pts
    ///     * player 2 does a three-of-a-kind and gets 6pts
    ///     * player 1 does a four-of-a-kind and gets 12pts
    fn pairs_points(&self) -> u32 {
        if self.stack.len() < 2 {
            return 0;
        }

        let mut matching_ranks = 0;
        let top_card_rank = self.stack.last().unwrap().rank;

        for card in self.stack.iter().rev().take(4).skip(1) {
            if card.rank == top_card_rank {
                matching_ranks += 1;
            } else {
                break;
            }
        }

        match matching_ranks {
            3 => 12,
            2 => 6,
            1 => 2,
            _ => 0,
        }
    }
}

impl Default for PlayData {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Card>> for PlayData {
    /// Convert from [`Vec`] of [`Card`]s.
    ///
    /// Mainly used for testing. This updates the stack and the stack score.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::PlayData;
    ///
    /// let cards = vec![Card::new(Rank::Ace, Suit::Hearts), Card::new(Rank::Ace, Suit::Clubs)];
    ///
    /// let expected = PlayData {
    ///     stack: cards.clone(),
    ///     stack_score: 2,
    /// };
    ///
    /// let result = PlayData::from(cards);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn from(input: Vec<Card>) -> PlayData {
        let mut data = PlayData::new();

        for card in input {
            data.add_card(card);
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cards::{Card, Rank, Suit},
        game::PredeterminedController,
    };

    #[test]
    fn test_can_make_run_of_index_diff_false() {
        let run_size = 4;
        let last_card_index = 6;
        let last_card = Card::new(Rank::Two, Suit::Clubs);
        let card_index = 2;
        let card = Card::new(Rank::Five, Suit::Clubs);

        assert!(!PlayData::can_make_run_of(
            card_index,
            &card,
            last_card_index,
            &last_card,
            run_size
        ));
    }

    #[test]
    fn test_can_make_run_of_rank_diff_false() {
        let run_size = 4;
        let last_card_index = 6;
        let last_card = Card::new(Rank::Two, Suit::Clubs);
        let card_index = 3;
        let card = Card::new(Rank::Six, Suit::Clubs);

        assert!(!PlayData::can_make_run_of(
            card_index,
            &card,
            last_card_index,
            &last_card,
            run_size
        ));
    }

    #[test]
    fn test_can_make_run_of_true() {
        let run_size = 3;
        let last_card_index = 7;
        let last_card = Card::new(Rank::Ace, Suit::Clubs);
        let card_index = 5;
        let card = Card::new(Rank::Three, Suit::Clubs);

        assert!(PlayData::can_make_run_of(
            card_index,
            &card,
            last_card_index,
            &last_card,
            run_size
        ));
    }

    #[test]
    fn test_can_make_run_of_larger_run_than_diffs_true() {
        let run_size = 7;
        let last_card_index = 7;
        let last_card = Card::new(Rank::Ace, Suit::Clubs);
        let card_index = 5;
        let card = Card::new(Rank::Three, Suit::Clubs);

        assert!(PlayData::can_make_run_of(
            card_index,
            &card,
            last_card_index,
            &last_card,
            run_size
        ));
    }

    #[test]
    fn test_is_run_of_has_gaps_no_multiple_false() {
        let points = 7;
        let run = [0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0];

        assert!(!PlayData::is_run_of(&run, points));
    }

    #[test]
    fn test_is_run_of_no_gaps_has_multiple_false() {
        let points = 7;
        let run = [1, 2, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];

        assert!(!PlayData::is_run_of(&run, points));
    }

    #[test]
    fn test_is_run_of_no_gap_no_multiple_run_too_small_false() {
        let points = 4;
        let run = [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert!(!PlayData::is_run_of(&run, points));
    }

    #[test]
    fn test_is_run_of_true() {
        let points = 3;
        let run = [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert!(PlayData::is_run_of(&run, points));
    }

    #[test]
    fn test_largest_run_points_stack_too_small_0() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_largest_run_points_top_of_stack_not_run_0() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_largest_run_points_top_of_stack_not_run_long_0() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_largest_run_points_3() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 3);
    }

    #[test]
    fn test_largest_run_points_with_break_in_middle_3() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Two, Suit::Hearts),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 3);
    }

    #[test]
    fn test_largest_run_points_4() {
        let cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_largest_run_points_5() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 5);
    }

    #[test]
    fn test_largest_run_points_6() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 6);
    }

    #[test]
    fn test_largest_run_points_7() {
        let cards = vec![
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];

        let data = PlayData::from(cards);

        let result = data.largest_run_points();

        assert_eq!(result, 7);
    }

    #[test]
    fn test_fifteen_points_0() {
        let cards = vec![Card::new(Rank::King, Suit::Clubs)];

        let data = PlayData::from(cards);

        let result = data.fifteen_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_fifteen_points_2() {
        let cards = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
        ];

        let data = PlayData::from(cards);

        let result = data.fifteen_points();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_thirty_one_points_0() {
        let cards = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Spades),
        ];

        let data = PlayData::from(cards);

        let result = data.thirty_one_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_thirty_one_points_2() {
        let cards = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let data = PlayData::from(cards);

        let result = data.thirty_one_points();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_pairs_points_stack_too_small_0() {
        let cards = vec![Card::new(Rank::King, Suit::Clubs)];

        let data = PlayData::from(cards);

        let result = data.pairs_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_pairs_points_0() {
        let cards = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let data = PlayData::from(cards);

        let result = data.pairs_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_pairs_points_2() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
        ];

        let data = PlayData::from(cards);

        let result = data.pairs_points();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_pairs_points_6() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Spades),
        ];

        let data = PlayData::from(cards);

        let result = data.pairs_points();

        assert_eq!(result, 6);
    }

    #[test]
    fn test_pairs_points_12() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Diamonds),
        ];

        let data = PlayData::from(cards);

        let result = data.pairs_points();

        assert_eq!(result, 12);
    }

    #[test]
    fn test_go_point_player_1_can_play_0() {
        let controller = PredeterminedController::from(vec![]);

        let player_1_cards = vec![Card::new(Rank::Ace, Suit::Clubs)];
        let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);

        let player_2_cards = Vec::new();
        let player_2 = Player::new_with_cards(controller.clone(), player_2_cards);

        let stack = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let data = PlayData::from(stack);

        let result = data.go_point(&player_1, &player_2);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_go_point_player_2_can_play_0() {
        let controller = PredeterminedController::from(vec![]);

        let player_1_cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);

        let player_2_cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let player_2 = Player::new_with_cards(controller.clone(), player_2_cards);

        let stack = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let data = PlayData::from(stack);

        let result = data.go_point(&player_1, &player_2);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_go_point_neither_can_play_stack_score_thirty_one_0() {
        let controller = PredeterminedController::from(vec![]);

        let player_1_cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);

        let player_2_cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let player_2 = Player::new_with_cards(controller.clone(), player_2_cards);

        let stack = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Diamonds),
        ];
        let data = PlayData::from(stack);

        let result = data.go_point(&player_1, &player_2);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_go_point_neither_player_can_play_stack_score_thirty_1() {
        let controller = PredeterminedController::from(vec![]);

        let player_1_cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let player_1 = Player::new_with_cards(controller.clone(), player_1_cards);

        let player_2_cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let player_2 = Player::new_with_cards(controller.clone(), player_2_cards);

        let stack = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let data = PlayData::from(stack);

        let result = data.go_point(&player_1, &player_2);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_current_points_0() {
        let stack = vec![
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
        ];
        let data = PlayData::from(stack);

        let result = data.current_points();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_current_points_pair_and_fifteen_4() {
        let stack = vec![
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
        ];
        let data = PlayData::from(stack);

        let result = data.current_points();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_current_points_three_of_a_kind_and_thirty_one_8() {
        let stack = vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Hearts),
        ];
        let data = PlayData::from(stack);

        let result = data.current_points();

        assert_eq!(result, 8);
    }
}
