use crate::{
    input::Input,
    item::Item,
    ui,
    wasm4::{rect, DRAW_COLORS},
};

pub struct TextScreen {
    pub found_item: Item,
}

impl TextScreen {
    pub fn update(&self, input: &Input) -> bool {
        input.pressed_any()
    }

    pub fn draw(&self) {
        unsafe { *DRAW_COLORS = 0x21 };
        rect(10, 40, 140, 110);

        ui::draw_text_top_center("New item:", 80, 54);
        ui::draw_item(self.found_item, 80 - (ui::ITEM_WIDTH_PX as i32 / 2), 68);
        ui::draw_text_top_center(self.found_item.name(), 80, 96);
        ui::draw_text_top_center("[Press any key\nto continue]", 80, 120);
    }
}
