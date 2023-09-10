
mod game;

use std::io;

use game::entities::player::Player;
use game::lib::input::*;
use game::lib::save_manager::*;


const WELCOME_MSG: &str = "Welcome to RustScape!";

fn main() {
    
    welcome();


    let player: Player = load_game().unwrap();

    player.print_stats();
}

fn load_game() -> Result<Player, io::Error> {
    if read_save_file().is_ok() {
        let save = read_save_file().unwrap();
        let player = Player::new_full(
            save.player_name.as_str(),
            save.player_health,
            save.player_stamina,
            save.player_mana,
        );
        Ok(player)
    } else {
        let player = Player::new(&read_line("What is your name?".to_string()));
        let _ = GameSave::new(&player);
        Ok(player)
    }
}

fn welcome() {
    // Clears screen (since it's not obvious, I mean, what is this?)
    print!("{esc}c", esc = 27 as char);

    println!("{}", WELCOME_MSG);
}