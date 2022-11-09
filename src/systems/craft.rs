use std::borrow::Borrow;
use std::collections::HashMap;

use specs::hibitset::{BitSetLike, DrainableBitSet};
use specs::{
    Entities, Entity, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
};

use crate::components::items::CraftQueue;
use crate::spawner::{axe, fire_pit};
use crate::{InBackpack, Name};

pub struct Requirement {
    pub item_name: &'static str,
    pub amount: i32,
}

pub struct Recipe {
    pub requirements: [Requirement; 2],
    pub result_item_name: &'static str,
}

pub const RECIPES: [Recipe; 2] = [
    Recipe {
        requirements: [
            Requirement {
                item_name: "Flint",
                amount: 3,
            },
            Requirement {
                item_name: "Wooden Stick",
                amount: 2,
            },
        ],
        result_item_name: "Flint Axe",
    },
    Recipe {
        requirements: [
            Requirement {
                item_name: "Flint",
                amount: 2,
            },
            Requirement {
                item_name: "Wooden Stick",
                amount: 1,
            },
        ],
        result_item_name: "Fire Pit",
    },
];

pub struct CraftSystem {}

impl<'a> System<'a> for CraftSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, CraftQueue>,
        WriteStorage<'a, InBackpack>,
        ReadStorage<'a, Name>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, entities, mut to_craft, mut backpack, names, lazy) = data;

        let item_to_craft = &to_craft.join().nth(0);
        if item_to_craft.is_none() {
            return;
        }

        let item_name = &item_to_craft.unwrap().item_name;
        let recipe = RECIPES
            .iter()
            .filter(|recipe| item_name == recipe.result_item_name)
            .nth(0)
            .unwrap();

        let to_remove = {
            let mut map = HashMap::new();

            (&to_craft, &backpack, &names)
                .join()
                .for_each(|(_, _, item_in_backpack)| {
                    let count = (&backpack, &names)
                        .join()
                        .filter(|(_, name)| &name.name == item_name)
                        .count();

                    map.insert(&item_in_backpack.name, count);
                });

            map
        };

        if to_remove.len() == 0 {
            to_craft.clear();
            return;
        };

        to_remove.iter().for_each(|(item_name, _)| {
            let name = item_name.clone().to_owned();
            let amount = recipe
                .requirements
                .iter()
                .filter(|requirement| name == requirement.item_name.to_string())
                .nth(0)
                .expect("could not find correct item to remove")
                .amount as usize;

            (&entities, &backpack, &names)
                .join()
                .filter(|(_, _, name)| &&name.name == item_name)
                .take(amount)
                .for_each(|item| entities.delete(item.0).expect("should delete item"));
        });

        match item_name.as_ref() {
            "Flint Axe" => axe(lazy.create_entity(&entities), *player, 0),
            "Fire Pit" => fire_pit(lazy.create_entity(&entities), *player),
            _ => println!("tried to craft {}", item_name),
        }

        to_craft.clear();
    }
}
