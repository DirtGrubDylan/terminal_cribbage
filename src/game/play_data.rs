//! This module is for the Play (pegging) part of the game.
//!
//! It's not called pegging because I am immature.

#[cfg(doc)]
use crate::cards::Rank;

use crate::cards::Card;
use crate::game::{Controller, Player};

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
    /// Creates a new [`PlayData`] with an empty stack and a `0` stack_score.
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
    pub fn reset_if_needed<C>(&mut self, player_1: &Player<C>, player_2: &Player<C>)
    where
        C: Controller,
    {
        unimplemented!()
    }

    /// Indicates if [`Player`] has a [`Card`] to make a play.
    ///
    /// A play is only possible if the [`Player`] has a [`Card`] whose score summed with the stack
    /// score is less than, or equal to, `31`.
    pub fn can_play<C>(&self, player: &Player<C>) -> bool
    where
        C: Controller,
    {
        unimplemented!()
    }

    /// Indicates if any [`Player`] has a [`Card`] to make a play.
    ///
    /// A play is only possible if the [`Player`] has a [`Card`] whose score summed with the stack
    /// score is less than, or equal to, `31`.
    pub fn any_can_play<C>(&self, player_1: &Player<C>, player_2: &Player<C>) -> bool
    where
        C: Controller,
    {
        unimplemented!()
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
    pub fn play_once<C>(&mut self, player: &mut Player<C>)
    where
        C: Controller,
    {
        unimplemented!()
    }

    /// Returns `0` or `1` if neither [`Player`] can play.
    ///
    /// It's important to note, this is calculated **AFTER** a [`Player`] has played. Thus, the GO
    /// point is added to that [`Player`].
    ///
    /// Uses [`PlayData::any_can_play`].
    ///
    /// Maybe move this outside of the publc points?
    pub fn go_point(&self) -> u32 {
        unimplemented!()
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
    /// * Go (played last card) - 1pt
    /// * Flushes and Nobs count do not count.
    pub fn current_points(&self) -> u32 {
        unimplemented!()
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
    fn largest_run_points(&self) -> u32 {
        unimplemented!()
    }

    /// Returns `0` or `2` if the stack score is `15`.
    fn fifteen_points(&self) -> u32 {
        unimplemented!()
    }

    /// Returns `0` or `2` if the stack score is `31`.
    fn thirty_one_points(&self) -> u32 {
        unimplemented!()
    }

    /// Returns `0`, `2`, `6`, or `12` depending on the [`Rank`] matching of the top 2-4 [`Card`]s.
    ///
    /// Pairs are counted as:
    ///     * player 1 does a pair and gets 2pts
    ///     * player 2 does a three-of-a-kind and gets 6pts
    ///     * player 1 does a four-of-a-kind and gets 12pts
    fn pairs_points(&self) -> u32 {
        unimplemented!()
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

        input.into_iter().for_each(|card| {
            data.add_card(card);
        });

        data
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_largest_run_points_0() {
        unimplemented!()
    }

    #[test]
    fn test_largest_run_points_3() {
        unimplemented!()
    }

    #[test]
    fn test_largest_run_points_3_with_break_in_middle() {
        unimplemented!()
    }

    #[test]
    fn test_largest_run_points_4() {
        unimplemented!()
    }

    #[test]
    fn test_largest_run_points_5() {
        unimplemented!()
    }

    #[test]
    fn test_largest_run_points_6() {
        unimplemented!()
    }

    #[test]
    fn test_largest_run_points_7() {
        unimplemented!()
    }

    #[test]
    fn test_fifteen_points_0() {
        unimplemented!()
    }

    #[test]
    fn test_fifteen_points_2() {
        unimplemented!()
    }

    #[test]
    fn test_thirty_one_points_0() {
        unimplemented!()
    }

    #[test]
    fn test_thirty_one_points_2() {
        unimplemented!()
    }

    #[test]
    fn test_pairs_points_0() {
        unimplemented!()
    }

    #[test]
    fn test_pairs_points_2() {
        unimplemented!()
    }

    #[test]
    fn test_pairs_points_6() {
        unimplemented!()
    }

    #[test]
    fn test_pairs_points_12() {
        unimplemented!()
    }

    #[test]
    fn test_go_point_0() {
        unimplemented!()
    }

    #[test]
    fn test_go_point_1() {
        unimplemented!()
    }
}
