
use crate::game::interfaces::killable::Killable;

pub struct Player {
    pub name: String,
    pub health: u32,
    pub stamina: u32,
    pub mana: u32,
}
impl Player {
    pub fn new_empty() -> Player {
        Player {
            name: String::from(""),
            health: 0,
            stamina: 0,
            mana: 0,
        }
    }

    pub fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            health: 100,
            stamina: 100,
            mana: 100,
        }
    }
    pub fn new_full(name: &str, health: u32, stamina: u32, mana: u32) -> Player {
        Player {
            name: String::from(name),
            health: health,
            stamina: stamina,
            mana: mana,
        }
    }


    pub fn damage_health(&mut self, health: u32) {
        if self.health - health == 0 {
            self.die();
            return;
        }
        self.health -= health;
    }

    pub fn damage_stamina(&mut self, health: u32) {

    }

    
    pub fn print_stats(&self) {
        println!("{}'s stats:", self.name);
        println!("Health: {}", self.health);
        println!("Stamina: {}", self.stamina);
        println!("Mana: {}", self.mana);
    }
}


impl Killable for Player {
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn die(&mut self) {
        println!("{} has died!", self.name);
    }
}