extern crate libterminal_cribbage;

use std::io::{self, Write};

use libterminal_cribbage::cards::{Deck, Hand};
use libterminal_cribbage::game::{Player, PredeterminedController};

fn main() -> io::Result<()> {
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

    println!("********************************************************");

    let mut turn = 0;
    let mut input = String::new();

    while input != "6" {
        input.clear();

        print!("Please type something: ");

        io::stdout().flush()?;

        input = if turn % 2 == 0 {
            get_something()
        } else {
            get_something_auto()
        };

        if turn % 2 == 1 {
            println!("{input}");
        }

        turn += 1;
    }

    Ok(())
}

fn get_something_auto() -> String {
    "1".to_string()
}

fn get_something() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    input.trim().to_string()
}
