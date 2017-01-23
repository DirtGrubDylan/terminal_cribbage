use std::fmt;

#[derive(Debug)]
pub struct Card {
    value: u8,
    face: char,
    suit: char,
}


impl Card {
    pub fn new(face: &str, suit: char) -> Card {
        let value = match face {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "J" => 10,
            "Q" => 10,
            "K" => 10,
            "A" => 1,
            _ => 0,
        }
    }
}


impl fmt::Display for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[{}{}]", self.face, self.suit)?;

        Ok(())
    }
}