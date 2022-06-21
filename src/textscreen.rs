use crate::{
    item::Item,
    ui,
    wasm4::{rect, DRAW_COLORS},
};

pub enum TextScreen {
    Intro,
    Found(Item),
    Win,
}

impl TextScreen {
    pub fn update(&self, pressed: u8) -> bool {
        match self {
            TextScreen::Intro | TextScreen::Found(_) => pressed != 0,
            TextScreen::Win => false,
        }
    }

    pub fn draw(&self) {
        match self {
            TextScreen::Intro => {
                ui::clear();
                ui::draw_text_top_center("Find the\nDogChicken", 80, 8);

                ui::draw_item(Item::DogChicken, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 32);

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
                ui::draw_item(Item::DogChicken, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 64);
                ui::draw_text_top_center("Thanks for\nPlaying", 80, 120);
            }
            TextScreen::Found(item) => {
                unsafe { *DRAW_COLORS = 0x21 };
                rect(10, 40, 140, 110);

                ui::draw_text_top_center("New item:", 80, 54);
                ui::draw_item(*item, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 68);
                ui::draw_text_top_center(item.name(), 80, 96);
                ui::draw_text_top_center("[Press any key\nto continue]", 80, 120);
            }
        }
    }
}
