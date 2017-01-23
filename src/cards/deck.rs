use std::fmt;

use cards::card::{Card, Rank, Suit};


#[derive(Debug)]
pub struct Deck(pub Vec<Card>);


impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        let ranks: Vec<Rank> = vec![Rank::Ace,
                                    Rank::Two,
                                    Rank::Three,
                                    Rank::Four,
                                    Rank::Five,
                                    Rank::Six,
                                    Rank::Seven,
                                    Rank::Eight,
                                    Rank::Nine,
                                    Rank::Ten,
                                    Rank::Jack,
                                    Rank::Queen,
                                    Rank::King];
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Spades, Suit::Diamonds, Suit::Hearts];

        for suit in suits.into_iter() {
            for &rank in ranks.iter() {
                cards.push(Card { rank: rank, suit: suit });
            }
        }

        Deck(cards)
    }
}



impl fmt::Display for Deck {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for card in self.0.iter() {
            write!(formatter, "\n{}", card)?;
        }

        write!(formatter, "")
    }
}