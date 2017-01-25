extern crate libterminal_cribbage;

use libterminal_cribbage::cards;

fn main() {
    let mut d = cards::Deck::new();

    println!("Deck unshuffled: {}", d);

    d.shuffle();

    println!("Deck unshuffled: {}", d);
}
