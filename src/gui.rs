use crate::{BTerm, Log, World, BLACK};
use bracket_lib::color::{RGB, WHITE};
use specs::Component;
use specs::DenseVecStorage;
use specs_derive::Component;

#[derive(Component)]
pub struct UserInterfaceState {
    pub log: bool,
}

pub fn draw_ui(world: &World, ctx: &mut BTerm) {
    let ui = world.fetch::<UserInterfaceState>();

    if !ui.log {
        return;
    }

    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));

    let log = world.fetch::<Log>();
    let mut y = 44;
    for entry in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, entry);
            y += 1;
        }
    }
}
