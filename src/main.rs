extern crate libterminal_cribbage;


use libterminal_cribbage::cards::Deck;


fn main() {
    let mut d = Deck::new();

    for _ in 0..100000 {
        d = Deck::new();
    }
}
