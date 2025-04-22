use serde::Deserialize;
use std::{fs::{self, File}, io::Write};
use argh::FromArgs;
use utils::is_running;


mod utils;
mod colors;
mod config;
mod screen;
mod ui;


// Get Args
#[derive(FromArgs)]

#[argh(description = "This tool lets you display notification with customizable options. you can also use a configuration file to set theme
and everything easily (conf path = ~/.config/pino)")]
struct Arg {

    #[argh(option,short = 't', description = "set the notification title content")]
    title:Option<String> ,

    #[argh(option,short = 'm', description = "set the notification message content")]
    message:Option<String>,


    #[argh(option,short = 'd', description = "set the delay before program closes with secends")]
    delay:Option<i32> ,

    #[argh(switch,short = 'f', description = "print all the fonts that you can use it")]
    font:bool ,

    #[argh(option,short = 'c', description = "set a custom configuration file")]
    config:Option<String> ,


    #[argh(switch,short = 'v', description = "set a custom configuration file")]
    version:bool ,
}







// Read the Config File
#[derive(Debug, Deserialize)]
struct Config {
    screen: Screen,
    frame: Frame,
    border: Border,
    title: Title,
    message: Message,
    pywal: Pywal,
}

#[derive(Debug, Deserialize)]
struct Screen {
    monitor: usize,
    horizontal: String,
    vertical: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    delay: i32,
}
#[derive(Debug, Deserialize)]
struct Frame {
    fg_color: String,
    font_family: String,
}
#[derive(Debug, Deserialize)]
struct Border {
    weight: i32,
    color: String,
    radius: i32,
}
#[derive(Debug, Deserialize)]
struct Title {
    color: String,
    font_size: i32,
    x: i32,
    y: i32,
}
#[derive(Debug, Deserialize)]
struct Message {
    color: String,
    font_size: i32,
    x: i32,
    y: i32,
}
#[derive(Debug, Deserialize)]
struct Pywal {
    pywal: bool,
    background_color: String,
    border_color: String,
    title_color: String,
    message_color: String,
}



fn main() {
    let config_folder = colors::get_config_dir();
    let args:Arg = argh::from_env();

    if args.version {
        println!("v1.1.1");
        return ;
    }

    if args.font{
        ui::print_fonts();
        return ;
    }


    let config_file = match args.config {
        Some(path) => path,
        None => {
            if !config_folder.join("pino").exists() {
                config::generate_config(config_folder.clone());
            }
            config_folder.join("pino").join("config.toml").to_string_lossy().into_owned()
        }
    };

    let config_content = fs::read_to_string(config_file).expect("Faild ");
    let config: Config = toml::from_str(&config_content).expect("Faild");



    let mut pino_check = File::create("/tmp/pino-check").unwrap();

    writeln!(pino_check ,"{}",args.title.unwrap_or("Title".to_string())).unwrap();
    writeln!(pino_check ,"{}",args.message.unwrap_or("you didn't set the title or message".to_string())).unwrap();
    writeln!(pino_check ,"{}",args.delay.unwrap_or(config.screen.delay)).unwrap();


    if !is_running("pino"){

        let colors = match config.pywal.pywal {
            true => colors::pywal(
                config.pywal.background_color.to_string(),
                config.pywal.border_color.to_string(),
                config.pywal.title_color.to_string(),
                config.pywal.message_color.to_string()
            ),
            false => (
                config.frame.fg_color,
                config.border.color,
                config.title.color,
                config.message.color,
            ),

        };

        // Get Screen Placment
        let screen = screen::get_size(
            config.screen.monitor,
            config.screen.vertical.as_str(),
            config.screen.horizontal.as_str(),
            config.screen.x,
            config.screen.y,
            config.screen.width,
            config.screen.height,
        );

        //UI
        ui::ui(
            screen,config.frame.font_family,
            (
                config.border.weight,
                config.border.radius,
            ),
            (
                config.title.x,
                config.title.y,
                config.title.font_size,
            ),
            (
                config.message.x,
                config.message.y,
                config.message.font_size,
            ),colors
        );
    }
}


