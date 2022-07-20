use tcod::colors::*;
use tcod::console::*;
use crate::PLAYER;
use crate::Map;
use crate::fighter::*;
use crate::game::*;

// This is a generic object: the player, a monster, an item, the stairs...
// It's always represented by a character on screen.
#[derive(Debug)]
pub struct GameObject {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub name: String,
    pub color: Color,
    pub blocks_tile: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
}

impl GameObject {
    pub fn new(x: i32, y: i32, char: char, name: &str, color: Color, blocks_tile: bool) -> Self {
        GameObject {
            x,
            y,
            char,
            name: name.into(),
            color,
            blocks_tile,
            alive: false,
            fighter: None,
            ai: None,
        }
    }
    // The dyn keyword in &mut dyn Console highlights that Console is a trait
    // and not a concrete type (such as a struct or enum).
    pub fn draw(&self, console: &mut dyn Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.char, BackgroundFlag::None)
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn distance_to(&self, other: &GameObject) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32) {
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage
            }
        }
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self);
            }
        }
    }

    pub fn attack(&mut self, target: &mut GameObject) {
        let my_power = self.fighter.map_or(0, |fighter| fighter.power);
        let target_defense = target.fighter.map_or(0, |fighter| fighter.defense);
        let damage = my_power - target_defense;
        if damage > 0 {
            println!(
                "{} attacks {} for {} hit points.",
                self.name, target.name, damage
            );
            target.take_damage(damage);
        } else {
            println!(
                "{} attacks {} but the attack is deflected by {}'s armor.",
                self.name, target.name, target.name
            );
        }
    }
}

// move by the given amount, if the destination is not blocked.
pub fn move_by(index: usize, dx: i32, dy: i32, map: &Map, game_objects: &mut [GameObject]) {
    let (x, y) = game_objects[index].get_position();
    let can_move = !is_blocked(x + dx, y + dy, map, game_objects);
    if can_move {
        game_objects[index].set_position(x + dx, y + dy);
    }
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &Game, game_objects: &mut [GameObject]) {
    let (x, y) = game_objects[PLAYER].get_position();
    let (x, y) = (x + dx, y + dy);

    // check for attackable game_object at destination
    // The position method on an iterator runs a test on each object
    // and as soon as it finds one, it returns its index in the collection
    // (in our case a vec of GameObject).
    // Notice: It’s possible no match will be found, so it actually returns Option<usize> here.
    let target_index = game_objects
        .iter()
        .position(|game_object|
            game_object.fighter.is_some()
                && game_object.get_position() == (x, y));

    // attack if target found, else try to move
    match target_index {
        Some(target_index) => {
            let (player, target) = mut_two(PLAYER, target_index, game_objects);
            player.attack(target);
        }
        None => {
            move_by(PLAYER, dx, dy, &game.map, game_objects);
        }
    }
}

pub fn move_toward(index: usize, target_x: i32, target_y: i32, map: &Map, game_objects: &mut [GameObject]) {
    // vector from this object to the target, and distance.
    let dx = target_x - game_objects[index].x;
    let dy = target_y - game_objects[index].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preserving direction), then round it and
    // convert to integer so the movement is restricted to the map grid.
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(index, dx, dy, map, game_objects);
}

// really is_blocked_or_occupied
pub fn is_blocked(x: i32, y: i32, map: &Map, game_objects: &[GameObject]) -> bool {
    // first check the map for blocking tiles:
    if map[x as usize][y as usize].is_blocked { return true; }

    // check for any GameObjects for blocking game objects:
    game_objects
        .iter()
        .any(|game_object| game_object.blocks_tile && game_object.get_position() == (x, y))
}

