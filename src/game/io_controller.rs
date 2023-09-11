use std::io::{self, Stdin};

use cards::Card;
use game::{Controller, Display, UiDisplay};

/// A controller that gets all of it's moves from stdin.
#[derive(Debug)]
pub struct IoController {
    display: UiDisplay,
    stdin: Stdin,
}

impl IoController {
    /// Creates a new [`IoController`].
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::game::IoController;
    ///
    /// let controller = IoController::new();
    /// ```
    #[must_use]
    pub fn new() -> IoController {
        IoController {
            display: UiDisplay::new(),
            stdin: io::stdin(),
        }
    }

    /// Gets an index less than the given bound from the user via stdin.
    ///
    /// The index is chosen by prompting the user to choose a card index from the available cards.
    /// To make it easy for non-SWEs, the selection from the user is 1-based, but is translated to
    /// 0-based when returning.
    ///
    /// # Panics
    ///
    /// * If stdout buffer could not be flushed.
    /// * If the user input from stdin could not be read.
    fn get_index_from_user(&self, upper_bound: usize) -> Result<usize, String> {
        let mut input = String::new();

        self.display
            .flush_stdout()
            .expect("Could not flush the buffer!");

        self.stdin
            .read_line(&mut input)
            .expect("Error reading from stdin!");

        input = input.trim().to_string();

        match input.parse::<usize>() {
            Ok(index) if 0 < index && index <= upper_bound => Ok(index - 1),
            Ok(oob_index) => Err(format!(
                "{oob_index} is out of bounds. Please choose a number between 1 and {upper_bound}!"
            )),
            Err(_) => Err(format!("{input} is not a number!")),
        }
    }
}

impl Controller for IoController {
    /// Returns a possible index for a [`Card`] for a given array of [`Card`]s.
    ///
    /// The index is chosen by prompting the user to choose a card index from the available cards.
    /// To make it easy for non-SWEs, the selection from the user is 1-based, but is translated to
    /// 0-based when returning.
    ///
    /// # Examples
    ///
    /// ```
    /// use libterminal_cribbage::cards::{Card, Rank, Suit};
    /// use libterminal_cribbage::game::{Controller, IoController};
    ///
    /// let no_cards = vec![];
    /// let available_cards = vec![
    ///     Card::new(Rank::Queen, Suit::Hearts),
    ///     Card::new(Rank::King, Suit::Clubs),
    /// ];
    ///
    /// let mut controller = IoController::new();
    ///
    /// assert!(controller.get_card_index(&no_cards).is_none());
    /// assert!(controller.get_card_index(&available_cards).is_some());
    /// ```
    #[must_use]
    fn get_card_index(&mut self, available_cards: &[Card]) -> Option<usize> {
        let mut result = None;

        let number_of_cards = available_cards.len();

        let prompt_message = format!("Choose Card to Discard (1 to {number_of_cards}): ");

        // Keep looping to get all
        while !available_cards.is_empty() && result.is_none() {
            self.display.print_no_spacer_no_delay(&prompt_message);

            match self.get_index_from_user(number_of_cards) {
                Ok(index) => result = Some(index),
                Err(err) => self.display.println_no_spacer_no_delay(&err),
            }
        }

        result
    }
}

impl Default for IoController {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for IoController {
    fn clone(&self) -> Self {
        IoController {
            display: self.display.clone(),
            stdin: io::stdin(),
        }
    }
}
