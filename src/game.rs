use std::cmp;
use rand::Rng;
use crate::{MAP_HEIGHT, MAP_WIDTH};
use crate::tile::*;
use crate::rect::*;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

pub type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map,
}

pub fn make_map() -> Map {
    // fill the map with blocked tiles.
    // The vec! macro is a shortcut that creates a Vec and fills it with values. For example, vec!['a'; 42] would create a Vec containing the letter 'a' 42 times. We do the same trick above to build a column of tiles and then build the map of those columns.
    let width = MAP_WIDTH as usize;
    let height = MAP_HEIGHT as usize;
    let tile = Tile::wall();
    let mut map = vec![vec![tile; height]; width];

    // generate rooms
    let room_1 = Rect::new(20, 15, 10, 15);
    let room_2 = Rect::new(50, 15, 10, 15);
    create_room(room_1, &mut map);
    create_room(room_2, &mut map);
    create_h_tunnel(25, 55, 23, &mut map);

    map
}

// fn generate_rooms(map: &mut Map) {
//
// }

fn create_room(room: Rect, map: &mut Map) {
    // go through all the tiles in the rect and make them passable
    for i in (room.x1 + 1)..room.x2 {
        for j in (room.y1 + 1)..room.y2 {
            let x = i as usize;
            let y = j as usize;
            map[x][y] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    let start = cmp::min(x1, x2);
    let end = (cmp::max(x1, x2) + 1);
    for x in start..end {
        let x = x as usize;
        let y = y as usize;
        map[x][y] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `y1 > y2`
    let start = cmp::min(y1, y2);
    let end = (cmp::max(y1, y2) + 1);
    for y in start..end {
        let x = x as usize;
        let y = y as usize;
        map[x][y] = Tile::empty();
    }
}