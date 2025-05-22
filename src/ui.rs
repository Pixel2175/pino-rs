use fltk::{
    app::{self, get_font_names},
    draw,
    enums::{Align, Color, Event, Font},
    frame::Frame,
    prelude::*,
    window::Window,
};
use std::{
    io::Read,
    os::unix::net::UnixListener,
    path::Path,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub fn print_fonts() {
    for font in get_font_names() {
        println!("{font}");
    }
}

pub fn ui(
    screen_info: (i32, i32, i32, i32),
    font_family: String,
    border: (i32, i32),
    title: (i32, i32, i32),
    message: (i32, i32, i32),
    colors: (String, String, String, String),
    data: (String, String, u64),
    socket_path: String,
) {
    let app = app::App::default().load_system_fonts();

    let mut wind1 = Window::new(
        screen_info.0,
        screen_info.1,
        screen_info.2,
        screen_info.3,
        "Pino",
    );
    wind1.set_color(Color::from_hex_str(colors.1.as_str()).unwrap());

    let mut wind2 = Window::new(
        border.0 + border.1 / 2,
        border.0 + border.1 / 2,
        screen_info.2 - border.0 * 2 - border.1,
        screen_info.3 - border.0 * 2 - border.1,
        "Pino",
    );
    wind2.set_color(Color::from_hex_str(colors.0.as_str()).unwrap());

    let mut title_frame = Frame::new(
        5 + title.0,
        20 + title.1,
        wind2.w() - 10,
        wind2.h() - 10,
        "",
    );
    let mut message_frame = Frame::new(
        5 + message.0,
        20 + message.1,
        wind2.w() - 10,
        wind2.h() - 10,
        "",
    );

    title_frame.set_align(Align::Top | Align::Left);
    title_frame.set_label_color(Color::from_hex_str(colors.2.as_str()).unwrap());
    title_frame.set_label_font(Font::by_name(font_family.as_str()));
    title_frame.set_label_size(title.2);

    message_frame.set_align(Align::Top | Align::Left);
    message_frame.set_label_color(Color::from_hex_str(colors.3.as_str()).unwrap());
    message_frame.set_label_font(Font::by_name(font_family.as_str()));
    message_frame.set_label_size(message.2);

    wind1.draw(move |f| {
        draw::set_draw_color(Color::from_hex_str(colors.0.as_str()).unwrap());
        draw::draw_rounded_rectf(
            border.0 - 1,
            border.0 - 1,
            f.w() - border.0 * 2 + 2,
            f.h() - border.0 * 2 + 2,
            border.1,
        );
    });

    wind1.handle(|_, event| {
        if event == Event::Push {
            app::quit();
            true
        } else {
            false
        }
    });

    let (tx, rx) = mpsc::channel::<(String, String, u64)>();
    tx.send((data.0, data.1, data.2)).unwrap();

    if Path::new(&socket_path).exists() {
        std::fs::remove_file(&socket_path).unwrap();
    }

    let socket = socket_path.clone();
    thread::spawn(move || {
        let listener = UnixListener::bind(&socket).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    let n = stream.read(&mut buffer).unwrap();
                    let received = String::from_utf8_lossy(&buffer[..n]).to_string();

                    let parts: Vec<String> = received
                        .split("|+|")
                        .map(|item| item.replace('\n', ""))
                        .collect();
                    if parts.len() >= 3 {
                        let title = parts[0].to_string();
                        let message = parts[1].to_string();
                        let delay = parts[2].parse::<u64>().unwrap_or_else(|_| {
                            eprintln!("Invalid delay value, using 3s");
                            3
                        });
                        tx.send((title, message, delay)).unwrap();
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                    break;
                }
            }
        }
    });

    wind1.set_override();
    wind1.show();
    wind2.show();

    let mut close_timer: Option<Instant> = None;
    let mut current_delay_secs: Option<u64> = None;

    while app.wait() {
        if let Ok((title, message, delay)) = rx.try_recv() {
            if title_frame.label() != title {
                title_frame.set_label(&title);
            }
            if message_frame.label() != message {
                message_frame.set_label(&message);
            }
            wind2.redraw();

            if current_delay_secs.map(|d| d != delay).unwrap_or(true) {
                current_delay_secs = Some(delay);
                close_timer = Some(Instant::now() + Duration::from_secs(delay));
            }
        }

        if let Some(timer) = close_timer {
            if Instant::now() >= timer {
                app::quit();
                break;
            }
        }
    }
    if Path::new(&socket_path).exists() {
        std::fs::remove_file(socket_path).unwrap();
    }
}
