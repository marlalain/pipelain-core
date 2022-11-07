use bracket_lib::color::{GREEN, RGB, WHITE};

use crate::map::WIDTH;
use crate::{BTerm, Log, World, BLACK};

pub mod menu;

pub enum MenuMode {
    Default,
    Interact,
}

pub struct UserInterfaceState {
    pub log: bool,
    pub menu: bool,
    pub mode: MenuMode,
}

pub fn draw_log(world: &World, ctx: &mut BTerm) {
    let ui = world.fetch::<UserInterfaceState>();

    if !ui.log {
        return;
    }

    let width = WIDTH - 1;
    ctx.draw_box(0, 43, width, 6, RGB::named(WHITE), RGB::named(BLACK));

    let log = world.fetch::<Log>();
    let mut y = 44;
    for entry in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, entry);
            y += 1;
        }
    }
}