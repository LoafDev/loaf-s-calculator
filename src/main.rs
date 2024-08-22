use macroquad::prelude::{
    clear_background,
    next_frame,
    WHITE,
    is_key_down,
    KeyCode,
    Conf
};

use loaf_cal::{
    App,
    update,
    draw,
    WIN_H,
    WIN_W
};

fn init_win() -> Conf {
    Conf {
        window_height: WIN_H as i32,
        window_width: WIN_W as i32,
        window_resizable: false,
        fullscreen: false,
        window_title: "Loaf's useful calculator".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(init_win)]
async  fn main() {
    let mut app = App::default();

    loop {
        clear_background(WHITE);
        
        if app.exit || is_key_down(KeyCode::Escape) {break;}

        update(&mut app);
        draw(&app);

        next_frame().await;
    }
}