use std::collections::{BTreeMap, HashMap};
use std::os::macos::raw::stat;

use bracket_lib::color::{GREEN, RED, WHITE};
use specs::hibitset::BitSetLike;
use specs::shred::Fetch;
use specs::{Entity, Join, ReadStorage, WorldExt};

use crate::map::{xy_to_idx, TileType};
use crate::systems::craft::RECIPES;
use crate::{
    new_map, to_cp437, BTerm, CraftQueue, InBackpack, Item, Log, MenuMode, Name, Player, Position,
    Renderable, State, UserInterfaceState, World, BLACK, RGB,
};

#[derive(Clone)]
struct MenuOption {
    key: String,
    name: String,
}

impl MenuOption {
    fn print(self, ctx: &mut BTerm, x: i32, y: i32) {
        ctx.print_color(x, y, RGB::named(GREEN), RGB::named(BLACK), self.key.clone());

        let new_x = x + self.key.len() as i32;
        ctx.print(new_x, y, format!(": {}", self.name))
    }
}

struct Menu {
    options: Vec<MenuOption>,
}

pub fn draw_menu(world: &World, ctx: &mut BTerm) {
    let ui = world.fetch::<UserInterfaceState>();

    if !ui.menu {
        return;
    }

    let height = match ui.log {
        true => 42,
        false => 49,
    };
    ctx.draw_box(60, 0, 19, height, RGB::named(WHITE), RGB::named(BLACK));

    match ui.menu_mode {
        MenuMode::Default | MenuMode::Inventory | MenuMode::Craft => show_options(ctx, 62, 2),
        MenuMode::Interact => show_interact(world, ctx, 62, 2),
    }
}

fn show_interact(world: &World, ctx: &mut BTerm, x: i32, y: i32) {
    struct Option {
        glyph: String,
        name: String,
    };

    {
        let map = world.fetch::<Vec<TileType>>();
        let mut players = world.write_storage::<Player>();
        let mut positions = world.write_storage::<Position>();

        let (mut player_x, mut player_y) = (0, 0);
        for (_player, position) in (&players, &positions).join() {
            player_x = position.x;
            player_y = position.y;
        }

        (0..3).into_iter().for_each(|raw_offset_x| {
            let offset_x = raw_offset_x - 1 + &player_x;
            (0..3).into_iter().for_each(|raw_offset_y| {
                let offset_y = raw_offset_y - 1 + &player_y;

                if &offset_x == &player_x && &offset_y == &player_y {
                    return;
                }

                let red = RGB::named(RED);
                let black = RGB::named(BLACK);
                let tile = map[xy_to_idx(offset_x, offset_y)];

                match tile {
                    t => t.render_custom(ctx, offset_x, offset_y, red, black),
                }
            })
        });

        ctx.print(x, y, "no objects to");
        ctx.print(x, y + 1, "interact here")
    }
}

fn show_options(ctx: &mut BTerm, x: i32, y: i32) {
    let menu = Menu {
        options: vec![
            option("d", "build"),
            option("i", "interact"),
            option("e", "backpack"),
            option("c", "craft"),
            option("o", "options"),
        ],
    };

    (0..menu.options.len()).into_iter().for_each(|i| {
        let option = menu.options.get(i).expect("out of bounds").clone();
        option.print(ctx, x, y + (i as i32 * 2) + 1);
    });
}

fn option(key: &str, name: &str) -> MenuOption {
    MenuOption {
        key: key.to_string(),
        name: name.to_string(),
    }
}

pub fn show_inventory(state: &mut State, ctx: &mut BTerm) {
    let player = state.world.fetch::<Entity>();
    let names = state.world.read_storage::<Name>();
    let backpack = state.world.read_storage::<InBackpack>();

    ctx.draw_box(2, 2, 30, 30, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_centered_at(17, 2, "backpack");

    let mut inventory: BTreeMap<&String, i32> = BTreeMap::new();

    for (_pack, name) in (&backpack, &names)
        .join()
        .filter(|item| item_owner_is_player(item, &player))
    {
        if inventory.contains_key(&name.name) {
            let _table = inventory.clone();
            let entry = _table.get(&name.name).unwrap();
            inventory.insert(&name.name, entry + 1);
        } else {
            inventory.insert(&name.name, 1);
        }
    }

    let mut y = 4;
    inventory.iter().for_each(|(name, amount)| {
        ctx.print(4, y, format!("{} x{}", name, amount));
        y += 1;
    })
}

fn item_owner_is_player(item: &(&InBackpack, &Name), player: &Fetch<Entity>) -> bool {
    item.0.owner == **player
}

pub fn show_craft(state: &mut State, ctx: &mut BTerm) {
    let ui = state.world.fetch::<UserInterfaceState>();
    let player = state.world.fetch::<Entity>();
    let backpack = state.world.read_storage::<InBackpack>();
    let names = state.world.read_storage::<Name>();

    ctx.draw_box(2, 2, 30, 30, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_centered_at(17, 2, "craft");

    let selected_y = 4 + ui.selected_option;
    ctx.set(
        4,
        selected_y,
        RGB::named(WHITE),
        RGB::named(BLACK),
        to_cp437('→'),
    );

    let inventory: HashMap<&String, i32> = {
        let mut inner = HashMap::new();

        for (_pack, name) in (&backpack, &names)
            .join()
            .filter(|item| item_owner_is_player(item, &player))
        {
            if inner.contains_key(&name.name) {
                let amount = inner.get(&name.name).unwrap();
                inner.insert(&name.name, amount + 1);
            } else {
                inner.insert(&name.name, 1);
            }
        }

        inner
    };

    let mut y = 4;
    RECIPES.iter().for_each(|recipe| {
        let can_craft_item = {
            let mut requirements: HashMap<&str, i32> = HashMap::new();

            recipe.requirements.iter().for_each(|requirement| {
                if requirements.contains_key(requirement.item_name) {
                    let old_amount = requirements.get(requirement.item_name).unwrap();
                    requirements.insert(requirement.item_name, old_amount + requirement.amount);
                } else {
                    requirements.insert(requirement.item_name, requirement.amount);
                }
            });

            requirements.iter().fold(true, |can_craft, requirement| {
                let inventory_amount = inventory.get(&requirement.0.to_string()).unwrap_or(&0);
                can_craft && requirement.1 <= inventory_amount
            })
        };

        ctx.print_color(
            6,
            y,
            can_craft(can_craft_item),
            RGB::named(BLACK),
            &recipe.result_item_name,
        );

        y += 1;
    })
}

fn can_craft(check: bool) -> RGB {
    if check {
        RGB::named(GREEN)
    } else {
        RGB::named(RED)
    }
}

pub fn craft(state: &mut State, ctx: &mut BTerm) {
    let mut wants_to_craft = state.world.write_storage::<CraftQueue>();
    let entities = state.world.entities();
    let items = state.world.read_storage::<Item>();

    let selected_option = {
        let mut ui = state.world.fetch_mut::<UserInterfaceState>();
        ui.selected_option
    };

    let to_craft = RECIPES.get(selected_option).unwrap();

    for (entity, _item) in (&entities, &items).join() {
        wants_to_craft
            .insert(
                entity,
                CraftQueue {
                    item_name: to_craft.result_item_name.to_string(),
                },
            )
            .expect("can't craft item");
    }
}
