use super::{ingame::IngameScreen, Screen};
use crate::{input::Input, item::Item, ui, wasm4::BUTTON_1, wasm4::BUTTON_2};

pub struct IntroScreen;

impl Screen for IntroScreen {
    fn update(&mut self, input: &Input) -> Option<Box<dyn Screen + Send>> {
        if input.pressed(BUTTON_1) || input.pressed(BUTTON_2) {
            return Some(Box::new(IngameScreen::new()));
        }
        None
    }

    fn draw(&self) {
        ui::draw_text_top_center("Find the\nDogChicken", 80, 8);

        ui::draw_item(Item::DogChicken, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 32);

        ui::draw_text_top_center(
            "Controls:\nZ: Select left\nX: Select right\n<>^v: Move cursor",
            80,
            80,
        );

        ui::draw_text_top_center("[Press Z/X\nto start]", 80, 140);
    }
}
