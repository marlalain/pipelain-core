mod gui;
mod logs;
mod map;
mod player;
mod state;

use crate::gui::UserInterfaceState;
use crate::logs::Log;
use crate::map::new_map;
use crate::player::player_input;
use crate::player::Player;
use crate::state::State;
use bracket_lib::color::{BLACK, RGB, YELLOW};
use bracket_lib::prelude::{
    main_loop, to_cp437, BError, BTerm, BTermBuilder, FontCharType, GameState, VirtualKeyCode,
};
use specs::Component;
use specs::DenseVecStorage;
use specs::{Builder, World, WorldExt};
use specs_derive::Component;

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

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("PipeLain").build()?;

    let mut state = State {
        world: World::new(),
    };

    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<Player>();

    state.world.insert(new_map());
    state.world.insert(Log {
        entries: vec![
            "the game has fully loaded".to_string(),
            "press the apostrophe/grave key to show/hide the logs".to_string(),
        ],
    });
    state.world.insert(UserInterfaceState { log: true });

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
