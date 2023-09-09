use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use serde_json;

use crate::game::entities::player::Player;

#[derive(Serialize, Deserialize)]
pub struct GameSave {
    pub player_name: String,
    pub player_health: u32,
    pub player_stamina: u32,
    pub player_mana: u32,
}

impl GameSave {
    pub fn new(player: &Player) -> GameSave {
        
        let game_save = GameSave {
            player_name: player.name.clone(),
            player_health: player.health,
            player_stamina: player.stamina,
            player_mana: player.mana,
        };

        match write_save_file(&game_save) {
            Ok(()) => println!("Game saved!"),
            Err(e) => println!("Error saving game: {}", e),
        }
        game_save
    }
}

pub fn save_to_player(game_save: GameSave) -> Player {
    Player::new_full(
        game_save.player_name.as_str(),
        game_save.player_health,
        game_save.player_stamina,
        game_save.player_mana,
    )
}

pub fn read_save_file() -> Result<GameSave, io::Error> {
    let mut file = File::open("save.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let save: GameSave = serde_json::from_str(&contents)?;
    Ok(save)
}

pub fn write_save_file(data: &GameSave) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("save.json")?;
    let json = serde_json::to_string(data)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}