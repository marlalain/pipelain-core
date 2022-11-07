use std::cmp::{max, min};

use specs::Component;
use specs::{Join, WorldExt};
use specs_derive::Component;
use VirtualKeyCode::{Apostrophe, Down, Grave, Left, Right, Tab, Up, B, H, J, K, L, N, O, U, Y};

use crate::map::{is_tile_walkable, xy_to_idx, TileType};
use crate::{
    BTerm, DenseVecStorage, Log, Position, State, UserInterfaceState, VirtualKeyCode, World,
};

#[derive(Component, Debug)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();
    let map = world.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_to_idx(pos.x + delta_x, pos.y + delta_y);

        if is_tile_walkable(map[destination_idx]) {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(state: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            H | Left => try_move_player(-1, 0, &mut state.world),
            L | Right => try_move_player(1, 0, &mut state.world),
            K | Up => try_move_player(0, -1, &mut state.world),
            J | Down => try_move_player(0, 1, &mut state.world),
            Y => try_move_player(-1, -1, &mut state.world),
            U => try_move_player(1, -1, &mut state.world),
            B => try_move_player(-1, 1, &mut state.world),
            N => try_move_player(1, 1, &mut state.world),
            Apostrophe | Grave => {
                let mut ui = state.world.fetch_mut::<UserInterfaceState>();
                ui.log = !ui.log
            }
            Tab => {
                let mut ui = state.world.fetch_mut::<UserInterfaceState>();
                ui.menu = !ui.menu
            }
            O => {
                Log::info(&state.world, "there are no options yet");
            }
            VirtualKeyCode::Q => ctx.quit(),
            _ => {}
        },
    }
}
