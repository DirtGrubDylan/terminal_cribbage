extern crate libterminal_cribbage;

use libterminal_cribbage::game::{Game, IoController, Player, RngController, UiDisplay};

fn main() {
    let title_text = String::new()
        + "================================================================================\n"
        + "   ______  _______     _____  ______   ______        _        ______  ________  \n"
        + " .' ___  ||_   __ \\   |_   _||_   _ \\ |_   _ \\      / \\     .' ___  ||_   __  | \n"
        + "/ .'   \\_|  | |__) |    | |    | |_) |  | |_) |    / _ \\   / .'   \\_|  | |_ \\_| \n"
        + "| |         |  __ /     | |    |  __'.  |  __'.   / ___ \\  | |   ____  |  _| _  \n"
        + "\\ `.___.'\\ _| |  \\ \\_  _| |_  _| |__) |_| |__) |_/ /   \\ \\_\\ `.___]  |_| |__/ | \n"
        + " `.____ .'|____| |___||_____||_______/|_______/|____| |____|`._____.'|________| \n"
        + "================================================================================\n";

    println!("{title_text}");
    println!("Time to cut the deck!");

    let player_1 = Player::new(IoController::new());
    let player_2 = Player::new(RngController::new());

    let mut game = Game::new_default(player_1, player_2, UiDisplay::new());

    game.play_default();
}
