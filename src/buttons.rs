use macroquad::{
    prelude::*,

    ui::{
        root_ui,
        Skin,
        widgets::{self, Button}
    },
};

use crate::WIN_H;

//button's bad implementation offset
pub const FIRST: f32 = 170.;
pub const SECOND: f32 = 340.;
pub const THIRD: f32 = 510.;
pub const FOURTH: f32 = 680.;

pub struct Buttons<'a> {
    pub num_0: Button<'a>,
    pub num_1: Button<'a>,
    pub num_2: Button<'a>,
    pub num_3: Button<'a>,
    pub num_4: Button<'a>,
    pub num_5: Button<'a>,
    pub num_6: Button<'a>,
    pub num_7: Button<'a>,
    pub num_8: Button<'a>,
    pub num_9: Button<'a>,
    pub del: Button<'a>,
    pub ac: Button<'a>
}

pub struct ButtonSkin {
    pub skin1: Skin,
    pub skin2: Skin
}

impl Default for Buttons<'_> {
    fn default() -> Self {
        let init_vec = Vec2::new(10., WIN_H - 200.);
        let button_offset = -250.;

        Buttons {
            num_0: widgets::Button::new("0").position(init_vec),
            num_1: widgets::Button::new("1").position(vec2(init_vec.x + FIRST, init_vec.y)),
            num_2: widgets::Button::new("2").position(vec2(init_vec.x + SECOND, init_vec.y)),
            num_3: widgets::Button::new("3").position(vec2(init_vec.x + THIRD, init_vec.y)),
            num_4: widgets::Button::new("4").position(vec2(init_vec.x + FOURTH, init_vec.y)),
            num_5: widgets::Button::new("5").position(vec2(init_vec.x, init_vec.y + button_offset)),
            num_6: widgets::Button::new("6").position(vec2(init_vec.x + FIRST, init_vec.y + button_offset)),
            num_7: widgets::Button::new("7").position(vec2(init_vec.x + SECOND, init_vec.y + button_offset)),
            num_8: widgets::Button::new("8").position(vec2(init_vec.x + THIRD, init_vec.y + button_offset),),
            num_9: widgets::Button::new("9").position(vec2(init_vec.x + FOURTH, init_vec.y + button_offset)),
            del: widgets::Button::new("D").position(vec2(20., 18.)),
            ac: widgets::Button::new("AC").position(vec2(80., 18.))
        }
    }
}

impl Default for ButtonSkin {
    fn default() -> Self {
        let skin = {
            let button_style = root_ui()
            .style_builder()
            .text_color(BLUE)
            .color_hovered(GREEN)
            .color_clicked(WHITE)
            .text_color_clicked(YELLOW)
            .text_color_hovered(WHITE)
            .font_size(200)
            .build();
    
            Skin {
                button_style,
                ..root_ui().default_skin()
            }
        };
    
        let skin_2 = {
            let button_style = root_ui()
            .style_builder()
            .text_color(GREEN)
            .color_hovered(BLUE)
            .color_clicked(BLACK)
            .text_color_clicked(WHITE)
            .text_color_hovered(YELLOW)
            .font_size(70)
            .build();
    
            Skin {
                button_style,
                ..root_ui().default_skin()
            }
        };

        ButtonSkin {
            skin1: skin,
            skin2: skin_2,
        }
    }
}