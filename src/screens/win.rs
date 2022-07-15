use super::{ingame::IngameScreen, Screen};
use crate::{
    input::Input,
    item::Item,
    ui,
    wasm4::{BUTTON_1, BUTTON_2},
};
use strum::IntoEnumIterator;

pub struct WinScreen {
    found_items_count: usize,
    total_items_count: usize,
}

impl WinScreen {
    pub fn new(found_items_count: usize) -> Self {
        Self {
            found_items_count,
            total_items_count: Item::iter().count(),
        }
    }
}

impl Screen for WinScreen {
    fn update(&mut self, input: &Input) -> Option<Box<dyn Screen + Send>> {
        if input.pressed(BUTTON_1) && input.pressed(BUTTON_2) {
            return Some(Box::new(IngameScreen::new()));
        }
        None
    }

    fn draw(&self) {
        ui::draw_text_top_center("YOU FOUND\nTHE DOGCHICKEN!", 80, 32);
        ui::draw_item(Item::DogChicken, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 64);
        ui::draw_text_top_center("Thank you for\nplaying\n(Z+X to restart)", 80, 110);
        ui::draw_text_bottom_left(
            format!(
                "<{}/{} found>",
                self.found_items_count, self.total_items_count
            ),
            2,
            158,
        );
    }
}
