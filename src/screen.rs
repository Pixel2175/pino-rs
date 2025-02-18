use fltk::app;

pub fn get_size(
    monitor: usize,
    v: &str,
    h: &str,
    ax: i32,
    ay: i32,
    aw: i32,
    ah: i32,
) -> (i32, i32, i32, i32) {
    let mut screens = vec![];

    for screen in 0..app::screen_count() {
        screens.push(app::screen_xywh(screen));
    }

    if h.to_lowercase() == "left" {
        if v.to_lowercase() == "top" {
            return (screens[monitor].0 + ax, ay + screens[monitor].1, aw, ah);
        } else if v.to_lowercase() == "bottom" {
            return (
                screens[monitor].0 + ax,
                screens[monitor].1 + screens[monitor].3 - ah - ay,
                aw,
                ah,
            );
        }
    } else if h.to_lowercase() == "right" {
        if v.to_lowercase() == "top" {
            return (
                screens[monitor].0 + screens[monitor].2 - aw - ax,
                screens[monitor].1 + ay,
                aw,
                ah,
            );
        } else if v.to_lowercase() == "bottom" {
            return (
                screens[monitor].0 + screens[monitor].2 - aw - ax,
                screens[monitor].1 + screens[monitor].3 - ah - ay,
                aw,
                ah,
            );
        }
    }
    (0, 0, 100, 100)
}
