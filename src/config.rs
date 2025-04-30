use std::{
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::PathBuf,
};

pub fn generate_config(config_path: PathBuf) {
    let config = config_path.join("pino").join("config.toml");

    create_dir_all(config.parent().unwrap()).expect("Failed to create directories for template");
    println!("Creating config file at: {}", config.display());
    let mut template = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&config)
        .expect("Can't Create Template File !!!");

    template
        .write_all(
            b"[screen]
monitor = 0 # Set the monitor using index


horizontal = \"left\"   #[right] |  [left]
vertical = \"top\"      # [top]  | [bottom]
x = 25                  # X access palcement
y = 55                  # Y access palcement

width = 300     # Width  of the app
height = 100    # Height of the app

delay = 5 # Time to display the app (in seconds)

[frame]
fg_color = \"#1a1e24\" 

# Run \"pino -f\" to show all available fonts
font_family = \"Fira Code\"

[border]
weight = 4 
color = \"#ffffff\"
radius = 8

[title]
color = \"#c5c6c8\"
font_size = 19
x = 4
y = 10

[message]
color = \"#626977\"
font_size = 15
x = 10
y = 45

[pywal]
pywal = false

# You can choose colors from colors[0-15] or use these values: \"bg\", \"fg\"
# Check ~/.cache/wal/colors-pino.toml file
background_color  = \"bg\"
border_color      = \"color1\"
title_color       = \"fg\"
message_color     = \"color8\"

",
        )
        .expect("Can't Create Template File !!!");
}
