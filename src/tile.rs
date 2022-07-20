// The #[derive(…)] automatically implements certain behaviors (Rust calls them traits, other languages use interfaces) you list there.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub is_blocked: bool,
    pub is_sight_blocked: bool,
    pub is_explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            is_blocked: false,
            is_sight_blocked: false,
            is_explored: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            is_blocked: true,
            is_sight_blocked: true,
            is_explored: false,
        }
    }
}