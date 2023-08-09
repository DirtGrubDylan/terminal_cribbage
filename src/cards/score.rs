//! This is just a table of all scores based on a [`Hand`] and "starter" [`Card`].
//!
//! We can ignore single cards except if jack in [`Hand`] is same suit as "starter" [`Card`].
//!
//! There are a total of 31 [`Card`] combinations:
//!   * Combinations of 1 [`Card`] = 5
//!   * Combinations of 2 [`Card`]s = 10
//!   * Combinations of 3 [`Card`]s = 10
//!   * Combinations of 4 [`Card`]s = 5
//!   * Combinations of 5 [`Card`]s = 1
//!
//! Keep in mind for play (not hand/crib):
//!   * All points from above except flushes and nobs count during playing
//!   * Runs can last as long as possible in play - 1pt per card in run
//!       * Runs can go backwards or forwards and are not necessarily sequential
//!       * 5 -> 4 -> 7 -> 6 is a four card run
//!       * A -> 5 -> 3 -> 4 -> 6 -> 2 is a six card run
//!       * 3-5 card runs are worth 3-5pts respectively
//!           * player 1 does a 3 card run and gets 3 points
//!           * player 2 does a 4 card run and gets 4 points
//!           * player 1 does a 5 card run and gets 5 points
//!       * 6+ runs are just worth a point per play
//!           * player 2 does a 6 card run and gets 1 points
//!           * player 1 does a 7 card run and gets 1 points
//!   * Pairs are counted as:
//!       * player 1 does a pair and gets 2 points
//!       * player 2 does a three-of-a-kind and gets 6 points
//!       * player 1 does a four-of-a-kind and gets 12 points
//!   * 15s are only counted based on the previous card
//!   * 31 (play stack total is 31) - 2pts
//!   * Go (played last card) - 1pt
//!   * His Heels (jack is starter and player is dealer) - 2pts
//!
//! [`Card`]: struct.Card.html
//! [`Hand`]: struct.Hand.html
use itertools::Itertools;
use std::iter;

use cards::{Card, Hand, Rank, Suit};

/// Returns a positive score if combinations of [`Cards`] scores in [`Hand`] total to 15.
///
/// This counts all combinations of 2, 3, 4, and 5 cards.
///
/// A [`Card`] score is based on [`Card::score`].
///
/// [`Hand`]: struct.Hand.html
/// [`Card`]: struct.Card.html
///
/// # Examples
///
/// ```
/// use libterminal_cribbage::cards::{Card, Hand, Rank, Suit};
/// use libterminal_cribbage::cards::score;
///
/// let cards = vec![
///     Card::new(Rank::Jack, Suit::Clubs),
///     Card::new(Rank::Five, Suit::Diamonds),
///     Card::new(Rank::Five, Suit::Hearts),
///     Card::new(Rank::Five, Suit::Spades),
/// ];
///
/// let starter = Card::new(Rank::Five, Suit::Clubs);
///
/// let hand = Hand::from(cards);
///
/// let score = score::score_fifteens(&hand, &starter);
///
/// assert_eq!(score, 16);
/// ```
pub fn score_fifteens(hand: &Hand, starter: &Card) -> u32 {
    let score_per_fifteen = 2;

    let hand_starter_iter = hand.as_vec().iter().chain(iter::once(starter));

    let number_of_fifteen_sums = (1..=5)
        .flat_map(|combination_value| hand_starter_iter.clone().combinations(combination_value))
        .map(|combination| combination.iter().fold(0, |acc, card| acc + card.score()))
        .filter(|score_sum| *score_sum == 15)
        .count();

    score_per_fifteen * (number_of_fifteen_sums as u32)
}

