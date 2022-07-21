use tcod::{BackgroundFlag, Color, Console, TextAlignment};
use tcod::colors::*;
use tcod::console::Offscreen;



pub fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    background_color: Color,
) {
    // Render a bar (Generic, can be HP, MANA, XP, etc.)
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;
    // Render the background first.
    panel.set_default_background(background_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);
    // Now render the bar on top.
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }
    // Now render text on top of the bar.
    panel.set_default_foreground(WHITE);
    panel.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", name, value, maximum),
    )
}