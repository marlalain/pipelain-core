use crate::{BTerm, Log, World, BLACK};
use bracket_lib::color::{RGB, WHITE};

pub fn draw_ui(world: &World, ctx: &mut BTerm) {
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
