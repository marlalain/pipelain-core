use std::cmp::{max, min};

use specs::Component;
use specs::{Join, WorldExt};
use specs_derive::Component;
use VirtualKeyCode::*;

use MenuMode::{Interact, Inventory};

use crate::components::items::{get_item, BlocksMovement};
use crate::gui::menu::craft;
use crate::map::{is_tile_walkable, xy_to_idx, TileType};
use crate::systems::craft::RECIPES;
use crate::MenuMode::{Craft, Default};
use crate::{
    BTerm, DenseVecStorage, Log, MenuMode, Position, State, UserInterfaceState, VirtualKeyCode,
    World,
};

#[derive(Component, Debug)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let blockers = world.read_storage::<BlocksMovement>();
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();
    let map = world.fetch::<Vec<TileType>>();

    let (player_x, player_y) = {
        let player = (&players, &positions).join().nth(0).unwrap();
        (player.1.x, player.1.y)
    };

    let destination_idx = xy_to_idx(player_x + delta_x, player_y + delta_y);

    let blocker = (&positions, &blockers).join().find(|(position, _)| {
        (player_x + delta_x) == position.x && (player_y + delta_y) == position.y
    });
    let is_blocked = blocker.is_some();
    if is_tile_walkable(map[destination_idx]) && !is_blocked {
        let player = (&mut players, &mut positions).join().nth(0).unwrap();
        player.1.x = min(79, max(0, player_x + delta_x));
        player.1.y = min(49, max(0, player_y + delta_y));
    }
}

#[derive(Copy, Clone, Default)]
pub enum ControlMode {
    #[default]
    Default,
    Inventory,
    Craft,
}

impl ControlMode {
    pub fn handle_input(&self, state: &mut State, ctx: &mut BTerm) {
        match self {
            ControlMode::Default => ControlMode::default(state, ctx),
            ControlMode::Inventory => ControlMode::inventory(state, ctx),
            ControlMode::Craft => ControlMode::craft(state, ctx),
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
                C => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();

                    match ui.menu_mode {
                        Craft => ui.menu_mode = Default,
                        _ => ui.menu_mode = Craft,
                    };

                    match ui.control_mode {
                        ControlMode::Craft => ui.control_mode = ControlMode::Default,
                        _ => ui.control_mode = ControlMode::Craft,
                    };

                    ui.selected_option = 0;
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

    fn craft(state: &mut State, ctx: &mut BTerm) {
        match ctx.key {
            None => {}
            Some(key) => match key {
                Escape | Q => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();

                    ui.control_mode = ControlMode::Default;
                    ui.menu_mode = Default
                }
                J | Down => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();

                    if ui.selected_option + 2 > RECIPES.len() {
                        return;
                    }

                    ui.selected_option += 1;
                }
                K | Up => {
                    let mut ui = state.world.fetch_mut::<UserInterfaceState>();

                    if ui.selected_option == 0 {
                        return;
                    }

                    ui.selected_option -= 1;
                }
                Return | Space => craft(state, ctx),
                _ => {}
            },
        }
    }
}
