use tcod::colors::*;
use tcod::console::*;

// This is a generic object: the player, a monster, an item, the stairs...
// It's always represented by a character on screen.
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

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, console: &mut dyn Console) { // The dyn keyword in &mut dyn Console highlights that Console is a trait and not a concrete type (such as a struct or enum).
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.char, BackgroundFlag::None)
    }
}