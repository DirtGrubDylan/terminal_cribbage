extern crate libterminal_cribbage;


use libterminal_cribbage::cards::{Card, Suit, Rank};


fn main() {
    let c = Card { rank: Rank::Ace, suit: Suit::Clubs };

    println!("C: {}", c);
}
