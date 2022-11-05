mod player;
mod state;

use crate::player::player_input;
use crate::player::Player;
use crate::state::State;
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

    main_loop(context, state)
}
