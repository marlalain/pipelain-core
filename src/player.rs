use crate::map::{xy_idx, TileType};
use crate::{BTerm, DenseVecStorage, Position, State, VirtualKeyCode, World};
use specs::Component;
use specs::{Join, WorldExt};
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();
    let map = world.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);

        if map[destination_idx] == TileType::Floor {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(state: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::H | VirtualKeyCode::Left => try_move_player(-1, 0, &mut state.world),
            VirtualKeyCode::L | VirtualKeyCode::Right => try_move_player(1, 0, &mut state.world),
            VirtualKeyCode::K | VirtualKeyCode::Up => try_move_player(0, -1, &mut state.world),
            VirtualKeyCode::J | VirtualKeyCode::Down => try_move_player(0, 1, &mut state.world),
            VirtualKeyCode::Q => ctx.quit(),
            _ => {}
        },
    }
}
