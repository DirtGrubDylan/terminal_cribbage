//! # Terminal Cribbage
//!
//! Termial Cribbage is a simple Cribbage game that can be played on the terminal of any platform
//! (e.g. Windows Command Line, Unix Terminal, etc...). For those new to the game, [here are the
//! rules].
//!
//! As of right now, Terminal Cribbage can only be played with two players of three differnt
//! variations: One Human vs. One Human, One Human vs. One AI, or One AI vs. One AI.
//!
//! [GitHub Repository]
//!
//! ## Known Issues/Future Work
//!
//! * The AI is not robust and merely choose random cards to discard. This will eventually be
//! replaced by a more functional AI algrothim (e.g. Monte Carlo Search Tree).
//! * There is no board displayed. Only the score and cards are shown. Eventually I would like to
//! add a board where the user can see the pegs moving.
//! * For this project, I made my own cards module, complete with Decks, Hands, and Cards. This was
//! merely as an excercise for myself, and maybe it would be best to use an external crate.
//!
//! [here are the rules]: https://en.wikipedia.org/wiki/Rules_of_cribbage
//! [GitHub Repository]: https://github.com/DirtGrubDylan/terminal_cribbage

extern crate itertools;
extern crate rand;

pub mod cards;
pub mod game;
