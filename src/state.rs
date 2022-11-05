pub struct State {
    pub world: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
