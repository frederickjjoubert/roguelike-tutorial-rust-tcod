// The #[derive(…)] automatically implements certain behaviors (Rust calls them traits, other languages use interfaces) you list there.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }

    pub fn chasm() -> Self {
        Tile {
            blocked: true,
            block_sight: false,
        }
    }

    pub fn hidden_passage() -> Self {
        Tile {
            blocked: false,
            block_sight: true,
        }
    }
}