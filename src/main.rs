extern crate libterminal_cribbage;


use libterminal_cribbage::cards::Deck;


fn main() {
    let d = Deck::new();

    println!("D: {}", d);
}
