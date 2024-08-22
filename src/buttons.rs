use macroquad::{
    prelude::*,

    ui::{
        root_ui,
        Skin,
        widgets::{self, Button}
    },
};

use crate::WIN_W;

//button's bad implementation offset
const BUTTON_SIZE: Vec2 = vec2(100., 120.);

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
        let init_vec = Vec2::new(WIN_W - 300., 105.);

        Buttons {
            num_0: widgets::Button::new("0")
                .position(init_vec + BUTTON_SIZE * vec2(1., 3.))
                .size(BUTTON_SIZE),
            num_1: widgets::Button::new("1")
                .position(init_vec + BUTTON_SIZE * vec2(0., 0.))
                .size(BUTTON_SIZE),
            num_2: widgets::Button::new("2")
                .position(init_vec + BUTTON_SIZE * vec2(1., 0.))
                .size(BUTTON_SIZE),
            num_3: widgets::Button::new("3")
                .position(init_vec + BUTTON_SIZE * vec2(2., 0.))
                .size(BUTTON_SIZE),
            num_4: widgets::Button::new("4")
                .position(init_vec + BUTTON_SIZE * vec2(0., 1.))
                .size(BUTTON_SIZE),
            num_5: widgets::Button::new("5")
                .position(init_vec + BUTTON_SIZE * vec2(1., 1.))
                .size(BUTTON_SIZE),
            num_6: widgets::Button::new("6")
                .position(init_vec + BUTTON_SIZE * vec2(2., 1.))
                .size(BUTTON_SIZE),
            num_7: widgets::Button::new("7")
                .position(init_vec + BUTTON_SIZE * vec2(0., 2.))
                .size(BUTTON_SIZE),
            num_8: widgets::Button::new("8")
                .position(init_vec + BUTTON_SIZE * vec2(1., 2.))
                .size(BUTTON_SIZE),
            num_9: widgets::Button::new("9")
                .position(init_vec + BUTTON_SIZE * vec2(2., 2.))
                .size(BUTTON_SIZE),
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