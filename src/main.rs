extern crate libterminal_cribbage;

use libterminal_cribbage::cards::{Card, Deck, Rank, Suit};
use libterminal_cribbage::game::{Game, Player, PredeterminedController, RngController, UiDisplay};

fn main() {
    let _player_1_controller = PredeterminedController::new(
        vec![2, 5, 4, 0, 0, 0, 0, 5, 4, 0, 0, 0, 0, 5, 4, 0, 0],
        UiDisplay::new(),
    );
    let _player_2_controller =
        PredeterminedController::from(vec![2, 5, 4, 0, 0, 0, 0, 5, 4, 0, 0, 0, 0, 5, 4, 0, 0]);

    let player_1 = Player::new(RngController::new());
    let player_2 = Player::new(RngController::new());

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

    let mut game = Game::new_with_deck_default(player_1, player_2, deck.clone(), UiDisplay::new());

    game.play(&Some(deck));
}
