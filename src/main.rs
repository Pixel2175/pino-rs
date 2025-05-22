use argh::FromArgs;
use fltk::app::screen_xywh;
use serde::Deserialize;
use std::{fs, io::Write, os::unix::net::UnixStream};

mod colors;
mod config;
mod ui;

#[derive(FromArgs)]
#[argh(
    description = "This tool lets you display notification with customizable options. you can also use a configuration file to set theme
and everything easily (conf path = ~/.config/pino)"
)]
struct Arg {
    #[argh(
        option,
        short = 't',
        description = "set the notification title content"
    )]
    title: Option<String>,

    #[argh(
        option,
        short = 'm',
        description = "set the notification message content"
    )]
    message: Option<String>,

    #[argh(
        option,
        short = 'd',
        description = "set the delay before program closes with secends"
    )]
    delay: Option<u64>,

    #[argh(
        option,
        short = 's',
        description = "choice a name for new session by a special number (default = 0)"
    )]
    session: Option<u8>,

    #[argh(
        switch,
        short = 'f',
        description = "print all the fonts that you can use it"
    )]
    font: bool,

    #[argh(option, short = 'c', description = "set a custom configuration file")]
    config: Option<String>,

    #[argh(switch, short = 'v', description = "set a custom configuration file")]
    version: bool,
}

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
    monitor: Option<i32>,
    placement: Option<String>,
    x: Option<i32>,
    y: Option<i32>,
    width: Option<i32>,
    height: Option<i32>,
    delay: Option<u64>,
}
#[derive(Debug, Deserialize)]
struct Frame {
    fg_color: Option<String>,
    font_family: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Border {
    weight: Option<i32>,
    color: Option<String>,
    radius: Option<i32>,
}
#[derive(Debug, Deserialize)]
struct Title {
    color: Option<String>,
    font_size: Option<i32>,
    x: Option<i32>,
    y: Option<i32>,
}
#[derive(Debug, Deserialize)]
struct Message {
    color: Option<String>,
    font_size: Option<i32>,
    x: Option<i32>,
    y: Option<i32>,
}
#[derive(Debug, Deserialize)]
struct Pywal {
    pywal: Option<bool>,
    background_color: Option<String>,
    border_color: Option<String>,
    title_color: Option<String>,
    message_color: Option<String>,
}

fn main() {
    let config_folder = colors::get_config_dir();
    let args: Arg = argh::from_env();

    if args.version {
        println!("v1.1.4");
        return;
    }

    if args.font {
        ui::print_fonts();
        return;
    }

    let config_file = match args.config {
        Some(path) => path,
        None => {
            if !config_folder.join("pino").exists() {
                config::generate_config(config_folder.clone());
            }
            config_folder
                .join("pino")
                .join("config.toml")
                .to_string_lossy()
                .into_owned()
        }
    };

    let config_content = fs::read_to_string(config_file).expect("Faild ");
    let config: Config = toml::from_str(&config_content).expect("Faild");

    let socket = format!("/tmp/pino-check-{}.sock", args.session.unwrap_or(0));
    if let Ok(mut stream) = UnixStream::connect(&socket) {
        stream
            .write_all(
                format!(
                    "{}|+|{}|+|{}",
                    args.title.unwrap_or("Title".to_string()),
                    args.message
                        .unwrap_or("you didn't set the title or message".to_string()),
                    args.delay.unwrap_or(config.screen.delay.unwrap_or(3))
                )
                .as_bytes(),
            )
            .unwrap()
    } else {
        let fallback = (
            config
                .frame
                .fg_color
                .clone()
                .unwrap_or("#000000".to_string()),
            config.border.color.clone().unwrap_or("#62777d".to_string()),
            config.title.color.clone().unwrap_or("#b8b8b8".to_string()),
            config
                .message
                .color
                .clone()
                .unwrap_or("#501701".to_string()),
        );

        let colors = if config.pywal.pywal.unwrap_or(false) {
            colors::pywal(
                config
                    .pywal
                    .background_color
                    .clone()
                    .unwrap_or_else(|| fallback.0.clone()),
                config
                    .pywal
                    .border_color
                    .clone()
                    .unwrap_or_else(|| fallback.1.clone()),
                config
                    .pywal
                    .title_color
                    .clone()
                    .unwrap_or_else(|| fallback.2.clone()),
                config
                    .pywal
                    .message_color
                    .clone()
                    .unwrap_or_else(|| fallback.3.clone()),
            )
        } else {
            fallback
        };

        let (sx, sy, sw, sh) = screen_xywh(config.screen.monitor.unwrap_or(0));
        let (ax, ay, aw, ah) = (
            config.screen.x.unwrap_or(25),
            config.screen.y.unwrap_or(55),
            config.screen.width.unwrap_or(400),
            config.screen.height.unwrap_or(60),
        );
        let screen = match config
            .screen
            .placement
            .unwrap_or("top_center".to_string())
            .as_str()
        {
            "top_left" => (sx + ax, sy + ay),
            "top_center" => (sx + (sw - aw) / 2, sy + ay),
            "top_right" => (sx + sw - aw - ax, sy + ay),
            "bottom_left" => (sx + ax, sy + sh - ah - ay),
            "bottom_center" => (sx + (sw - aw) / 2, sy + sh - ah - ay),
            "bottom_right" => (sx + sw - aw - ax, sy + sh - ah - ay),
            _ => (20, 30),
        };
        ui::ui(
            (screen.0, screen.1, aw, ah),
            config.frame.font_family.unwrap_or("Monospace".to_string()),
            (
                config.border.weight.unwrap_or(2),
                config.border.radius.unwrap_or(10),
            ),
            (
                config.title.x.unwrap_or(5),
                config.title.y.unwrap_or(0),
                config.title.font_size.unwrap_or(17),
            ),
            (
                config.message.x.unwrap_or(10),
                config.message.y.unwrap_or(19),
                config.message.font_size.unwrap_or(13),
            ),
            colors,
            (
                args.title.unwrap_or("Title".to_string()),
                args.message
                    .unwrap_or("you didn't set the title or message".to_string()),
                args.delay.unwrap_or(config.screen.delay.unwrap_or(5)),
            ),
            socket,
        );
    }
}
