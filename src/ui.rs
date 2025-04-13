use fltk::{
    app::{self,get_font_names},
    draw,
    enums::{Align, Color, Event, Font},
    frame::Frame,
    prelude::*,
    window::Window,
};
use std::{
    fs,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};


pub fn print_fonts() {
    for font in get_font_names() {
        println!("{}", font);
    }
}




pub fn ui(
    screen_info: (i32, i32, i32, i32),
    font_family: String,
    border: (i32, i32),
    title: (i32,i32,i32),
    message: (i32,i32,i32),
    colors: (String, String, String, String),
) {
    let app = app::App::default();
    
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

    let mut title_frame =   Frame::new(5 + title.0, 20 + title.1, wind2.w() - 10, wind2.h()-10, "");
    let mut message_frame = Frame::new(5 + message.0, 20 + message.1, wind2.w() - 10, wind2.h()-10, "");

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
        draw::draw_rounded_rectf(border.0 - 1, border.0 -1 , f.w() - border.0 *2 +2, f.h() - border.0 * 2 +2, border.1);
    });

    wind1.handle(|_, event| {
        if event == Event::Push {
            app::quit();
            true
        } else {
            false
        }
    });

    let (tx, rx) = mpsc::channel::<(String, String, Option<u64>)>();
    thread::spawn(move || {
        let filename = "/tmp/pino-check";
        let mut current_title = String::new();
        let mut current_message = String::new();
        let mut current_delay = None;

        loop {
            let content = fs::read_to_string(filename).unwrap_or_default();
            let lines: Vec<&str> = content.lines().collect();
            let new_title = lines.first().unwrap_or(&"").to_string();
            let new_message = lines.get(1).unwrap_or(&"").to_string();
            let new_delay = lines.get(2).and_then(|s| s.parse::<u64>().ok());
            if new_title != current_title || new_message != current_message || new_delay != current_delay {
                tx.send((new_title.clone(), new_message.clone(), new_delay)).unwrap();
                current_title = new_title;
                current_message = new_message;
                current_delay = new_delay;
            }
            
            thread::sleep(Duration::from_millis(200));
        }
    });

    let mut close_timer: Option<Instant> = None;
    let mut current_delay_secs: Option<u64> = None;

    wind1.set_override();
    wind1.show();

    while app.wait() {
        if let Ok((title, message, delay)) = rx.try_recv() {
            if title_frame.label() != title {
                title_frame.set_label(&title);
            }
            if message_frame.label() != message {
                message_frame.set_label(&message);
            }
            wind2.redraw();

            if delay != current_delay_secs {
                current_delay_secs = delay;
                close_timer = delay.map(|secs| Instant::now() + Duration::from_secs(secs));
            }
        }

        if let Some(timer) = close_timer {
            if Instant::now() >= timer {
                app::quit();
                break;
            }
        }
    }
}
