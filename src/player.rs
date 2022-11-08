use std::cmp::{max, min};

use specs::Component;
use specs::{Join, WorldExt};
use specs_derive::Component;
use VirtualKeyCode::*;

use MenuMode::{Interact, Inventory};

use crate::components::items::get_item;
use crate::map::{is_tile_walkable, xy_to_idx, TileType};
use crate::MenuMode::Default;
use crate::{
    BTerm, DenseVecStorage, Log, MenuMode, Position, State, UserInterfaceState, VirtualKeyCode,
    World,
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

#[derive(Copy, Clone)]
pub enum ControlMode {
    Default,
    Inventory,
}

impl ControlMode {
    pub fn handle_input(&self, state: &mut State, ctx: &mut BTerm) {
        match self {
            ControlMode::Default => ControlMode::default(state, ctx),
            ControlMode::Inventory => ControlMode::inventory(state, ctx),
        }
    }

    fn default(state: &mut State, ctx: &mut BTerm) {
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
                Comma | G => get_item(&mut state.world),
                Apostrophe | Grave => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();
                    ui.log = !ui.log
                }
                Tab => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();
                    ui.menu = !ui.menu
                }
                O => Log::by_world(&state.world, "there are no options yet"),
                E => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();

                    match ui.menu_mode {
                        Inventory => ui.menu_mode = Default,
                        _ => ui.menu_mode = Inventory,
                    }

                    match ui.control_mode {
                        ControlMode::Inventory => ui.control_mode = ControlMode::Default,
                        _ => ui.control_mode = ControlMode::Inventory,
                    }
                }
                I => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();

                    match ui.menu_mode {
                        Interact => ui.menu_mode = Default,
                        _ => ui.menu_mode = Interact,
                    }
                }
                Q | Escape => ctx.quit(),
                _ => {}
            },
        }
    }

    fn inventory(state: &mut State, ctx: &mut BTerm) {
        match ctx.key {
            None => {}
            Some(key) => match key {
                Escape | Q => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();

                    ui.control_mode = ControlMode::Default;
                    ui.menu_mode = Default
                }
                _ => {}
            },
        }
    }
}
