use crate::item::{self, draw_item, Item};

pub struct Inventory {
    found: [bool; 128],
    found_ui: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        let base_items = vec![
            Item::Water,
            Item::Tree,
            Item::IronOre,
            Item::Ball,
            Item::Fire,
            Item::Cloth,
            Item::Cow,
            Item::Bottle,
            Item::Comb,
            Item::Plate,
            Item::Spring,
            Item::Bush,
        ];

        let mut found = [false; 128];
        base_items.iter().for_each(|item| {
            found[*item as usize] = true;
        });

        Self {
            found,
            found_ui: base_items,
        }
    }

    pub fn draw(&self) {
        let start_x = 12u32;
        let start_y = 52u32;
        let item_gap = 4u32;
        let each_row_count = 5u32;

        self.found_ui.iter().enumerate().for_each(|(idx, item)| {
            let x = (start_x
                + ((idx as u32 % each_row_count) * (item::SINGLE_OBJ_PIXELS + item_gap)))
                as i32;
            let y = (start_y
                + ((idx as u32 / each_row_count) * (item::SINGLE_OBJ_PIXELS + item_gap)))
                as i32;
            draw_item(*item, x, y);
        });
    }
}
