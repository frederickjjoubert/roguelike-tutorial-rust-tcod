mod gameobject;

use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const FPS_LIMIT: i32 = 1000;

struct Tcod {
    root: Root,
    console: Offscreen,
}

fn handle_keys(tcod: &mut Tcod, player: &mut gameobject::GameObject) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        // Movement Keys
        Key { code: Up, .. } => player.move_by(0, -1), // The two dots at the end mean "I don’t care about the other fields". If it wasn’t there, it would not compile until you specified values for every field of the Key struct.
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),
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

fn main() {
    println!("Starting Game!");

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rusty Roguelike")
        .init();

    let console = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, console };

    tcod::system::set_fps(FPS_LIMIT);

    let player = gameobject::GameObject::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    let npc = gameobject::GameObject::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    let mut gameobjects = [player, npc];

    // Game Loop
    while !tcod.root.window_closed() {
        tcod.console.set_default_foreground(WHITE); // Draw everything as WHITE.
        tcod.console.clear(); // Clear the console from the previous frame.
        for gameobject in &gameobjects {
            gameobject.draw(&mut tcod.console);
        }

        blit(&tcod.console,
             (0, 0),
             (SCREEN_WIDTH, SCREEN_HEIGHT),
             &mut tcod.root,
             (0, 0),
             1.0,
             1.0);

        tcod.root.flush(); // Draw everything at once.
        tcod.root.wait_for_keypress(true);
        let player = &mut gameobjects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit { break; }
    }
}
