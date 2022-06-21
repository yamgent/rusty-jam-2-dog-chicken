use crate::{
    item::{self, Item, SINGLE_OBJ_PIXELS},
    ui,
    wasm4::{rect, DRAW_COLORS},
};

pub enum TextScreen {
    Intro,
}

impl TextScreen {
    pub fn update(&self, pressed: u8) -> bool {
        match self {
            TextScreen::Intro => pressed != 0,
        }
    }

    pub fn draw(&self) {
        match self {
            TextScreen::Intro => {
                // TODO: Refactor clear()
                unsafe { *DRAW_COLORS = 0x11 };
                rect(0, 0, 160, 160);

                ui::draw_text_top_center("Find the\nDogChicken", 80, 8);

                item::draw_item(Item::DogChicken, 80 - (SINGLE_OBJ_PIXELS as i32 / 2), 32);

                ui::draw_text_top_center(
                    "Controls:\nZ: Select left\nX: Select right\n<>^v: Move cursor",
                    80,
                    80,
                );

                ui::draw_text_top_center("[Press any key\nto start]", 80, 140);
            }
        }
    }
}
