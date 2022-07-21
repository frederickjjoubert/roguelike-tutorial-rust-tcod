mod game_object;
mod tile;
mod game;
mod rect;
mod fighter;
mod gui;
mod messages;

use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use crate::fighter::*;
use crate::game::*;
use crate::game_object::*;
use crate::gui::render_bar;
use crate::messages::Messages;

const FPS_LIMIT: i32 = 100;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// GUI
const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

// This is so it appears to the right of the health bar, and fills up the rest of the space.
const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

// Game Map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const SIGHT_RADIUS: i32 = 10;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

const PLAYER: usize = 0;

pub struct Tcod {
    root: Root,
    // Everything is drawn to the root console (eventually).
    console: Offscreen,
    // We'll put our GUI here
    panel: Offscreen,
    // This represents the map only.
    fov: FovMap,
}

fn handle_keys(tcod: &mut Tcod, game: &mut Game, game_objects: &mut [GameObject]) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use crate::game::PlayerAction::*;

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = game_objects[PLAYER].alive;

    match (key, key.text(), player_alive) {
        // Fullscreen
        (Key { code: Enter, alt: true, .. }, _, _, ) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
        // Exit
        (Key { code: Escape, .. }, _, _) => {
            Exit
        }
        // Movement Keys
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, game_objects);
            TookTurn
        }
        // The two dots at the end mean "I don’t care about the other fields".
        // If it wasn’t there, it would not compile until you specified values for every field of the Key struct.
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, game_objects);
            TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, game_objects);
            TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, game_objects);
            TookTurn
        }

        // Everything else
        // _ => {} // This means "everything else" => "nothing happens"
        _ => DidntTakeTurn
    }
}

fn render_all(tcod: &mut Tcod, game: &mut Game, game_objects: &[GameObject], recompute_fov: bool) {
    // Recompute FOV (if needed).
    if recompute_fov {
        let player = &game_objects[PLAYER];
        tcod.fov.compute_fov(player.x, player.y, SIGHT_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    // Render Tiles
    // Go through all tiles, and set their background color:
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let x_index = x as usize;
            let y_index = y as usize;
            let blocks_sight = game.map[x_index][y_index].is_sight_blocked;

            let color = match (visible, blocks_sight) {
                // outside FOV
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                // inside FOV
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND
            };

            let explored = &mut game.map[x_index][y_index].is_explored;
            if visible {
                // if it's visible, mark it as explored
                *explored = true;
            }
            if *explored {
                // show explored tiles only
                tcod.console.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    // Instead of going through the objects list we clone it into a mutable vector
    // (render_all is taking &[Object] so it can’t change the list directly, nor should it).
    // Then we sort the vector such that all non-blocking objects come before all
    // the blocking ones. Since we can’t have two blocking objects on the same tile,
    // this will make sure that our player and monsters won’t get overwritten by corpses.
    let mut to_draw: Vec<_> = game_objects
        .iter()
        // filter out game objects that arent within FOV since we're not going to render them.
        .filter(|game_object| tcod.fov.is_in_fov(game_object.x, game_object.y))
        .collect();
    // sort so that non-blocking objects come first.
    to_draw.sort_by(|game_object_1, game_object_2|
        game_object_1.blocks_tile.cmp(&game_object_2.blocks_tile));

    // draw all the objects in the list that are within the FOV:
    for game_object in to_draw {
        game_object.draw(&mut tcod.console);
    }

    // GUI
    // Prepare to render the GUI Panel
    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();
    // Show the players stats
    let hp = game_objects[PLAYER]
        .fighter
        .map_or(0, |fighter| fighter.hp);
    let max_hp = game_objects[PLAYER]
        .fighter
        .map_or(0, |fighter| fighter.max_hp);
    render_bar(
        &mut tcod.panel,
        1, 1,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        LIGHT_RED,
        DARKER_RED,
    );
    // Print the Messages
    let mut y = MSG_HEIGHT as i32;
    // We’re going through the messages backwards (starting with the last message),
    // because we don’t know if we get to print all.
    // So we first calculate the height of the message (in case it gets wrapped),
    // we draw it at the corresponding y position by subtracting the height and then repeat.
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    // Blit the contents of "console" to the root console.
    blit(
        &tcod.console,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );

    // Blit the contents of "panel" to the root console.
    blit(
        &tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_Y),
        &mut tcod.root,
        (0, PANEL_Y),
        1.0,
        1.0,
    );
}

fn main() {
    println!("Starting Game!");

    // Set tcod lib fps limit
    tcod::system::set_fps(FPS_LIMIT);

    // Prepare Tcod parameters
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rusty Roguelike")
        .init();
    let console = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let panel = Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT);
    let fov_map: FovMap = FovMap::new(MAP_WIDTH, MAP_HEIGHT);

    // create Tcod
    let mut tcod = Tcod { root, console, panel, fov: fov_map };

    // Create player and add to list of game objects. Position will be set in 'make_map(...)'.
    let mut player = GameObject::new(0, 0, '@', "Player", WHITE, false);
    player.alive = true;
    player.fighter = Some(Fighter {
        max_hp: 30,
        hp: 30,
        defense: 2,
        power: 5,
        on_death: DeathCallback::Player,
    });
    let mut game_objects = vec![];
    game_objects.push(player);

    let mut game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(&mut game_objects),
        messages: Messages::new(),
    };

    // populate the FOV map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].is_sight_blocked,
                !game.map[x as usize][y as usize].is_blocked,
            )
        }
    }

    // force FOV "recompute" first time through the game loop
    let mut previous_player_position = (-1, -1);

    // test - a warm welcoming message
    game.messages.add(
        "Welcome to the dungeon! Prepare to die.",
        RED,
    );

    // Game Loop
    while !tcod.root.window_closed() {
        // Clear the console from the previous frame.
        tcod.console.clear();

        // Render the screen and recompute FOV if needed.
        let fov_recompute = previous_player_position != game_objects[PLAYER].get_position();
        render_all(&mut tcod, &mut game, &game_objects, fov_recompute);

        // Draw everything at once.
        tcod.root.flush();

        // Handle Input and Exit if needed.
        previous_player_position = game_objects[PLAYER].get_position();
        // player turn
        let player_action = handle_keys(&mut tcod, &mut game, &mut game_objects);
        if player_action == PlayerAction::Exit { break; }

        if game_objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            // monsters take their turn
            // for game_object in &game_objects {
            //     // only if object is not the player.
            //     // The as *const _ bit is there to do a pointer comparison.
            //     // Rust’s equality operators (== and !=) test for value equality,
            //     // but we haven’t implemented that for Object and we don’t care anyway,
            //     // we just want to make sure to not process player here.
            //     if (game_object as *const _) != (&game_objects[PLAYER] as *const _) {
            //         println!("The {} growls!", game_object.name);
            //     }
            // }
            for index in 0..game_objects.len() {
                if game_objects[index].ai.is_some() {
                    ai_take_turn(index, &tcod, &mut game, &mut game_objects);
                }
            }
        }
    }
}
