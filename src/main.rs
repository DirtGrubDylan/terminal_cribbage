extern crate libterminal_cribbage;

use std::io;

use libterminal_cribbage::cards::{Card, Deck, Rank, Suit};
use libterminal_cribbage::game::{Game, Player, PredeterminedController};

fn main() -> io::Result<()> {
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

    game.play(&Some(deck));

    Ok(())
}
