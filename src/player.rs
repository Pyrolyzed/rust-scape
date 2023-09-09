
use crate::interfaces::killable::Killable;

pub struct Player {
    name: String,
    health: u32,
    stamina: u32,
    mana: u32,
}
impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            health: 100,
            stamina: 100,
            mana: 100,
        }
    }

    pub fn damage_health(&mut self, health: u32) {
        if self.health - health == 0 {
            self.die();
            return;
        }
        self.health -= health;
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