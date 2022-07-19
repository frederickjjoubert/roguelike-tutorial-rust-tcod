use crate::{MAP_HEIGHT, MAP_WIDTH};
use crate::tile::*;

type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map,
}

pub fn make_map() -> Map {
    // fill the map with unblocked tiles.
    // The vec! macro is a shortcut that creates a Vec and fills it with values. For example, vec!['a'; 42] would create a Vec containing the letter 'a' 42 times. We do the same trick above to build a column of tiles and then build the map of those columns.
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}