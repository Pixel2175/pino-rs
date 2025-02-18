use fltk::{
    app::{self, font_size, get_font_names},
    draw::{self, draw_text2},
    enums::{Align, Color, Event, Font},
    prelude::{WidgetBase, WidgetExt, WindowExt},
    window,
};

use draw::{draw_rounded_rectf, set_draw_color, set_font};

use fltk::draw::measure;

fn draw_wrapped_text(text: &str, x: i32, mut y: i32) {
    for line in text.split("\\n") {
        let (_, height) = measure(line, false);
        draw_text2(line, x.into(), y.into(), 0, 0, Align::Left);
        y += height;
    }
}

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
    args: (String, String, i32),
) {
    let app = app::App::default();
    let mut wind = window::Window::new(
        screen_info.0,
        screen_info.1,
        screen_info.2,
        screen_info.3,
        "Pino",
    );
    wind.set_color(Color::from_hex_str(&colors.1.as_str()).expect("Can't set border color!!!"));
    wind.set_override();

    wind.draw(move |w| {
        set_draw_color(Color::from_hex_str(&colors.0).expect(""));
        draw_rounded_rectf(
            border.0 - 1,
            border.0 - 1,
            w.width() - (border.0 - 1) * 2,
            w.height() - (border.0 - 1) * 2,
            border.1,
        );

        set_draw_color(Color::from_hex_str(&colors.2.as_str()).expect("ppp"));
        set_font(Font::by_name(font_family.as_str()), title.2.into());
        draw_wrapped_text(
            args.0.as_str(),
            title.0 + border.0 + 4,
            title.1 + border.0 + font_size() -4
        );

        set_draw_color(Color::from_hex_str(&colors.3.as_str()).expect("ppp"));
        set_font(Font::by_name(font_family.as_str()), message.2.into());
        draw_wrapped_text(
            args.1.as_str(),
            message.0 + border.0  + 4,
            message.1 + border.0  + font_size() -4,
        );
    });

    wind.handle(|_, event| {
        if event == Event::Push {
            app::quit(); // This will close the window automatically
            true
        } else {
            false
        }
    });

    wind.show();

    app::add_timeout3(args.2.into(), move |_| {
        wind.hide();
        app::quit();
    });
    app.run().unwrap();
}
