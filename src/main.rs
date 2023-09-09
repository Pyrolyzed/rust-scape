
mod game;

use game::entities::player::Player;
use game::lib::input::read_line;

const WELCOME_MSG: &str = "Welcome to RustScape!";

fn main() {
    
    welcome();

    let mut player = Player::new(&read_line("What is your name?".to_string()));

    player.print_stats();
}


fn welcome() {
    // Clears screen (since it's not obvious, I mean, what is this?)
    print!("\x1B[2J\x1B[1;1H");

    println!("{}", WELCOME_MSG);
}