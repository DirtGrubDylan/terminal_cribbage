//! Handles the display of the game.

/// A struct for just displaying the game.
///
/// Not sure how this will work yet.
pub struct Display {}

impl Display {
    /// Creates a new [`Display`] struct.
    pub fn new() -> Display {
        Display {}
    }
}

impl Default for Display {
    fn default() -> Self {
        Display::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        unimplemented!()
    }
}
