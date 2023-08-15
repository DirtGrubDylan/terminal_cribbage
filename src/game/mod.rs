//! This module holds the logic for the gameplay:
//! * Choose dealer
//! * Deal hands to players
//! * Discards from players for dealer crib
//! * Get starter card from deck
//! * Play (peg)
//! * Count hands
//! * Repeat until one player reaches 121pts

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

use crate::cards::{Card, Deck, Hand, Rank};

/// The actual game!
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
    pub fn new_with_deck(player_1: Player<C>, player_2: Player<C>, deck: Deck) -> Game<C> {
        Game {
            dealer: player_1,
            pone: player_2,
            deck,
            display: Display::new(),
        }
    }

    /// Play the full game.
    ///
    /// * Each player chooses a random card from deck. The highest value card wins,
    ///   and card suit order is hearts, spades, diamonds, clubs. The winner is the
    ///   dealer who gets the crib.
    /// * The deck is shuffled and each player is dealt 6 cards.
    /// * The players choose 2 cards to discard. These cards are put into a new hand,
    ///   and given to the dealer player as their crib.
    /// * The top of the deck is popped and stored as the starter card.
    /// * If this card is a jack, the dealer gets two points.
    /// * Starting with the non-dealer (Pone) each player puts a card from his hand
    ///   on the stack (maybe indicate which player put down the card?) And the score
    ///   is counted incrementally. All players must play as long as the running score
    ///   is not 31 or over. If one player can't make a move, they pass (GO) to the next payer.
    ///   If both can't make a move, the running score is reset to zero, and the last player to
    ///   put down a card gets to put down another card. This is until all cards are laid out
    /// * Afterwards the players hands/cribs are scored, with the starter card, starting with the Pone.
    /// * If neither players score is 121, then switch dealer and loop from dealing cards step.
    pub fn play(&mut self) {
        // make copies of players and deck for reset
        //
        // Choose dealer
        //
        // game: while dealer.points != 121 || player.points != 121
        //     * Flip starter
        //         * If jack -> add 2pts to dealer
        //
        //     * If dealer has 121 -> break game
        //
        //     * play: while either player has cards in hand
        //         * run play round
        //             * players gather discarded back into hand
        //
        //     * if either player has 121 -> break game
        //
        //     * Run counting
        //
        //     * pone counts hand
        //     * if pone has 121 -> break game;
        //     * deal counts hand and crib
        //     * if pone has 121 -> break game;
        //
        //     * reset deck
        //         * drain players hands and crib to deck
        //         * assert players hands/crib are empty
        //         * assert deck is same size as last

        unimplemented!()
    }

    /// Chose dealer and pone.
    ///
    /// This is done by having each player choose a [`Card`] from the [`Deck`]
    /// and the dealer is the highest value [`Card`].
    /// * The highest value card wins.
    /// * Card suit order is Hearts, Spades, Diamonds, Clubs.
    ///
    /// # Panics
    ///
    /// If the [`Player::controller`] returns an index that is out of bounds of the
    /// [`Deck`].
    fn choose_dealer(&mut self) {
        let mut temp_deck = self.deck.clone();

        let dealer_chosen_card = self.dealer.choose_card_for_cut(&mut temp_deck).unwrap();
        let pone_chosen_card = self.pone.choose_card_for_cut(&mut temp_deck).unwrap();

        // Maybe do a memswap instead
        if dealer_chosen_card < pone_chosen_card {
            let temp_player = self.dealer.clone();

            self.dealer = self.pone.clone();
            self.pone = temp_player;
        }
    }

    /// This method facilitates the [`Player`]s discarding for cribs.
    ///
    /// Each [`Player`] is dealt 6 [`Card`]s. Then [`Player`]s choose 2 [`Card`]s to discard.
    /// These [`Card`]s are put into a new [`Hand`], and given to the dealer [`Player`] as
    /// their crib.
    ///
    /// # Panics
    ///
    /// * If there are not enough [`Card`]s in the deck to deal 12 [`Card`]s.
    /// * If either [`Player::controller`] chooses a discard out of bounds of their [`Hand`]s.
    ///
    fn run_discard_round(&mut self) {
        for _ in 0..6 {
            match (self.deck.deal(), self.deck.deal()) {
                (Some(card_1), Some(card_2)) => {
                    self.dealer.add_card(card_1);
                    self.pone.add_card(card_2);
                }
                _ => panic!("There are not enough cards to deal!"),
            }
        }

        let discards = vec![
            self.dealer.remove_card().unwrap(),
            self.dealer.remove_card().unwrap(),
            self.pone.remove_card().unwrap(),
            self.pone.remove_card().unwrap(),
        ];

        let crib = Hand::from(discards);

        self.dealer.crib = crib;
    }

    /// Return starter [`Card`], which is the card at the top of the [`Deck`].
    ///
    /// If the starter is a [`Rank::Jack`], give 2 points to the dealer.
    ///
    /// # Panics
    ///
    /// If [`Deck`] is empty.
    fn get_starter(&mut self) -> Card {
        let starter = self.deck.deal().unwrap();

        if starter.rank == Rank::Jack {
            self.dealer.points += 2;
        }

        starter
    }

    /// This method facilitates the play round.
    ///
    /// Starting with the non-dealer (Pone) each player puts a card from his hand
    /// on the stack (maybe indicate which player put down the card?) And the score
    /// is counted incrementally. All players must play as long as the running score
    /// is not 31 or over. If one player can't make a move, they pass (GO) to the next payer.
    /// If both can't make a move, the running score is reset to zero, and the last player to
    /// put down a card gets to put down another card. This is until all cards are laid out
    ///
    /// # Panics
    ///
    /// If something goes wrong with counting turns or if this method exceeded 100 turns.
    fn run_play_round(&mut self) {
        let mut turn: usize = 0;
        let mut play_data = PlayData::new();

        while self.dealer.has_cards() || self.pone.has_cards() {
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
            if 100 <= turn {
                panic!(
                    "Too many turns!\nTurn: {}\nPlayData: {:?}\nDealer: {:?}\nPone: {:?}",
                    turn, play_data, self.dealer, self.pone
                );
            }
        }

        self.dealer.gather_discarded();
        self.pone.gather_discarded();
    }

    /// This method facilitates the scoring round.
    ///
    /// The players hands/cribs are scored, with the starter card, starting with the Pone.
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
    /// This will drain all the [`Card`]s from the dealer's and pone's [`Hand`] and [`Crib`].
    /// In addition to adding back in the starter card.
    ///
    /// Theoretically, this should be fine since all the [`Card`]s that the [`Player`]s have
    /// came from the deck. Same goes for the starter.
    fn reset_deck(&mut self, starter: Card) {
        let mut remaining_deck_cards = self.deck.as_vec().clone();

        remaining_deck_cards.append(&mut self.dealer.remove_all());

        remaining_deck_cards.append(&mut self.pone.remove_all());

        remaining_deck_cards.push(starter);

        self.deck = Deck::new_with_cards(remaining_deck_cards);
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
    fn test_game_run_discard_round() {
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
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
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

        game.run_discard_round();

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

        // assert that the hands were reset
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
}
