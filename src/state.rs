use specs::{Join, RunNow, WorldExt};

use gui::draw_log;

use crate::gui::menu::draw_menu;
use crate::map::{draw_map, TileType};
use crate::systems::interaction::Interactable;
use crate::{gui, player_input, BTerm, GameState, Name, Player, Position, Renderable, World};

pub struct State {
    pub world: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut interaction = Interactable {};
        interaction.run_now(&self.world);

        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.run_systems();

        player_input(self, ctx);

        let map = self.world.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let names = self.world.read_storage::<Name>();
        let players = self.world.read_storage::<Player>();
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (pos, render, name) in (&positions, &renderables, &names).join() {
            if name.name == "Player".to_string() {
                continue;
            }

            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        for (pos, render, _player) in (&positions, &renderables, &players).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        drop(players);
        drop(positions);
        drop(renderables);

        draw_log(&self.world, ctx);
        draw_menu(&self.world, ctx);
    }
}
