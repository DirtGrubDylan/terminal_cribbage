//! This module holds the logic for the gameplay:
//! * Choose dealer
//! * Deal [`Hand`]s to [`Player`]s
//! * Discards from [`Player`]s for dealer crib
//! * Get starter [`Card`] from [`Deck`]
//! * Play (peg)
//! * Count [`Hand`]s
//! * Repeat until one [`Player`] reaches 121pts

mod controller;
mod display;
mod play_data;
mod player;
mod predetermined_controller;

pub use self::controller::Controller;
pub use self::display::Display;
pub use self::play_data::PlayData;
pub use self::player::Player;
pub use self::predetermined_controller::PredeterminedController;

#[cfg(doc)]
use crate::cards::Suit;

use crate::cards::{Card, Deck, Hand, Rank};

/// The struct holding all the necessary data for playing a game of cribbage.
#[derive(Debug, PartialEq)]
pub struct Game<C>
where
    C: Controller + Clone + std::fmt::Debug,
{
    dealer: Player<C>,
    pone: Player<C>,
    deck: Deck,
    display: Display,
}

impl<C> Game<C>
where
    C: Controller + Clone + std::fmt::Debug,
{
    /// Creates a new [`Game`] with given [`Player`]s.
    ///
    /// The [`Deck`] is created with the [`Deck::new`] function, and then shuffled.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::game::{Game, Player, PredeterminedController};
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let player_1 = Player::new(controller.clone());
    /// let player_2 = Player::new(controller);
    ///
    /// let game = Game::new(player_1, player_2);
    /// ```
    pub fn new(player_1: Player<C>, player_2: Player<C>) -> Game<C> {
        let mut deck = Deck::new();

        deck.shuffle();

        Game {
            dealer: player_1,
            pone: player_2,
            deck,
            display: Display::new(),
        }
    }

    /// Creates a new [`Game`] with given [`Player`]s and [`Deck`].
    ///
    /// This is intended to be used for testing.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Deck, Rank, Suit};
    /// use libterminal_cribbage::game::{Game, Player, PredeterminedController};
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let player_1 = Player::new(controller.clone());
    /// let player_2 = Player::new(controller);
    ///
    /// let deck_cards = vec![
    ///     Card::new(Rank::Five, Suit::Clubs),
    ///     Card::new(Rank::Four, Suit::Diamonds),
    ///     Card::new(Rank::Three, Suit::Hearts),
    /// ];
    /// let deck = Deck::new_with_cards(deck_cards);
    ///
    /// let game = Game::new_with_deck(player_1, player_2, deck.clone());
    /// ```
    pub fn new_with_deck(player_1: Player<C>, player_2: Player<C>, deck: Deck) -> Game<C> {
        Game {
            dealer: player_1,
            pone: player_2,
            deck,
            display: Display::new(),
        }
    }

    /// Play the default game.
    ///
    /// This is simply calls [`Game::play`], but with `reset_with_deck` set to [`None`].
    ///
    /// # Panics
    ///
    /// * If there have been 1,000 rounds, indicating that the game is broken and can't end loop.
    /// * If the [`Player::controller`] returns an index that is out of bounds of the [`Deck`].
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use libterminal_cribbage::game::{Game, Player, PredeterminedController};
    ///
    /// let controller = PredeterminedController::from(vec![0, 1, 2]);
    ///
    /// let player_1 = Player::new(controller.clone());
    /// let player_2 = Player::new(controller);
    ///
    /// let mut game = Game::new(player_1, player_2);
    ///
    /// // Panics because the controller does not have enough moves to play a game.
    /// game.play_default();
    /// ```
    pub fn play_default(&mut self) {
        self.play(&None);
    }

    /// Play the full game.
    ///
    /// The `reset_with_deck` parameter is for testing. If [`Some`], then instead of using
    /// [`Game::reset_deck`] and shuffling, it will just set [`Game::deck`] to the given
    /// [`Option<Deck>`].
    ///
    /// How the play works:
    /// * Each [`Player`] chooses a random [`Card`] from [`Deck`]. The highest value [`Card`] wins,
    ///   and [`Card`] suit order is [`Suit::Hearts`], [`Suit::Spades`], [`Suit::Diamonds`],
    ///   [`Suit::Clubs`]. The winner is the dealer who gets the crib.
    /// * The [`Deck`] is shuffled and each [`Player`] is dealt 6 [`Card`]s.
    /// * The [`Player`]s choose 2 [`Card`]s to discard. These [`Card`]s are put into a new
    ///   [`Hand`], and given to the dealer [`Player`] as their crib.
    /// * The top of the [`Deck`] is popped and stored as the starter [`Card`].
    /// * If this [`Card`] is a [`Rank::Jack`], the dealer gets two points.
    /// * Starting with the non-dealer (Pone) each [`Player`] puts a [`Card`] from their [`Hand`]
    ///   on the stack and the score is counted incrementally. All [`Player`]s must play as long as
    ///   the running score is not 31 or over. If one [`Player`] can't make a move, they pass (GO)
    ///   to the next [`Player`]. If both can't make a move, the running score is reset to zero, and
    ///   the last [`Player`] to put down a [`Card`] gets to put down another [`Card`]. This is
    ///   until all [`Card`]s are laid out.
    /// * Afterwards the [`Player`]s [`Hand`]s/cribs are scored, with the starter [`Card`], starting
    ///   with the Pone.
    /// * If neither [`Player`]s score is 121, then switch dealer and loop from dealing [`Card`]s
    ///   step.
    ///
    /// # Panics
    ///
    /// If there have been 1,000 rounds, indicating that the game is broken and can't end loop.
    pub fn play(&mut self, reset_with_deck: &Option<Deck>) {
        let mut round = 0;

        self.choose_dealer();

        loop {
            self.run_deal_and_discard_round();

            let starter = self.get_starter();

            if self.player_has_won() {
                break;
            }

            self.run_play_round();

            if self.player_has_won() {
                break;
            }

            self.run_counting_round(&starter);

            if self.player_has_won() {
                break;
            }

            match reset_with_deck {
                Some(ref deck) => self.reset_deck_with(deck.clone()),
                None => self.reset_deck(starter),
            }

            self.swap_dealer_and_pone();

            round += 1;

            assert!(1_000 >= round, "Play got stuck at round 1000!");
        }
    }

    /// Chose dealer and pone.
    ///
    /// This is done by having each [`Player`] choose a [`Card`] from the [`Deck`]
    /// and the dealer is the highest value [`Card`].
    /// * The highest value [`Card`] wins.
    /// * Card suit order is [`Suit::Hearts`], [`Suit::Spades`], [`Suit::Diamonds`],
    ///   [`Suit::Clubs`].
    ///
    /// # Panics
    ///
    /// If the [`Player::controller`] returns an index that is out of bounds of the [`Deck`].
    fn choose_dealer(&mut self) {
        let mut temp_deck = self.deck.clone();

        let dealer_chosen_card = self.dealer.choose_card_for_cut(&mut temp_deck).unwrap();
        let pone_chosen_card = self.pone.choose_card_for_cut(&mut temp_deck).unwrap();

        let dealer_won = dealer_chosen_card > pone_chosen_card;

        self.display.println(self.display.game_after_cut(
            &dealer_chosen_card,
            &pone_chosen_card,
            dealer_won,
        ));

        // Maybe do a memswap instead
        if !dealer_won {
            let temp_player = self.dealer.clone();

            self.dealer = self.pone.clone();
            self.pone = temp_player;
        }
    }

    /// Indicates that the game is won by [`Deck::dealer`] or [`Deck::pone`].
    ///
    /// If either [`Player`] has at least 121 points, the game is won for them.
    fn player_has_won(&self) -> bool {
        (121 <= self.dealer.points) || (121 <= self.pone.points)
    }

    /// This method facilitates the [`Player`]s discarding for cribs.
    ///
    /// Each [`Player`] is dealt 6 [`Card`]s. Then [`Player`]s choose 2 [`Card`]s to discard.
    /// These [`Card`]s are put into a new [`Hand`], and given to the dealer [`Player`] as
    /// their crib. The dealer is dealt first even though that is wrong.
    ///
    /// # Panics
    ///
    /// * If there are not enough [`Card`]s in the [`Deck`] to deal 12 [`Card`]s.
    /// * If either [`Player::controller`] chooses a discard out of bounds of their [`Hand`]s.
    ///
    fn run_deal_and_discard_round(&mut self) {
        for _ in 0..6 {
            match (self.deck.deal(), self.deck.deal()) {
                (Some(card_1), Some(card_2)) => {
                    self.dealer.add_card(card_1);
                    self.pone.add_card(card_2);
                }
                _ => panic!("There are not enough cards to deal!"),
            }
        }

        let mut discards = vec![];

        for _ in 0..2 {
            self.display
                .println(self.display.game_before_play_to_string(
                    /*starter=*/ None,
                    &self.dealer,
                    &self.pone,
                ));

            discards.push(
                self.pone
                    .remove_card()
                    .expect("Pone Controller has no moves for first discard!"),
            );
            discards.push(
                self.dealer
                    .remove_card()
                    .expect("Dealer Controller has no moves for first discard!"),
            );
        }

        self.display
            .println(self.display.game_before_play_to_string(
                /*starter=*/ None,
                &self.dealer,
                &self.pone,
            ));

        let crib = Hand::from(discards);

        self.dealer.crib = crib;
    }

    /// Return starter [`Card`], which is the [`Card`] at the top of the [`Deck`].
    ///
    /// If the starter is a [`Rank::Jack`], give 2 points to the dealer.
    ///
    /// # Panics
    ///
    /// If [`Deck`] is empty.
    fn get_starter(&mut self) -> Card {
        let starter = self
            .deck
            .deal()
            .expect("Could not get starter from empty deck!");

        if starter.rank == Rank::Jack {
            self.dealer.points += 2;
        }

        starter
    }

    /// This method facilitates the play round.
    ///
    /// Starting with the non-dealer (Pone) each [`Player`] puts a [`Card`] from his [`Hand`]
    /// on the stack and the score is counted incrementally. All [`Player`]s must play as long as
    /// the running score is not 31 or over. If one [`Player`] can't make a move, they pass (GO) to
    /// the next [`Player`]. If both can't make a move, the running score is reset to zero, and the
    /// last [`Player`] to put down a [`Card`] gets to put down another [`Card`]. This is until all
    /// [`Card`]s are laid out
    ///
    /// # Panics
    ///
    /// * If something goes wrong with counting turns or if this method exceeded 100 turns.
    /// * If either [`Player::controller`] chooses a discard out of bounds of their [`Hand`]s.
    fn run_play_round(&mut self) {
        let mut turn: usize = 0;
        let mut play_data = PlayData::new();

        while self.dealer.has_cards_in_hand() || self.pone.has_cards_in_hand() {
            match turn % 2 {
                0 => play_data.play_once(&mut self.pone, &self.dealer),
                1 => play_data.play_once(&mut self.dealer, &self.pone),
                _ => panic!("Something went wrong with alternating turns: {}", turn),
            }

            if (121 <= self.dealer.points) || (121 <= self.pone.points) {
                break;
            }

            let reset = play_data.reset_if_needed(&self.dealer, &self.pone);

            if !reset {
                turn += 1;
            }

            // Panic if too many turns has taken place.
            assert!(
                100 >= turn,
                "Too many turns!\nTurn: {}\nPlayData: {:?}\nDealer: {:?}\nPone: {:?}",
                turn,
                play_data,
                self.dealer,
                self.pone
            );
        }

        self.dealer.gather_discarded();
        self.pone.gather_discarded();
    }

    /// This method facilitates the scoring round.
    ///
    /// The [`Player`]s [`Hand`]s/cribs are scored, with the starter [`Card`], starting with the
    /// Pone.
    fn run_counting_round(&mut self, starter: &Card) {
        self.pone.points += self.pone.hand.total(starter, /*is_crib=*/ false);

        if 121 <= self.pone.points {
            return;
        }

        self.dealer.points += self.dealer.hand.total(starter, /*is_crib=*/ false);
        self.dealer.points += self.dealer.crib.total(starter, /*is_crib=*/ true);
    }

    /// Resets the [`Deck`].
    ///
    /// This will drain all the [`Card`]s from the dealer's and pone's [`Hand`] and
    /// [`Player::crib`]. In addition to adding back in the starter [`Card`].
    ///
    /// Theoretically, this should be fine since all the [`Card`]s that the [`Player`]s have
    /// came from the [`Deck`]. Same goes for the starter.
    fn reset_deck(&mut self, starter: Card) {
        let mut remaining_deck_cards = self.deck.as_vec().clone();

        remaining_deck_cards.append(&mut self.dealer.remove_all());

        remaining_deck_cards.append(&mut self.pone.remove_all());

        remaining_deck_cards.push(starter);

        self.deck = Deck::new_with_cards(remaining_deck_cards);
    }

    /// Resets the [`Game::deck`] with a given [`Deck`].
    ///
    /// This will drain all the [`Card`]s from the dealer's and pone's [`Hand`] and
    /// [`Player::crib`].
    fn reset_deck_with(&mut self, deck: Deck) {
        self.deck = deck;

        self.dealer.reset();
        self.pone.reset();
    }

    /// Swaps [`Deck::dealer`] and [`Deck::pone`]
    fn swap_dealer_and_pone(&mut self) {
        let temp = self.dealer.clone();

        self.dealer = self.pone.clone();
        self.pone = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cards::{Card, Deck, Rank, Suit};
    use crate::game::{Player, PredeterminedController};

    #[test]
    fn test_game_choose_dealer_player_1_wins_higher_value() {
        // Chose King of Diamonds
        let player_1_controller = PredeterminedController::from(vec![1, 32]);
        let player_1 = Player::new(player_1_controller);

        // Chose Eight of Clubs
        let player_2_controller = PredeterminedController::from(vec![2, 69]);
        let player_2 = Player::new(player_2_controller);

        let deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1.clone(), player_2.clone(), deck.clone());

        let expected_player_1_controller = PredeterminedController::from(vec![32]);
        let expected_player_1 = Player::new(expected_player_1_controller);

        let expected_player_2_controller = PredeterminedController::from(vec![69]);
        let expected_player_2 = Player::new(expected_player_2_controller);

        game.choose_dealer();

        assert_eq!(game.deck, deck);
        assert_eq!(game.dealer, expected_player_1);
        assert_eq!(game.pone, expected_player_2);
    }

    #[test]
    fn test_game_choose_dealer_player_1_wins_same_value_higher_suit() {
        // Chose Eight of Clubs
        let player_1_controller = PredeterminedController::from(vec![3, 32]);
        let player_1 = Player::new(player_1_controller);

        // Chose Eight of Diamonds
        let player_2_controller = PredeterminedController::from(vec![0, 69]);
        let player_2 = Player::new(player_2_controller);

        let deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1.clone(), player_2.clone(), deck.clone());

        let expected_player_1_controller = PredeterminedController::from(vec![32]);
        let expected_player_1 = Player::new(expected_player_1_controller);

        let expected_player_2_controller = PredeterminedController::from(vec![69]);
        let expected_player_2 = Player::new(expected_player_2_controller);

        game.choose_dealer();

        assert_eq!(game.deck, deck);
        assert_eq!(game.dealer, expected_player_1);
        assert_eq!(game.pone, expected_player_2);
    }

    #[test]
    fn test_game_choose_dealer_player_2_wins_higher_value() {
        // Chose Eight of Diamonds
        let player_1_controller = PredeterminedController::from(vec![0, 32]);
        let player_1 = Player::new(player_1_controller);

        // Chose King of Diamonds
        let player_2_controller = PredeterminedController::from(vec![0, 69]);
        let player_2 = Player::new(player_2_controller);

        let deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1.clone(), player_2.clone(), deck.clone());

        let expected_player_1_controller = PredeterminedController::from(vec![32]);
        let expected_player_1 = Player::new(expected_player_1_controller);

        let expected_player_2_controller = PredeterminedController::from(vec![69]);
        let expected_player_2 = Player::new(expected_player_2_controller);

        game.choose_dealer();

        assert_eq!(game.deck, deck);
        assert_eq!(game.dealer, expected_player_2);
        assert_eq!(game.pone, expected_player_1);
    }

    #[test]
    fn test_game_choose_dealer_player_2_wins_same_value_higher_suit() {
        // Chose Eight of Diamonds
        let player_1_controller = PredeterminedController::from(vec![0, 32]);
        let player_1 = Player::new(player_1_controller);

        // Chose Eight of Clubs
        let player_2_controller = PredeterminedController::from(vec![2, 69]);
        let player_2 = Player::new(player_2_controller);

        let deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1.clone(), player_2.clone(), deck.clone());

        let expected_player_1_controller = PredeterminedController::from(vec![32]);
        let expected_player_1 = Player::new(expected_player_1_controller);

        let expected_player_2_controller = PredeterminedController::from(vec![69]);
        let expected_player_2 = Player::new(expected_player_2_controller);

        game.choose_dealer();

        assert_eq!(game.deck, deck);
        assert_eq!(game.dealer, expected_player_2);
        assert_eq!(game.pone, expected_player_1);
    }

    #[test]
    fn test_game_run_deal_and_discard_round() {
        // Discard Six of Hearts and Eight of Clubs to crib
        let player_1_controller = PredeterminedController::from(vec![0, 3, 32]);
        let player_1 = Player::new(player_1_controller);

        // Discard Five of Clubs and Six of Clubs to crib
        let player_2_controller = PredeterminedController::from(vec![2, 3, 69]);
        let player_2 = Player::new(player_2_controller);

        // Deck is dealt in reverse!
        let deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Hearts),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1.clone(), player_2.clone(), deck.clone());

        let expected_player_1_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let expected_player_1_crib = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let expected_player_1_controller = PredeterminedController::from(vec![32]);
        let expected_player_1 = Player::new_with_cards_and_crib(
            expected_player_1_controller,
            expected_player_1_cards,
            expected_player_1_crib,
        );

        let expected_player_2_cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
        ];
        let expected_player_2_controller = PredeterminedController::from(vec![69]);
        let expected_player_2 =
            Player::new_with_cards(expected_player_2_controller, expected_player_2_cards);

        game.run_deal_and_discard_round();

        assert_eq!(game.deck, Deck::new_with_cards(Vec::new()));
        assert_eq!(game.dealer, expected_player_1);
        assert_eq!(game.pone, expected_player_2);
    }

    #[test]
    fn test_game_get_starter_not_jack() {
        // Discard Six of Hearts and Eight of Clubs to crib
        let player_1_controller = PredeterminedController::from(vec![0, 3, 32]);
        let player_1 = Player::new(player_1_controller);

        // Discard Five of Clubs and Six of Clubs to crib
        let player_2_controller = PredeterminedController::from(vec![2, 3, 69]);
        let player_2 = Player::new(player_2_controller);

        // Deck is dealt in reverse!
        let deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Hearts),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1.clone(), player_2.clone(), deck.clone());

        let expected_deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
        ];
        let expected_dealer_points = 0;
        let expected_pone_points = 0;

        let starter = game.get_starter();

        assert_eq!(starter, Card::new(Rank::Six, Suit::Hearts));
        assert_eq!(game.deck.as_vec(), &expected_deck_cards);
        assert_eq!(game.dealer.points, expected_dealer_points);
        assert_eq!(game.pone.points, expected_pone_points);
    }

    #[test]
    fn test_game_get_starter_jack() {
        // Discard Six of Hearts and Eight of Clubs to crib
        let player_1_controller = PredeterminedController::from(vec![0, 3, 32]);
        let player_1 = Player::new(player_1_controller);

        // Discard Five of Clubs and Six of Clubs to crib
        let player_2_controller = PredeterminedController::from(vec![2, 3, 69]);
        let player_2 = Player::new(player_2_controller);

        // Deck is dealt in reverse!
        let deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Diamonds),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1.clone(), player_2.clone(), deck.clone());

        let expected_deck_cards = vec![
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Hearts),
        ];
        let expected_dealer_points = 2;
        let expected_pone_points = 0;

        let starter = game.get_starter();

        assert_eq!(starter, Card::new(Rank::Jack, Suit::Diamonds));
        assert_eq!(game.deck.as_vec(), &expected_deck_cards);
        assert_eq!(game.dealer.points, expected_dealer_points);
        assert_eq!(game.pone.points, expected_pone_points);
    }

    #[test]
    fn test_game_run_play_round() {
        // Play stack (start with p2)
        //     * Stack 1 -> 7D(p2, 0pt, 7), 7C(p1, 2pt, 14), 8D(p2, 0pt, 22), 6D(p2, 3pt, 28),
        //                  GO(p2, 1pt, 28)
        //     * Stack 2 -> 4C(p2, 0pt, 4), JD(p1, 0pt, 14), QD(p1, 0pt, 24), GO(p1, 1pt, 24)
        //     * Stack 3 -> KD(p1, 0pt, 10), GO (p1, 1pt, 10)
        //
        // Score at end: p1 = 4 (pair and 2 GOs), p2 = 4 (run of 3 and a GO)

        // Discard: 7C, JD, QD, KD
        let player_1_controller = PredeterminedController::from(vec![1, 0, 0, 0, 32]);
        let player_1_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let player_1 = Player::new_with_cards(player_1_controller, player_1_cards);

        // Discard: 7D, 8D, 6D, 4C
        let player_2_controller = PredeterminedController::from(vec![2, 2, 1, 0, 69]);
        let player_2_cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
        ];
        let player_2 = Player::new_with_cards(player_2_controller, player_2_cards);

        let mut game = Game::new(player_1, player_2);

        let expected_dealer_points = 4;
        let expected_pone_points = 4;

        game.run_play_round();

        assert_eq!(game.dealer.points, expected_dealer_points);
        assert_eq!(game.pone.points, expected_pone_points);

        // assert that the [`Hand`]s were reset
        assert_eq!(game.dealer.hand.as_vec().len(), 4);
        assert_eq!(game.pone.hand.as_vec().len(), 4);
        assert!(game.dealer.discarded.is_empty());
        assert!(game.pone.discarded.is_empty());
    }

    #[test]
    fn test_game_run_play_round_player_1_hit_121_before_first_reset() {
        // Play stack (start with p2) p1.points = 118 && p2.points = 120
        //     * Stack 1 -> 7D(p2, 0pt, 7), 7C(p1, 2pt, 14), 8D(p2, 0pt, 22), 6D(p2, 3pt, 28),
        //                  GO(p2, 1pt, 28)
        //     * p1 hit 121 break
        //
        // Score at end: p1 = 120 (pair), p2 = 124 (run of 3 and a GO)

        // Discard: 7C, JD, QD, KD
        let player_1_controller = PredeterminedController::from(vec![1, 0, 0, 0, 32]);
        let player_1_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let mut player_1 = Player::new_with_cards(player_1_controller, player_1_cards);
        player_1.points = 118;

        // Discard: 7D, 8D, 6D, 4C
        let player_2_controller = PredeterminedController::from(vec![2, 2, 1, 0, 69]);
        let player_2_cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
        ];
        let mut player_2 = Player::new_with_cards(player_2_controller, player_2_cards);
        player_2.points = 120;

        let mut game = Game::new(player_1, player_2);

        let expected_dealer_points = 120;
        let expected_pone_points = 124;

        game.run_play_round();

        assert_eq!(game.dealer.points, expected_dealer_points);
        assert_eq!(game.pone.points, expected_pone_points);

        // assert that the hands were reset
        assert_eq!(game.dealer.hand.as_vec().len(), 4);
        assert_eq!(game.pone.hand.as_vec().len(), 4);
        assert!(game.dealer.discarded.is_empty());
        assert!(game.pone.discarded.is_empty());
    }

    #[test]
    fn test_game_run_counting_round() {
        let controller = PredeterminedController::from(Vec::new());

        let starter = Card::new(Rank::Eight, Suit::Diamonds);

        // Hand Score 6pts: 15 2pts, 3-run 3pts, Nobs 1pt
        // Crib Score 13pts: 15 4pts, 4-run 4pts, 5-flush 5pts
        // Total Score 19pts
        let player_1_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let player_1_crib = vec![
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
        ];
        let player_1 =
            Player::new_with_cards_and_crib(controller.clone(), player_1_cards, player_1_crib);

        // Hand Score 12pts: 15 4pts, Pair 2pts, 2x 3-run 6pts
        let player_2_cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let player_2 = Player::new_with_cards(controller, player_2_cards);

        let mut game = Game::new(player_1, player_2);

        let expected_dealer_points = 19;
        let expected_pone_points = 12;

        game.run_counting_round(&starter);

        assert_eq!(game.dealer.points, expected_dealer_points);
        assert_eq!(game.pone.points, expected_pone_points);
    }

    #[test]
    fn test_game_run_counting_round_player_2_hit_121_before_player_1_can_count() {
        let controller = PredeterminedController::from(Vec::new());

        let starter = Card::new(Rank::Eight, Suit::Diamonds);

        // Hand Score 6pts: 15 2pts, 3-run 3pts, Nobs 1pt
        // Crib Score 13pts: 15 4pts, 4-run 4pts, 5-flush 5pts
        // Total Score 19pts
        let player_1_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let player_1_crib = vec![
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
        ];
        let player_1 =
            Player::new_with_cards_and_crib(controller.clone(), player_1_cards, player_1_crib);

        // Hand Score 12pts: 15 4pts, Pair 2pts, 2x 3-run 6pts
        let player_2_cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let mut player_2 = Player::new_with_cards(controller, player_2_cards);
        player_2.points = 110;

        let mut game = Game::new(player_1, player_2);

        let expected_dealer_points = 0;
        let expected_pone_points = 122;

        game.run_counting_round(&starter);

        assert_eq!(game.dealer.points, expected_dealer_points);
        assert_eq!(game.pone.points, expected_pone_points);
    }

    #[test]
    fn test_game_reset_deck() {
        let controller = PredeterminedController::from(Vec::new());

        let starter = Card::new(Rank::Eight, Suit::Diamonds);

        let player_1_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let player_1_crib = vec![
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
        ];
        let player_1 =
            Player::new_with_cards_and_crib(controller.clone(), player_1_cards, player_1_crib);

        let player_2_cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let player_2 = Player::new_with_cards(controller, player_2_cards);

        let deck = Deck::new_with_cards(Vec::new());
        let mut game = Game::new_with_deck(player_1, player_2, deck);

        let expected_deck_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Diamonds),
        ];
        let expected_deck = Deck::new_with_cards(expected_deck_cards);

        game.reset_deck(starter);

        assert_eq!(game.deck, expected_deck);
    }

    #[test]
    fn test_game_reset_deck_with() {
        let controller = PredeterminedController::from(Vec::new());

        let player_1_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let player_1_crib = vec![
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
        ];
        let player_1 =
            Player::new_with_cards_and_crib(controller.clone(), player_1_cards, player_1_crib);

        let player_2_cards = vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Clubs),
        ];
        let player_2 = Player::new_with_cards(controller, player_2_cards);

        let deck = Deck::new_with_cards(Vec::new());
        let mut game = Game::new_with_deck(player_1, player_2, deck);

        let expected_deck_cards = vec![
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Diamonds),
        ];
        let expected_deck = Deck::new_with_cards(expected_deck_cards);

        game.reset_deck_with(expected_deck.clone());

        assert_eq!(game.deck, expected_deck);
        assert!(!game.dealer.has_cards());
        assert!(!game.pone.has_cards());
    }

    #[test]
    fn test_game_play() {
        // The maximum number of points that can be scored in a single round by the dealer is 78.
        //     * Pegging: 29pts
        //     * Hand: 20pts
        //     * Crib: 29pts
        //
        // This is achieved by dealing the following
        //     * Pone: 3H, 3S, 4H, 4S, 5H, JC
        //     * Dealer: 3D, 3C, 4D, 4C, 5S, 5D
        //
        // So the state after dealing, discarding, and getting the starter:
        //     * Pone Hand: 3H, 3S, 4H, 4S
        //         * Discarded/Removed: JC, 5H
        //     * Dealer Hand: 3D, 3C, 4D, 4C
        //         * Discarded/Removed: 5D, 5S
        //         * Indices: 5, 4
        //     * Dealer Crib: JC, 5D, 5H, 5S
        //     * Starter Card: 5C
        //         * Note: The Pone's Jack matches suit of the starter 5 (Clubs).
        //     * So deck has to atleast be (in exact order):
        //         * 5C, JC, 5D, 5H, 5S, 4S, 4C, 4H, 4D, 3S, 3C, 3H, 3D
        //     * Both players removed indices: 5, 4
        //
        // Then the Pegging (for the 29 dealer score) would be:
        //     * 3H(P,0), 3D(D,2), 3S(P,6), 3C(D,12), 4H(P,0), 4D(D,2), 4S(P,6), 4C(D,12),GO(D,1)
        //     * (P,110), (D,112), (P,116), (D,124*) if P and D start w/ 110pts
        //         * D = Dealer played.
        //         * P = Pone played.
        //         * # = Points earned.
        //     * Dealer Scores: 29pts
        //     * Pone Scores: 12pts
        //     * Both players play indices: 0, 0, 0, 0
        //
        // Counting scores:
        //     * Pone Hand: 3H, 3S, 4H, 4S, 5C (Starter)
        //         * Score w/ Starter: 20pts
        //             * 2x15s (4pts) + 2xPairs (4pts) + 4xrun-of-3 (12pts)
        //     * Dealer Hand: 3D, 3C, 4D, 4C, 5C (Starter)
        //         * Score w/ Starter: 20pts
        //             * 2x15s (4pts) + 2xPairs (4pts) + 4xrun-of-3 (12pts)
        //     * Dealer Crib: JC, 5D, 5H, 5S, 5C (Starter)
        //         * Score w/ Starter: 29pts
        //             * 8x15s (16pts) + 6xPairs (12pts) + Nobs (1pt)
        //
        // Total Points for Players:
        //     * Pone: 32pts
        //         * Peggings (12pts) + Hand (20pts)
        //     * Dealer: 78pts
        //         * Peggings (29pts) + Hand (20pts) + Crib (29pts)
        //
        // If deck is doesn't change between rounds, but dealers alternate:
        //     * P1 chooses JC for cut, P2 chooses 5D for cut
        //         * P1 wins and is first dealer
        //     * Round 1 (P1 = Dealer, P2 = Pone):
        //         * P1: 78pts
        //         * P2: 32pts
        //         * Both players chose indices for discarding and pegging: 5,4,0,0,0,0
        //     * Round 2 (P1 = Pone, P2 = Dealer):
        //         * P1: 110pts
        //         * P2: 110pts
        //         * Both players chose indices for discarding and pegging: 5,4,0,0,0,0
        //     * Round 3 (P1 = Dealer, P2 = Pone):
        //         * P1: 124pts
        //         * P2: 116pts
        //         * Both players chose indices for discarding and pegging: 5,4,0,0
        //         * Game ends when dealers plays their 3C during pegging.
        //     * For all rounds both players chose the following indices for discarding and pegging:
        //         * 5,4,0,0,0,0,5,4,0,0,0,0,5,4,0,0
        let controller =
            PredeterminedController::from(vec![2, 5, 4, 0, 0, 0, 0, 5, 4, 0, 0, 0, 0, 5, 4, 0, 0]);

        let player_1 = Player::new(controller.clone());
        let player_2 = Player::new(controller);

        let deck_cards = vec![
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
        ];
        let deck = Deck::new_with_cards(deck_cards);

        let mut game = Game::new_with_deck(player_1, player_2, deck.clone());

        let expected_dealer_points = 124;
        let expected_pone_points = 116;

        game.play(&Some(deck));

        assert_eq!(game.dealer.points, expected_dealer_points);
        assert_eq!(game.pone.points, expected_pone_points);
    }
}
