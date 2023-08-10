//! This module holds the logic for the gameplay:
//! * Choose dealer
//! * Deal hands to players
//! * Discards from players for dealer crib
//! * Get starter card from deck
//! * Play (peg)
//! * Count hands
//! * Repeat until one player reaches 121pts

pub use self::controller::Controller;
pub use self::player::Player;
pub use self::predetermined_controller::PredeterminedController;

mod controller;
mod player;
mod predetermined_controller;
