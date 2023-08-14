extern crate libterminal_cribbage;

use libterminal_cribbage::cards::{Deck, Hand};
use libterminal_cribbage::game::{Player, PredeterminedController};

fn main() {
    let mut d = Deck::new();

    println!("Deck unshuffled: {d}");

    d.shuffle();

    println!("Deck unshuffled: {d}");

    let mut hand = Hand::new();

    for _ in 0..5 {
        if let Some(card) = d.deal() {
            hand.add_card(card);
        }
    }

    println!("Deck after deal: {d}");
    println!("Hand after deal: {hand}");

    let player_controller = PredeterminedController::from(vec![0, 0, 0]);

    let mut player = Player::new(player_controller);

    for _ in 0..5 {
        if let Some(card) = d.deal() {
            player.add_card(card);
        }
    }

    println!("Deck after second deal: {d}");
    println!("Hand after second deal: {hand}");
    println!("Player after second deal: {player}");

    let _ = player.discard();
    let _ = player.discard();

    println!("Player after two discards: {player}");

    let player_str = player.to_string();

    println!("Player to string: {player_str}");
}