/// Returns a positive score if the [`Cards`] in [`Hand`] with the starter match [`Rank`].
///
/// This counts all pairs matching [`Rank`]s in the [`Card`]s. A three-of-a-kind is 3 pairs.
/// While a four-of-a-kind is 6 pairs.
///
/// [`Hand`]: struct.Hand.html
/// [`Card`]: struct.Card.html
/// [`Rank`]: enum.Rank.html
///
/// # Examples
///
/// ```
/// use libterminal_cribbage::cards::{Card, Hand, Rank, Suit};
/// use libterminal_cribbage::cards::score;
///
/// let cards = vec![
///     Card::new(Rank::Six, Suit::Clubs),
///     Card::new(Rank::Four, Suit::Clubs),
///     Card::new(Rank::Four, Suit::Hearts),
///     Card::new(Rank::Four, Suit::Spades),
/// ];
///
/// let starter_1 = Card::new(Rank::Four, Suit::Diamonds);
/// let starter_2 = Card::new(Rank::Six, Suit::Diamonds);
///
/// let hand = Hand::from(cards);
///
/// let score_1 = score::score_pairs(&hand, &starter_1);
/// let score_2 = score::score_pairs(&hand, &starter_2);
///
/// assert_eq!(score_1, 12);
/// assert_eq!(score_2, 8);
/// ```
pub fn score_pairs(hand: &Hand, starter: &Card) -> u32 {
    let score_per_pair = 2;

    let number_of_matching_pairs = hand
        .as_vec()
        .iter()
        .chain(iter::once(starter))
        .tuple_combinations()
        .filter(|(card_1, card_2)| card_1.rank == card_2.rank)
        .count();

    score_per_pair * (number_of_matching_pairs as u32)
}

/// Returns a positive score if the [`Cards`] in [`Hand`] with the starter is sequential.
///
/// This can be mutiplicative if there are matching [`Rank`]s in the [`Card`]s.
///
/// [`Hand`]: struct.Hand.html
/// [`Card`]: struct.Card.html
/// [`Rank`]: enum.Rank.html
///
/// # Examples
///
/// ```
/// use libterminal_cribbage::cards::{Card, Hand, Rank, Suit};
/// use libterminal_cribbage::cards::score;
///
/// let cards = vec![
///     Card::new(Rank::Six, Suit::Clubs),
///     Card::new(Rank::Four, Suit::Clubs),
///     Card::new(Rank::Four, Suit::Hearts),
///     Card::new(Rank::Three, Suit::Clubs),
/// ];
///
/// let starter_1 = Card::new(Rank::Two, Suit::Clubs);
/// let starter_2 = Card::new(Rank::Three, Suit::Clubs);
///
/// let hand = Hand::from(cards);
///
/// let score_1 = score::score_runs(&hand, &starter_1);
/// let score_2 = score::score_runs(&hand, &starter_2);
///
/// assert_eq!(score_1, 6);
/// assert_eq!(score_2, 0);
/// ```
pub fn score_runs(hand: &Hand, starter: &Card) -> u32 {
    let mut score = 0;
    let mut max_multiplier = 1;
    let mut max_run = 0;
    let mut current_run = 0;

    // This is a way to keep track of which ranks we have found using the enum to usize conversion.
    // Rank::Ace is mapped to index 0 and Rank::King is mapped to index 12
    let mut ranks_found = [0; 13];

    hand.as_vec()
        .iter()
        .chain(iter::once(starter))
        .for_each(|card| match ranks_found.get_mut(card.rank as usize) {
            Some(count) => *count += 1,
            None => panic!("Rank {:?} not handled", card.rank),
        });

    for current_multiplier in ranks_found {
        current_run += 1;

        // If we haven't found a rank reset the current_run.
        if current_multiplier == 0 {
            current_run = 0;

            // If we also haven't found a run so far reset the max_multiplier.
            if max_run < 3 {
                max_multiplier = 1;
            }
        }

        if current_run > max_run {
            max_run = current_run;
            max_multiplier *= current_multiplier;
        }
    }

    if 3 <= max_run {
        score = max_run;
    }

    max_multiplier * score
}

