use cards::{Card, Hand};

/// This is just a table of all scores based on a [`Hand`] and "starter" [`Card`].
///
/// We can ignore single cards except if jack in [`Hand`] is same suit as "starter" [`Card`].
///
/// There are a total of 31 [`Card`] combinations:
///   * Combinations of 1 [`Card`] = 5
///   * Combinations of 2 [`Card`]s = 10
///   * Combinations of 3 [`Card`]s = 10
///   * Combinations of 4 [`Card`]s = 5
///   * Combinations of 5 [`Card`]s = 1
///
/// Just going to sum all the combinations.
///
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b768f384631c90f3cdbaad7446123d9c
///
/// let v: Vec<Temp> = vec![Temp::new(1), Temp::new(2), Temp::new(3), Temp::new(4), Temp::new(5)];
///
/// println!("{:?}\n", v);
///
/// let it: Vec<(Vec<Temp>, u32)> = (1..=5)
///     .flat_map(|i| v.iter().copied().combinations(i))
///     .map(|vec| (vec.clone(), vec.iter().fold(0, |acc, temp| acc + temp.score())))
///     .collect();
///
/// for (tv, tt) in it {
///     println!("{:?} -> {}", tv, tt);
/// }
///
/// let a1 = vec![1, 2, 3];
/// let a2 = vec![4, 5, 6];
/// let a3 = 7;
/// let a4: &Vec<usize> = &a1;
/// let mut iter = a4.iter().chain(a2.iter()).chain(iter::once(&a3));
///
/// [`Card`]: struct.Card.html
/// [`Hand`]: struct.Hand.html
///
/// Check for:
///   * 15s (combinations and sum scores) - 2pts each
///   * Four of a kind (combinations, just counting distinct pairs) - 12 pts
///   * Three of a kinds (combinations, just counting distinct pairs) - 6 pts
///   * Pair (combinations) - 2pts each (three of a kind is 3 distinct pairs, four is 6 pairs)
///   * 5 card run - 5pts
///   * 4 card run - 4pts
///   * 3 card run - 3pts (one point for every card in order past 2 cards)
///   * 5 card flush (hand + starter) - 5pts
///   * 4 card flush (hand, but not crib) - 4pts (if starter is same suit, +1 additional)
///   * Nobs (if jack in hand/crib matches starter suit) - 1pt
///
/// Keep in mind for play (not hand/crib):
///   * All points from above except flushes and nobs count during playing
///   * Runs can last as long as possible in play - 1pt per card in run
///       * Runs can go backwards or forwards and are not necessarily sequential
///       * 5 -> 4 -> 7 -> 6 is a four card run
///       * A -> 5 -> 3 -> 4 -> 6 -> 2 is a six card run
///       * 3-5 card runs are worth 3-5pts respectively
///           * player 1 does a 3 card run and gets 3 points
///           * player 2 does a 4 card run and gets 4 points
///           * player 1 does a 5 card run and gets 5 points
///       * 6+ runs are just worth a point per play
///           * player 2 does a 6 card run and gets 1 points
///           * player 1 does a 7 card run and gets 1 points
///   * Pairs are counted as:
///       * player 1 does a pair and gets 2 points
///       * player 2 does a three-of-a-kind and gets 6 points
///       * player 1 does a four-of-a-kind and gets 12 points
///   * 15s are only counted based on the previous card
///   * 31 (play stack total is 31) - 2pts
///   * Go (played last card) - 1pt
///   * His Heels (jack is starter and player is dealer) - 2pts
pub fn score_fifteens(hand: &Hand, starter: &Card) -> u32 {
    unimplemented!()
}

pub fn score_pairs(hand: &Hand, starter: &Card) -> u32 {
    unimplemented!()
}

pub fn score_runs(hand: &Hand, starter: &Card) -> u32 {
    unimplemented!()
}

pub fn score_flushes(hand: &Hand, starter: &Card, is_crib: bool) -> u32 {
    unimplemented!()
}

pub fn score_nobs(hand: &Hand, starter: &Card) -> u32 {
    unimplemented!()
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

        let score = score_fifteens(&hand, &starter);

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

        let score = score_fifteens(&hand, &starter);

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

        assert_eq!(score, 4);
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
