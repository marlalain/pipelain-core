use crate::{to_cp437, BTerm, RGB};
use bracket_lib::random::RandomNumberGenerator;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;
pub const MAP_COUNT: usize = HEIGHT * WIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Tree,
    Wall,
    Floor,
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

    let mut rng = RandomNumberGenerator::new();

    (0..(MAP_COUNT / 8)).into_iter().for_each(|_| {
        let x = rng.roll_dice(1, (WIDTH - 1) as i32);
        let y = rng.roll_dice(1, (HEIGHT - 1) as i32);
        let idx = xy_to_idx(x, y);

        let is_at_center = idx == xy_to_idx((WIDTH / 2) as i32, (HEIGHT / 2) as i32);
        let is_wall_already = map[idx] == TileType::Wall;
        if !is_at_center && !is_wall_already {
            map[idx] = TileType::Tree;
        }
    });

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut x = 0;
    let mut y = 0;

    map.iter().for_each(|tile| {
        match tile {
            TileType::Floor => ctx.set(
                x,
                y,
                RGB::from_f32(0.5, 0.5, 0.5),
                RGB::from_f32(0., 0., 0.),
                to_cp437('.'),
            ),
            TileType::Wall => ctx.set(
                x,
                y,
                RGB::from_f32(0.5, 0.5, 0.),
                RGB::from_f32(0., 0., 0.),
                to_cp437('#'),
            ),
            TileType::Tree => ctx.set(
                x,
                y,
                RGB::from_f32(0., 1., 0.),
                RGB::from_f32(0., 0., 0.),
                to_cp437('♠'),
            ),
        }

        x += 1;
        let should_be_next_row = x > WIDTH - 1;
        if should_be_next_row {
            x = 0;
            y += 1;
        }
    })
}

pub fn is_tile_walkable(tt: TileType) -> bool {
    match tt {
        TileType::Wall | TileType::Tree => false,
        _ => true,
    }
}
