use bracket_lib::color::{BLACK, RED, RGB, YELLOW};
use bracket_lib::prelude::{
    main_loop, to_cp437, BError, BTerm, BTermBuilder, FontCharType, GameState,
};
use specs::Component;
use specs::DenseVecStorage;
use specs::{Builder, Join, World, WorldExt};
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    world: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("PipeLain").build()?;

    let mut state = State {
        world: World::new(),
    };

    state.world.register::<Position>();
    state.world.register::<Renderable>();

    let _player = state
        .world
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .build();

    for i in 0..10 {
        state
            .world
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: to_cp437('♂'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .build();
    }

    main_loop(context, state)
}
