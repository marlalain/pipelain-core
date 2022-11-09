use bracket_lib::color::{BURLYWOOD, GREY, RED};
use specs::world::{EntityResBuilder, LazyBuilder};
use specs::{Builder, Entity, EntityBuilder, ReadExpect, WorldExt, WriteStorage};

use crate::components::items;
use crate::components::items::{name_by_tier, Flint, Item, Rose};
use crate::map::{xy_to_idx, TileType, HEIGHT, MAP_COUNT, WIDTH};
use crate::{
    to_cp437, Axe, Bush, FirePit, InBackpack, Name, Player, Position, RandomNumberGenerator,
    Renderable, Tier, WoodenStick, World, BLACK, RGB, YELLOW,
};

pub fn player(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(Name {
            name: "Player".to_string(),
        })
        .build()
}

pub fn generate_items(world: &mut World) {
    generate_item(world, 32, bush);
    generate_item(world, 64, wooden_stick);
    generate_item(world, 64, rose);
    generate_item(world, 128, flint);
}

fn generate_item(
    world: &mut World,
    chances: usize,
    generator: fn(&mut World, x: i32, y: i32) -> Entity,
) {
    let mut rng = RandomNumberGenerator::new();

    (0..(MAP_COUNT / chances)).into_iter().for_each(|_| {
        let x = rng.roll_dice(1, (WIDTH - 2) as i32);
        let y = rng.roll_dice(1, (HEIGHT - 2) as i32);
        let idx = xy_to_idx(x, y);

        let is_at_center = idx == xy_to_idx((WIDTH / 2) as i32, (HEIGHT / 2) as i32);
        if !is_at_center {
            generator(world, x, y);
        }
    });
}

fn flint(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('°'),
            fg: RGB::named(GREY),
            bg: RGB::named(BLACK),
        })
        .with(Item {
            can_be_picked: true,
            can_be_crafted: false,
        })
        .with(Flint {})
        .with(Name {
            name: "Flint".to_string(),
        })
        .build()
}

fn bush(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('%'),
            fg: RGB::from_f32(0., 0.75, 0.),
            bg: RGB::named(BLACK),
        })
        .with(Item::default())
        .with(Bush {})
        .with(Name {
            name: "Bush".to_string(),
        })
        .build()
}

fn wooden_stick(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('\\'),
            fg: RGB::named(BURLYWOOD),
            bg: RGB::named(BLACK),
        })
        .with(Item {
            can_be_picked: true,
            can_be_crafted: false,
        })
        .with(WoodenStick {})
        .with(Name {
            name: "Wooden Stick".to_string(),
        })
        .build()
}

fn rose(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('±'),
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
        })
        .with(Item::default())
        .with(Rose {})
        .with(Name {
            name: "Rose".to_string(),
        })
        .build()
}

fn craftable() -> Item {
    Item {
        can_be_picked: true,
        can_be_crafted: true,
    }
}

pub fn axe(builder: LazyBuilder, owner: Entity, level: u8) {
    builder
        .with(craftable())
        .with(Axe {})
        .with(Name {
            name: format!("{} Axe", name_by_tier(level).to_string()),
        })
        .with(Tier {
            alternative_name: Some(format!("{} Axe", name_by_tier(level))),
            level,
        })
        .with(InBackpack { owner })
        .build();
}

pub fn fire_pit(builder: LazyBuilder, owner: Entity) {
    builder
        .with(craftable())
        .with(FirePit {})
        .with(Name {
            name: "Fire Pit".to_string(),
        })
        .with(InBackpack { owner })
        .build();
}
