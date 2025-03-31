use std::{path::PathBuf,fs::{create_dir_all,OpenOptions},io::Write};


pub fn generate_config(config_path: PathBuf){
    let config = config_path
        .join("pino")
        .join("config.toml");

    create_dir_all(config.parent().unwrap())
    .expect("Failed to create directories for template");
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
# Set the monitor using index
monitor = 0

# App placement on the screen
horizontal = \"left\"
vertical = \"top\"
x = 25
y = 55

# Width and height of the app
width = 300
height = 100

# Time to display the app (in seconds)
delay = 5

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

")
.expect("Can't Create Template File !!!");
}