/// Returns `0`, `4`, or `5` based on the [`Suit`]s of the [`Hand`] and starter [`Card`].
///
/// This is called a flush. If all the [`Card`]s in the [`Hand`] have the same [`Suit`],
/// then the score is `4`. If the starter [`Card`] also matches that [`Suit`], then the
/// score is `5`. However, if this is for a "crib" [`Hand`], then all [`Card`]s must match,
/// including the starter; otherwise, the score is `0`, even if all [`Card`]s in the
/// [`Hand`] match.
///
/// [`Hand`]: struct.Hand.html
/// [`Card`]: struct.Card.html
/// [`Suit`]: enum.Suit.html
///
/// # Examples
///
/// ```
/// use libterminal_cribbage::cards::{Card, Hand, Rank, Suit};
/// use libterminal_cribbage::cards::score;
///
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Clubs),
///     Card::new(Rank::Two, Suit::Clubs),
///     Card::new(Rank::Three, Suit::Clubs),
///     Card::new(Rank::Four, Suit::Clubs),
/// ];
///
/// let starter = Card::new(Rank::Five, Suit::Spades);
///
/// let hand = Hand::from(cards);
///
/// let crib_score = score::score_flushes(&hand, &starter, /*is_crib=*/ true);
/// let hand_score = score::score_flushes(&hand, &starter, /*is_crib=*/ false);
///
/// assert_eq!(crib_score, 0);
/// assert_eq!(hand_score, 4);
/// ```
pub fn score_flushes(hand: &Hand, starter: &Card, is_crib: bool) -> u32 {
    let hand_vec = hand.as_vec();

    let target_suit = hand_vec.get(0).map_or(Suit::Clubs, |card| card.suit);

    let all_suits_match = hand_vec.iter().all(|card| card.suit == target_suit);

    let starter_suit_matches = starter.suit == target_suit;

    if all_suits_match && starter_suit_matches {
        5
    } else if all_suits_match && !is_crib {
        4
    } else {
        0
    }
}

/// Returns `0` or `1` depending on a [`Rank::Jack`] in the [`Hand`] matching the starter [`Suit`].
///
/// This is called "Nobs".
///
/// [`Hand`]: struct.Hand.html
/// [`Rank`]: enum.Rank.html
/// [`Suit`]: enum.Suit.html
///
/// # Examples
///
/// ```
/// use libterminal_cribbage::cards::{Card, Hand, Rank, Suit};
/// use libterminal_cribbage::cards::score;
///
/// let cards = vec![
///     Card::new(Rank::Five, Suit::Clubs),
///     Card::new(Rank::Five, Suit::Hearts),
///     Card::new(Rank::Five, Suit::Diamonds),
///     Card::new(Rank::Jack, Suit::Spades),
/// ];
///
/// let starter = Card::new(Rank::Five, Suit::Spades);
///
/// let hand = Hand::from(cards);
///
/// let score = score::score_nobs(&hand, &starter);
///
/// assert_eq!(score, 1);
/// ```
pub fn score_nobs(hand: &Hand, starter: &Card) -> u32 {
    let target_jack = Card::new(Rank::Jack, starter.suit);

    hand.as_vec().iter().any(|card| *card == target_jack) as u32
}

#[cfg(test)]
mod test {
    use super::*;
    use cards::{Card, Hand, Rank, Suit};

    #[test]
    fn score_fifteens_0() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let starter = Card::new(Rank::King, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_fifteens(&hand, &starter);

        assert_eq!(score, 0);
    }

