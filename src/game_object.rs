use tcod::colors::*;
use tcod::console::*;
use crate::{Game, PLAYER};
use crate::Map;

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
        }
    }

    pub fn draw(&self, console: &mut dyn Console) { // The dyn keyword in &mut dyn Console highlights that Console is a trait and not a concrete type (such as a struct or enum).
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
    let target_index = game_objects.iter().position(|game_object| game_object.get_position() == (x, y));

    // attack if target found, else try to move
    match target_index {
        Some(target_index) => {
            println!(
                "The {} laughs at your puny attempt to attack!",
                game_objects[target_index].name
            );
        }
        None => {
            move_by(PLAYER, dx, dy, &game.map, game_objects);
        }
    }
}

// really is_blocked_or_occupied
pub fn is_blocked(x: i32, y: i32, map: &Map, game_objects: &[GameObject]) -> bool {
    // first check the map for blocking tiles:
    if map[x as usize][y as usize].is_blocked { return true; }

    // check for any GameObjects for blocking game objects:
    game_objects
        .iter()
        .any(|mut game_object| game_object.blocks_tile && game_object.get_position() == (x, y))
}