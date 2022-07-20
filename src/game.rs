use std::cmp;
use rand::Rng;
use tcod::colors;
use crate::{Ai, DeathCallback, Fighter, GameObject, is_blocked, MAP_HEIGHT, MAP_WIDTH, move_toward, PLAYER, Tcod};
use crate::tile::*;
use crate::rect::*;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;

pub type Map = Vec<Vec<Tile>>;

// (deriving PartialEq lets us use == and != to compare the enums together)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub struct Game {
    pub map: Map,
}

pub fn make_map(game_objects: &mut Vec<GameObject>) -> Map {
    // fill the map with blocked tiles.
    // The vec! macro is a shortcut that creates a Vec and fills it with values. For example, vec!['a'; 42] would create a Vec containing the letter 'a' 42 times. We do the same trick above to build a column of tiles and then build the map of those columns.
    let width = MAP_WIDTH as usize;
    let height = MAP_HEIGHT as usize;
    let tile = Tile::wall();
    let mut map = vec![vec![tile; height]; width];

    // rooms
    let mut rooms = vec![];

    // generate rooms
    for _ in 0..MAX_ROOMS {
        // random width & height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random location
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);
        // create room
        let new_room = Rect::new(x, y, w, h);
        // check if it intersects with any other room, and if not, add it to rooms vec
        let intersects = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !intersects {
            // create room
            create_room(new_room, &mut map);
            // create monsters in room
            place_objects(new_room, &map, game_objects);

            // this will be useful later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // special case, place player in the first room
                let player = &mut game_objects[PLAYER];
                player.set_position(new_x, new_y);
            } else {
                // connect to previous room with a tunnel
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room)
        }
    }
    map
}

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
    let end = cmp::max(x1, x2 + 1);
    for x in start..end {
        let x = x as usize;
        let y = y as usize;
        map[x][y] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `y1 > y2`
    let start = cmp::min(y1, y2);
    let end = cmp::max(y1, y2) + 1;
    for y in start..end {
        let x = x as usize;
        let y = y as usize;
        map[x][y] = Tile::empty();
    }
}

fn place_objects(room: Rect, map: &Map, game_objects: &mut Vec<GameObject>) {
    // choose random amount of monsters
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        // choose a random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // check if the spot is occupied, if not, create the monster
        let occupied = is_blocked(x, y, map, game_objects);
        if !occupied {
            // Calling rand::random::<f32>() will produce an f32 number between 0.0 and 1.0
            let mut monster = if rand::random::<f32>() < 0.8 {
                // 80% chance of an Ork
                let mut orc = GameObject::new(x, y, 'o', "Orc", colors::DESATURATED_GREEN, true);
                orc.fighter = Some(Fighter {
                    max_hp: 10,
                    hp: 10,
                    defense: 0,
                    power: 3,
                    on_death: DeathCallback::Monster,
                });
                orc.ai = Some(Ai::Basic);
                orc
            } else {
                // 20% chance of a Troll
                let mut troll = GameObject::new(x, y, 'T', "Troll", colors::DARKER_GREEN, true);
                troll.fighter = Some(Fighter {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                    on_death: DeathCallback::Monster,
                });
                troll.ai = Some(Ai::Basic);
                troll
            };
            monster.alive = true;
            game_objects.push(monster);
        }
    }
}

pub fn ai_take_turn(monster_index: usize, tcod: &Tcod, game: &Game, game_objects: &mut [GameObject]) {
    // A basic monster takes its turn. If you can see it, it can see you!
    let (monster_x, monster_y) = game_objects[monster_index].get_position();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if game_objects[monster_index].distance_to(&game_objects[PLAYER]) >= 2.0 {
            // move toward player if far away.
            let (player_x, player_y) = game_objects[PLAYER].get_position();
            move_toward(monster_index, player_x, player_y, &game.map, game_objects);
        } else if game_objects[PLAYER].fighter.map_or(false, |fighter| fighter.hp > 0) {
            // close enough and player is alive
            let (monster, player) = mut_two(monster_index, PLAYER, game_objects);
            monster.attack(player);
        }
    }
}

// Need this to allow monster to attack
// see https://tomassedovic.github.io/roguelike-tutorial/part-6-going-berserk.html
// not sure if this is just a hack or not...
pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