    #[test]
    fn score_fifteens_2() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
        ];

        let starter = Card::new(Rank::King, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_fifteens(&hand, &starter);

        assert_eq!(score, 2);
    }

    #[test]
    fn score_fifteens_16() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_fifteens(&hand, &starter);

        assert_eq!(score, 16);
    }

    #[test]
    fn score_pairs_0() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_pairs(&hand, &starter);

        assert_eq!(score, 0);
    }

    #[test]
    fn score_pairs_one_pair_without_starter_2() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Two, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_pairs(&hand, &starter);

        assert_eq!(score, 2);
    }

    #[test]
    fn score_pairs_one_pair_with_starter_2() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_pairs(&hand, &starter);

        assert_eq!(score, 2);
    }

    #[test]
    fn score_pairs_two_pair_4() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Ace, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_pairs(&hand, &starter);

        assert_eq!(score, 4);
    }

    #[test]
    fn score_pairs_three_of_a_kind_6() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Ace, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_pairs(&hand, &starter);

        assert_eq!(score, 6);
    }

    #[test]
    fn score_pairs_four_of_a_kind_12() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_pairs(&hand, &starter);

        assert_eq!(score, 12);
    }

    #[test]
    fn score_runs_0() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 0);
    }

    #[test]
    fn score_runs_three_card_run_3() {
        let cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Ace, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 3);
    }

    #[test]
    fn score_runs_two_three_card_runs_without_starter_6() {
        let cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let starter = Card::new(Rank::Ace, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 6);
    }

    #[test]
    fn score_runs_two_three_card_runs_with_starter_6() {
        let cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 6);
    }

    #[test]
    fn score_runs_four_three_card_runs_with_starter_12() {
        let cards = vec![
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 12);
    }

    #[test]
    fn score_runs_four_card_run_without_starter_4() {
        let cards = vec![
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Spades),
        ];

        let starter = Card::new(Rank::Ace, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 4);
    }

    #[test]
    fn score_runs_four_card_run_with_starter_4() {
        let cards = vec![
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Six, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 4);
    }

    #[test]
    fn score_runs_two_four_card_runs_4() {
        let cards = vec![
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Spades),
        ];

        let starter = Card::new(Rank::Three, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 8);
    }

    #[test]
    fn score_runs_five_card_run_5() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let starter = Card::new(Rank::Ace, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_runs(&hand, &starter);

        assert_eq!(score, 5);
    }

    #[test]
    fn score_flushes_four_card_flush_not_crib_flush_on_starter_0() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Clubs);

        let hand = Hand::from(cards);

        let score = score_flushes(&hand, &starter, /*is_crib=*/ false);

        assert_eq!(score, 0);
    }

    #[test]
    fn score_flushes_four_card_flush_not_crib_4() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_flushes(&hand, &starter, /*is_crib=*/ false);

        assert_eq!(score, 4);
    }

    #[test]
    fn score_flushes_four_card_flush_crib_0() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_flushes(&hand, &starter, /*is_crib=*/ true);

        assert_eq!(score, 0);
    }

    #[test]
    fn score_flushes_five_card_flush_not_crib_5() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
        ];

        let starter = Card::new(Rank::Ace, Suit::Clubs);

        let hand = Hand::from(cards);

        let score = score_flushes(&hand, &starter, /*is_crib=*/ false);

        assert_eq!(score, 5);
    }

    #[test]
    fn score_flushes_five_card_flush_crib_5() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
        ];

        let starter = Card::new(Rank::Ace, Suit::Clubs);

        let hand = Hand::from(cards);

        let score = score_flushes(&hand, &starter, /*is_crib=*/ true);

        assert_eq!(score, 5);
    }

    #[test]
    fn score_nobs_no_jack_0() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_nobs(&hand, &starter);

        assert_eq!(score, 0);
    }

    #[test]
    fn score_nobs_no_matching_jack_0() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Diamonds);

        let hand = Hand::from(cards);

        let score = score_nobs(&hand, &starter);

        assert_eq!(score, 0);
    }

    #[test]
    fn score_nobs_matching_jack_1() {
        let cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let starter = Card::new(Rank::Five, Suit::Spades);

        let hand = Hand::from(cards);

        let score = score_nobs(&hand, &starter);

        assert_eq!(score, 1);
    }
}
