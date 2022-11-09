use bracket_lib::color::{BROWN1, BROWN2, BURLYWOOD, GREY, RED};
use bracket_lib::random::RandomNumberGenerator;

use crate::spawner::generate_items;
use crate::{to_cp437, BTerm, BLACK, RGB};

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;
pub const MAP_COUNT: usize = HEIGHT * WIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    pub fn render_custom(&self, ctx: &mut BTerm, x: i32, y: i32, fg: RGB, bg: RGB) {
        match self {
            TileType::Floor => ctx.set(x, y, fg, bg, to_cp437('.')),
            TileType::Wall => ctx.set(x, y, fg, bg, to_cp437('#')),
        }
    }

    pub fn render(&self, ctx: &mut BTerm, x: i32, y: i32) {
        match self {
            TileType::Floor => {
                self.render_custom(ctx, x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::named(BLACK))
            }
            TileType::Wall => self.render_custom(
                ctx,
                x,
                y,
                RGB::from_f32(0.25, 0.25, 0.25),
                RGB::named(BLACK),
            ),
        }
    }
}

pub fn xy_to_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; WIDTH * HEIGHT];

    for x in 0..(WIDTH as i32) {
        map[xy_to_idx(x, 0)] = TileType::Wall;
        map[xy_to_idx(x, (HEIGHT - 1) as i32)] = TileType::Wall;
    }

    for y in 0..(HEIGHT as i32) {
        map[xy_to_idx(0, y)] = TileType::Wall;
        map[xy_to_idx((WIDTH - 1) as i32, y)] = TileType::Wall;
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut x = 0;
    let mut y = 0;

    map.iter().for_each(|tile| {
        match tile {
            t => t.render(ctx, x, y),
        };

        x += 1;
        let should_be_next_row = x > (WIDTH - 1) as i32;
        if should_be_next_row {
            x = 0;
            y += 1;
        }
    })
}

pub fn is_tile_walkable(tt: TileType) -> bool {
    match tt {
        TileType::Wall => false,
        _ => true,
    }
}
