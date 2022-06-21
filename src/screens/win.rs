use super::Screen;
use crate::{input::Input, item::Item, ui};

pub struct WinScreen;

impl Screen for WinScreen {
    fn update(&mut self, _input: &Input) -> Option<Box<dyn Screen + Send>> {
        None
    }

    fn draw(&self) {
        ui::draw_text_top_center("YOU FOUND\nTHE DOGCHICKEN!", 80, 32);
        ui::draw_item(Item::DogChicken, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 64);
        ui::draw_text_top_center("Thanks for\nPlaying", 80, 120);
    }
}
