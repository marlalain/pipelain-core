use crate::gui::draw_menu;
use crate::map::{draw_map, TileType};
use crate::{gui, player_input, BTerm, GameState, Position, Renderable, World};
use gui::draw_log;
use specs::{Join, WorldExt};

pub struct State {
    pub world: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);

        let map = self.world.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        draw_log(&self.world, ctx);
        draw_menu(&self.world, ctx);
    }
}
