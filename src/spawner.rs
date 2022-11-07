use bracket_lib::color::GREY;
use specs::{Builder, Entity, WorldExt, WriteStorage};

use crate::components::items;
use crate::components::items::{Flint, Item};
use crate::map::{xy_to_idx, TileType, HEIGHT, MAP_COUNT, WIDTH};
use crate::{to_cp437, Position, RandomNumberGenerator, Renderable, World, BLACK, RGB, YELLOW};

pub fn player(world: &mut World, x: i32, y: i32) -> Entity {
    world
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .build()
}

pub fn generate_items(world: &mut World) {
    generate_item(world, 128, flint);
}

fn generate_item(
    world: &mut World,
    chances: usize,
    generator: fn(&mut World, x: i32, y: i32) -> Entity,
) {
    let mut rng = RandomNumberGenerator::new();

    (0..(MAP_COUNT / chances)).into_iter().for_each(|_| {
        let x = rng.roll_dice(1, (WIDTH - 1) as i32);
        let y = rng.roll_dice(1, (HEIGHT - 1) as i32);
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
        .with(Item {})
        .with(Flint {})
        .build()
}
