use bracket_lib::color::{BLACK, RED, RGB, YELLOW};
use bracket_lib::prelude::{
    main_loop, to_cp437, BError, BTerm, BTermBuilder, FontCharType, GameState, VirtualKeyCode,
};
use bracket_lib::terminal::Font;
use specs::Component;
use specs::DenseVecStorage;
use specs::{Builder, Join, World, WorldExt};
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

struct State {
    world: World,
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

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(state: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::H => try_move_player(-1, 0, &mut state.world),
            VirtualKeyCode::L => try_move_player(1, 0, &mut state.world),
            VirtualKeyCode::K => try_move_player(0, -1, &mut state.world),
            VirtualKeyCode::J => try_move_player(0, 1, &mut state.world),
            _ => {}
        },
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("PipeLain").build()?;

    let mut state = State {
        world: World::new(),
    };

    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<Player>();

    let _player = state
        .world
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
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
