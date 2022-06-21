use crate::{
    item::{self, Item, SINGLE_OBJ_PIXELS},
    ui,
};

pub enum TextScreen {
    Intro,
    Win,
}

impl TextScreen {
    pub fn update(&self, pressed: u8) -> bool {
        match self {
            TextScreen::Intro => pressed != 0,
            TextScreen::Win => false,
        }
    }

    pub fn draw(&self) {
        match self {
            TextScreen::Intro => {
                ui::clear();
                ui::draw_text_top_center("Find the\nDogChicken", 80, 8);

                item::draw_item(Item::DogChicken, 80 - (SINGLE_OBJ_PIXELS as i32 / 2), 32);

                ui::draw_text_top_center(
                    "Controls:\nZ: Select left\nX: Select right\n<>^v: Move cursor",
                    80,
                    80,
                );

                ui::draw_text_top_center("[Press any key\nto start]", 80, 140);
            }
            TextScreen::Win => {
                ui::clear();
                ui::draw_text_top_center("YOU FOUND\nTHE DOGCHICKEN!", 80, 32);
                item::draw_item(Item::DogChicken, 80 - (SINGLE_OBJ_PIXELS as i32 / 2), 64);
                ui::draw_text_top_center("Thanks for\nPlaying", 80, 120);
            }
        }
    }
}
