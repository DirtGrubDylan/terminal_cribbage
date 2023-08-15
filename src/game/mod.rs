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

use crate::cards::Deck;

/// The actual game!
pub struct Game<C>
where
    C: Controller,
    C: Clone,
{
    dealer: Player<C>,
    pone: Player<C>,
    deck: Deck,
    display: Display,
}

impl<C> Game<C>
where
    C: Controller,
    C: Clone,
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
        //         * if either player has 121 -> break game
        //
        //     * players gather discarded back into hand
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
    fn run_discard_round(&mut self) {
        // Alternate deal two cards to each player
        // Let players each remove (discard) two cards
        // Consolidate the cards to a new hand.
        // Set dealer.crib to that new hand.
        
        unimplemented!()
    }

    /// This method facilitates the play round.
    ///
    /// Starting with the non-dealer (Pone) each player puts a card from his hand
    /// on the stack (maybe indicate which player put down the card?) And the score
    /// is counted incrementally. All players must play as long as the running score
    /// is not 31 or over. If one player can't make a move, they pass (GO) to the next payer.
    /// If both can't make a move, the running score is reset to zero, and the last player to
    /// put down a card gets to put down another card. This is until all cards are laid out
    fn run_play_round(&mut self) {
        // Set a turn variable to 0
        //
        // If either player has cards in their hand then play
        //     * if turn % 2 is 0 -> pone plays
        //     * if turn % 2 is 1 -> dealer plays
        //     * if either player score is 121 end game.
        //
        // use playdata struct
        unimplemented!()
    }

    /// This method facilitates the scoring round.
    ///
    /// The players hands/cribs are scored, with the starter card, starting with the Pone.
    fn run_counting_round(&mut self) {
        // Count pone hand first
        // Then count dealer hand and crib
        // if either player score is 121 end game

        unimplemented!()
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
        // Discard Six of Clubs and Five of Clubs to crib
        let player_1_controller = PredeterminedController::from(vec![1, 2, 32]);
        let player_1 = Player::new(player_1_controller);

        // Discard Eight of Clubs and Six of Hearts to crib
        let player_2_controller = PredeterminedController::from(vec![1, 4, 69]);
        let player_2 = Player::new(player_2_controller);

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
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
        ];
        let expected_player_1_crib = vec![
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Six, Suit::Hearts),
        ];
        let expected_player_1_controller = PredeterminedController::from(vec![32]);
        let expected_player_1 = Player::new_with_cards_and_crib(
            expected_player_1_controller,
            expected_player_1_cards,
            expected_player_1_crib,
        );

        let expected_player_2_cards = vec![
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Diamonds),
        ];
        let expected_player_2_controller = PredeterminedController::from(vec![69]);
        let expected_player_2 =
            Player::new_with_cards(expected_player_2_controller, expected_player_2_cards);
        game.run_discard_round();

        assert_eq!(game.deck, Deck::new_with_cards(Vec::new()));
        assert_eq!(game.dealer, expected_player_2);
        assert_eq!(game.pone, expected_player_1);
    }
}
