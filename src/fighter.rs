use tcod::colors::DARK_RED;
use crate::{GameObject};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ai {
    Basic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    pub fn callback(self, game_object: &mut GameObject) {
        use DeathCallback::*;
        let callback: fn(&mut GameObject) = match self {
            Player => player_death,
            Monster => monster_death
        };
        callback(game_object);
    }
}

fn player_death(player: &mut GameObject) {
    println!("You died!");
    println!("Press ESC to QUIT.");
    // Make the player a corpse.
    player.char = '%';
    player.color = DARK_RED;
}

fn monster_death(monster: &mut GameObject) {
    println!("{} has died!", monster.name);
    monster.char = '%';
    monster.color = DARK_RED;
    monster.blocks_tile = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("The remains of {}", monster.name);
}
