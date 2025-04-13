use serde::Deserialize;
use std::fs;
use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

pub fn get_config_dir() -> PathBuf {
    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg_config);
    }
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".config");
    }
    panic!("Could not determine config directory");
}

fn create_template(app:&str) {
    let config = get_config_dir()
        .join(app)
        .join("templates")
        .join("colors-pino.toml");

    fs::create_dir_all(config.parent().unwrap())
        .expect("Failed to create directories for template");
    println!("Creating template at: {:?}", config);
    let mut template = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&config)
        .expect("Can't Create Template File !!!");

    template
        .write_all(
            b"bg = \"{background}\"
fg = \"{foreground}\"

color0  = \"{color0}\"
color1  = \"{color1}\"
color2  = \"{color2}\"
color3  = \"{color3}\"
color4  = \"{color4}\"
color5  = \"{color5}\"
color6  = \"{color6}\"
color7  = \"{color7}\"
color8  = \"{color8}\"
color9  = \"{color9}\"
color10 = \"{color10}\"
color11 = \"{color11}\"
color12 = \"{color12}\"
color13 = \"{color13}\"
color14 = \"{color14}\"
color15 = \"{color15}\"
",
        )
        .expect("Can't Create Template File !!!");
}

#[derive(Debug, Deserialize)]
struct Pywal {
    bg: String,
    fg: String,
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
    color8: String,
    color9: String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
}

fn get_color<'a>(colors: &'a Pywal, key: &str) -> &'a str {
    match key {
        "bg" => &colors.bg,
        "fg" => &colors.fg,
        "color0" => &colors.color0,
        "color1" => &colors.color1,
        "color2" => &colors.color2,
        "color3" => &colors.color3,
        "color4" => &colors.color4,
        "color5" => &colors.color5,
        "color6" => &colors.color6,
        "color7" => &colors.color7,
        "color8" => &colors.color8,
        "color9" => &colors.color9,
        "color10" => &colors.color10,
        "color11" => &colors.color11,
        "color12" => &colors.color12,
        "color13" => &colors.color13,
        "color14" => &colors.color14,
        "color15" => &colors.color15,
        _ => panic!("Invalid key!"),
    }
}

pub fn pywal(
    background_color: String,
    border_color: String,
    title_color: String,
    message_color: String,
) ->(String,String,String,String) {
    let home = env::var("HOME").expect("Can't Find Home Dir");

    if !get_config_dir().join("wal/templates/colors-pino.toml").exists(){
        create_template("wal");
    }



    if !get_config_dir().join("walrs/templates/colors-pino.toml").exists(){
        create_template("walrs");
    }


    let cache_colors = PathBuf::from(home)
        .join(".cache")
        .join("wal")
        .join("colors-pino.toml");
    let content = fs::read_to_string(&cache_colors).expect("Can't Read Colors File!!!");
    let colors: Pywal = toml::from_str(&content).expect("Invalid TOML Format!!!");
     (
        get_color(&colors, background_color.as_str()).to_string(),
        get_color(&colors, border_color.as_str()).to_string(),
        get_color(&colors, title_color.as_str()).to_string(),
        get_color(&colors, message_color.as_str()).to_string(),
    )
}
