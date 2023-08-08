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
