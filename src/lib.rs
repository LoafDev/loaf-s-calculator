use macroquad::{
    prelude::*, ui::root_ui
};

mod buttons;
use buttons::{
    ButtonSkin,
    Buttons
};

//screen's width and height
pub const WIN_W: f32 = 800.;
pub const WIN_H: f32 = 600.;

pub struct App {
    pub number: String,
    pub exit: bool
}

impl Default for App {
    fn default() -> Self {
        App {
            number: String::new(),
            exit: false
        }
    }
}

pub fn update(app: &mut App) {
    let buttons = Buttons::default();
    let button_skin = ButtonSkin::default();

    root_ui().push_skin(&button_skin.skin1);

    //first row
    if buttons.num_0.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key0) {app.number.push_str("0");}
    if buttons.num_1.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key1) {app.number.push_str("1");}
    if buttons.num_2.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key2) {app.number.push_str("2");}
    if buttons.num_3.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key3) {app.number.push_str("3");}
    if buttons.num_4.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key4) {app.number.push_str("4");}

    //second row
    if buttons.num_5.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key5) {app.number.push_str("5");}
    if buttons.num_6.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key6) {app.number.push_str("6");}
    if buttons.num_7.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key7) {app.number.push_str("7");}
    if buttons.num_8.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key8) {app.number.push_str("8");}
    if buttons.num_9.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Key9) {app.number.push_str("9");}

    root_ui().pop_skin();

    //other function buttons
    root_ui().push_skin(&button_skin.skin2);

    if buttons.del.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Backspace) {app.number.pop();}
    if buttons.ac.ui(&mut *root_ui()) || is_key_pressed(KeyCode::Delete) {app.number.clear();}

    root_ui().pop_skin();
}

pub fn draw(app: &App) {
    let mut text_pos = vec2((WIN_W / 2.) -40. -(app.number.len() as f32 * 10.), 50.);
    if text_pos.x <= 155. {text_pos.x = 155.;}

    draw_line(0., 100., WIN_W, 100., 5., BLUE);
    draw_text(&app.number, text_pos.x, text_pos.y, 50., BLUE);

    draw_line(65., 0., 65., 100., 3., BLUE);
    draw_line(150., 0., 150., 100., 3., BLUE);
}