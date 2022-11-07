use bracket_lib::color::{GREEN, WHITE};

use crate::{BTerm, UserInterfaceState, World, BLACK, RGB};

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

    show_options(ctx, 62, 2);
}

fn show_options(ctx: &mut BTerm, x: i32, y: i32) {
    let menu = Menu {
        options: vec![
            option("d", "build"),
            option("i", "interact"),
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
