use crate::{
    input::Input,
    item::{self, Item},
    ui,
    wasm4::{rect, BUTTON_1, BUTTON_2, DRAW_COLORS},
};

#[derive(Clone, Copy)]
pub struct ComboResult {
    pub valid_item: Option<Item>,
}

pub struct Combine {
    left: Option<Item>,
    right: Option<Item>,
    result: Option<ComboResult>,
}

fn draw_combo_box(item: &Option<Item>, x: i32, y: i32) {
    let color = match item {
        Some(_) => 0x22,
        None => 0x30,
    };

    unsafe { *DRAW_COLORS = color }
    rect(x - 2, y - 2, ui::ITEM_WIDTH_PX + 4, ui::ITEM_HEIGHT_PX + 4);
}

fn draw_result_box(result: &Option<ComboResult>, x: i32, y: i32) {
    let color = match result {
        Some(result) => match result.valid_item {
            Some(_) => 0x22,
            None => 0x40,
        },
        None => 0x30,
    };

    unsafe { *DRAW_COLORS = color }
    rect(x - 2, y - 2, ui::ITEM_WIDTH_PX + 4, ui::ITEM_HEIGHT_PX + 4);
}

impl Combine {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            result: None,
        }
    }

    pub fn update(&mut self, input: &Input, selected_item: Item) -> Option<ComboResult> {
        let updated = if input.pressed(BUTTON_2) {
            self.left = Some(selected_item);
            true
        } else if input.pressed(BUTTON_1) {
            self.right = Some(selected_item);
            true
        } else {
            false
        };

        if updated {
            if let (Some(left), Some(right)) = (self.left, self.right) {
                self.result = match item::combine_items(&item::ITEM_RECIPES, &left, &right) {
                    Some(valid_item) => Some(ComboResult {
                        valid_item: Some(valid_item),
                    }),
                    None => Some(ComboResult { valid_item: None }),
                };
                return self.result;
            }
        }

        None
    }

    pub fn draw(&self) {
        draw_combo_box(&self.left, 18, 16);
        if let Some(left) = self.left {
            ui::draw_item(left, 18, 16);
        }

        ui::draw_text("+", 50, 16 + 8);

        draw_combo_box(&self.right, 66, 16);
        if let Some(right) = self.right {
            ui::draw_item(right, 66, 16);
        }

        ui::draw_text("=", 100, 16 + 8);

        draw_result_box(&self.result, 116, 16);
        if let Some(result) = self.result {
            match result.valid_item {
                Some(result_item) => {
                    ui::draw_item(result_item, 116, 16);
                }
                None => {
                    ui::draw_text("X", 124, 24);
                }
            }
        }
    }
}
