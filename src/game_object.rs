use tcod::colors::*;
use tcod::console::*;
use crate::Game;

// This is a generic object: the player, a monster, an item, the stairs...
// It's always represented by a character on screen.
#[derive(Debug)]
pub struct GameObject {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl GameObject {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        GameObject { x, y, char, color }
    }

    // move by the given amount, if the destination is not blocked.
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        let x: usize = (self.x + dx) as usize;
        let y: usize = (self.y + dy) as usize;
        let can_move: bool = !game.map[x][y].blocked;
        if can_move {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, console: &mut dyn Console) { // The dyn keyword in &mut dyn Console highlights that Console is a trait and not a concrete type (such as a struct or enum).
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.char, BackgroundFlag::None)
    }
}