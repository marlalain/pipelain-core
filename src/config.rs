use std::env;
use std::fmt::{Display, Formatter, Write};
use std::fs::{read_to_string, File};

use knuffel::Decode;
use knuffel::Error;

#[derive(Decode, Copy, Clone)]
pub enum Config {
    Performance(Performance),
}

#[derive(Decode, Copy, Clone)]
pub struct Performance {
    #[knuffel(property)]
    pub show_fps: bool,

    #[knuffel(property)]
    pub fps_cap: u8,
}

impl Display for Performance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(
            "Performance(show_fps={}, fps_cap={})",
            self.show_fps, self.fps_cap
        ))
    }
}

pub fn load_config() -> Config {
    let mut current_path =
        env::current_exe().expect("could not load current directory for configuration loading");
    current_path.pop(); // removes the binary
    current_path.push("config.kdl");

    if !current_path.as_path().exists() {
        File::create(&current_path).unwrap();
    }

    let config = parse_config(
        &current_path
            .to_str()
            .expect("could not parse config")
            .to_string(),
    )
    .unwrap();

    *config
        .iter()
        .nth(0)
        .unwrap_or(&Config::Performance(Performance {
            show_fps: true,
            fps_cap: 144,
        }))
}

fn parse_config(path: &String) -> Result<Vec<Config>, Error> {
    let text = read_to_string(path).unwrap();

    knuffel::parse::<Vec<Config>>(&*path, &text)
}
