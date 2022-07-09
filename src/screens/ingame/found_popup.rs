use crate::{
    input::Input,
    item::Item,
    ui,
    wasm4::{rect, BUTTON_1, BUTTON_2, DRAW_COLORS},
};

pub struct FoundPopup {
    found_item: Option<Item>,
}

impl FoundPopup {
    pub fn new() -> Self {
        Self { found_item: None }
    }

    pub fn will_consume_input(&self) -> bool {
        self.found_item.is_some()
    }

    pub fn update(&mut self, input: &Input) {
        if input.pressed(BUTTON_1) || input.pressed(BUTTON_2) {
            self.found_item = None;
        }
    }

    pub fn draw(&self) {
        if let Some(item) = self.found_item {
            unsafe { *DRAW_COLORS = 0x21 };
            rect(10, 40, 140, 110);

            ui::draw_text_top_center("New item:", 80, 54);
            ui::draw_item(item, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 68);
            ui::draw_text_top_center(item.name(), 80, 96);
            ui::draw_text_top_center("[Press Z/X\nto continue]", 80, 120);
        }
    }

    pub fn show(&mut self, item: Item) {
        self.found_item = Some(item);
    }
}
