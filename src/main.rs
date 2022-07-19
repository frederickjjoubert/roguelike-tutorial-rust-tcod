mod gameobject;
mod tile;
mod game;

use tcod::colors::*;
use tcod::console::*;
use crate::game::*;
use crate::gameobject::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const FPS_LIMIT: i32 = 100;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

struct Tcod {
    root: Root,
    // Everything is drawn to the root console (eventually).
    console: Offscreen, // This represents the map only.
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut gameobject::GameObject) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        // Movement Keys
        Key { code: Up, .. } => player.move_by(0, -1, game), // The two dots at the end mean "I don’t care about the other fields". If it wasn’t there, it would not compile until you specified values for every field of the Key struct.
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => {
            return true;
        }
        _ => {} // This means "everything else" => "nothing happens"
    }

    false
}

fn render_all(tcod: &mut Tcod, game: &Game, gameobjects: &[GameObject]) {
    // draw all the objects in the list:
    for gameobject in gameobjects {
        gameobject.draw(&mut tcod.console);
    }

    // go through all tiles, and set their background color:
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.console.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set)
            } else {
                tcod.console.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set)
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

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rusty Roguelike")
        .init();

    let console = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod { root, console };

    tcod::system::set_fps(FPS_LIMIT);

    let player = gameobject::GameObject::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    let npc = gameobject::GameObject::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    let mut gameobjects = [player, npc];

    let game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(),
    };

    // Game Loop
    while !tcod.root.window_closed() {
        tcod.console.set_default_foreground(WHITE); // Draw everything as WHITE.
        tcod.console.clear(); // Clear the console from the previous frame.

        render_all(&mut tcod, &game, &gameobjects);

        tcod.root.flush(); // Draw everything at once.
        tcod.root.wait_for_keypress(true);
        let player = &mut gameobjects[0];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit { break; }
    }
}
