mod game_object;
mod tile;
mod game;
mod rect;

use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use crate::game::*;
use crate::game_object::*;

const FPS_LIMIT: i32 = 100;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const SIGHT_RADIUS: i32 = 10;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

struct Tcod {
    root: Root,
    // Everything is drawn to the root console (eventually).
    console: Offscreen,
    // This represents the map only.
    fov: FovMap,
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut GameObject) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        // Fullscreen
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        // Exit
        Key { code: Escape, .. } => {
            return true;
        }
        // Movement Keys
        Key { code: Up, .. } => player.move_by(0, -1, game), // The two dots at the end mean "I don’t care about the other fields". If it wasn’t there, it would not compile until you specified values for every field of the Key struct.
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        // Everything else
        _ => {} // This means "everything else" => "nothing happens"
    }

    false
}

fn render_all(tcod: &mut Tcod, game: &mut Game, game_objects: &[GameObject], recompute_fov: bool) {
    // recompute FOV if needed
    if recompute_fov {
        let player = &game_objects[0];
        tcod.fov.compute_fov(player.x, player.y, SIGHT_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    // go through all tiles, and set their background color:
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let x_index = x as usize;
            let y_index = y as usize;
            let blocks_sight = game.map[x_index][y_index].block_sight;

            let color = match (visible, blocks_sight) {
                // outside FOV
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                // inside FOV
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND
            };

            let explored = &mut game.map[x_index][y_index].explored;
            if visible {
                // if it's visible, mark it as explored
                *explored = true;
            }
            if *explored {
                // show explored tiles only
                tcod.console.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }

        // draw all the objects in the list that are within the FOV:
        for game_object in game_objects {
            if tcod.fov.is_in_fov(game_object.x, game_object.y) {
                game_object.draw(&mut tcod.console);
            }
        }
    }

    // blit the contents of "console" to the root console
    blit(&tcod.console,
         (0, 0),
         (MAP_WIDTH, MAP_HEIGHT),
         &mut tcod.root,
         (0, 0),
         1.0,
         1.0);
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
    let fov_map: FovMap = FovMap::new(MAP_WIDTH, MAP_HEIGHT);

    // create Tcod
    let mut tcod = Tcod { root, console, fov: fov_map };

    // place the player and add these game objects to the game objects list.
    let player = GameObject::new(25, 23, '@', WHITE);
    let npc = GameObject::new(27, 23, '@', YELLOW);
    let mut game_objects = [player, npc];

    let mut game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(&mut game_objects[0]),
    };

    // populate the FOV map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            )
        }
    }

    // force FOV "recompute" first time through the game loop
    let mut previous_player_position = (-1, -1);

    // Game Loop
    while !tcod.root.window_closed() {
        // Clear the console from the previous frame.
        tcod.console.clear();

        // Render the screen and recompute FOV if needed.
        let player = &game_objects[0];
        let fov_recompute = previous_player_position != (player.x, player.y);
        render_all(&mut tcod, &mut game, &game_objects, fov_recompute);

        // Draw everything at once.
        tcod.root.flush();

        // Handle Input and Exit if needed.
        let player = &mut game_objects[0];
        previous_player_position = (player.x, player.y);
        let exit = handle_keys(&mut tcod, &game, player);
        if exit { break; }
    }
}
