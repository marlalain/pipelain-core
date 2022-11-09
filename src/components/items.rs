use specs::{Entity, Join, WorldExt};

use crate::{Component, Player, Position, World};
use crate::{DenseVecStorage, Log};

#[derive(Component, Debug, Default)]
pub struct Item {
    pub can_be_picked: bool,
    pub can_be_crafted: bool,
}

pub fn get_item(world: &mut World) {
    let players = world.write_storage::<Player>();
    let positions = world.write_storage::<Position>();
    let (mut player_x, mut player_y) = (0, 0);

    for (_player, position) in (&players, &positions).join() {
        (player_x, player_y) = (position.x, position.y);
    }

    let player = world.fetch::<Entity>();
    let entities = world.entities();
    let items = world.read_storage::<Item>();
    let mut log = world.fetch_mut::<Log>();

    let mut target: Option<Entity> = None;
    for (item_entity, item, position) in (&entities, &items, &positions).join() {
        if !item.can_be_picked {
            continue;
        }

        if position.x == player_x && position.y == player_y {
            target = Some(item_entity);
        }
    }

    match target {
        None => log.log("there is nothing to be picked up here"),
        Some(item) => {
            let mut pickup = world.write_storage::<PickupQueue>();
            pickup
                .insert(
                    *player,
                    PickupQueue {
                        collected_by: *player,
                        item,
                    },
                )
                .expect("could not use pickup system");
        }
    }
}

#[derive(Component, Debug)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct PickupQueue {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct CraftQueue {
    pub item_name: String,
}

#[derive(Component)]
pub struct Tier {
    pub level: u8,
    pub alternative_name: Option<String>,
}

pub fn name_by_tier(level: u8) -> &'static str {
    match level {
        0 => "Flint",
        _ => todo!(),
    }
}

#[derive(Component)]
pub struct Flint {}

#[derive(Component)]
pub struct Bush {}

#[derive(Component)]
pub struct WoodenStick {}

#[derive(Component)]
pub struct Rose {}

#[derive(Component)]
pub struct Axe {}

#[derive(Component)]
pub struct FirePit {}
