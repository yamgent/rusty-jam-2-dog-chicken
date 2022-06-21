use strum::IntoEnumIterator;

use super::Screen;
use crate::{
    input::Input,
    item::Item,
    ui::{self, ITEM_HEIGHT_PX, ITEM_WIDTH_PX},
    wasm4::{BUTTON_DOWN, BUTTON_UP},
};

pub struct DebugGalleryScreen {
    all_items: Vec<Item>,
    current_page: usize,
    total_pages: usize,
}

const PAGE_ITEMS_COUNT: usize = 5;

impl DebugGalleryScreen {
    pub fn new() -> Self {
        let all_items = Item::iter().collect::<Vec<_>>();

        Self {
            total_pages: (all_items.len() / PAGE_ITEMS_COUNT)
                + if all_items.len() % PAGE_ITEMS_COUNT == 0 {
                    0
                } else {
                    1
                },
            all_items,
            current_page: 0,
        }
    }
}

impl Screen for DebugGalleryScreen {
    fn update(&mut self, input: &Input) -> Option<Box<dyn Screen + Send>> {
        if input.pressed(BUTTON_DOWN) && self.current_page + 1 < self.total_pages {
            self.current_page += 1;
        }

        if input.pressed(BUTTON_UP) && self.current_page != 0 {
            self.current_page -= 1;
        }
        None
    }

    fn draw(&self) {
        ui::draw_text_top_center(
            format!(
                "Debug Gallery ({}/{})",
                self.current_page + 1,
                self.total_pages
            ),
            80,
            0,
        );

        let start_idx = self.current_page * PAGE_ITEMS_COUNT;

        self.all_items
            .iter()
            .skip(start_idx)
            .take(PAGE_ITEMS_COUNT)
            .enumerate()
            .for_each(|(idx, item)| {
                let x = 4;
                let y = 12 + idx as i32 * (ITEM_HEIGHT_PX as i32 + 4);

                ui::draw_item(*item, x, y);
                ui::draw_text(
                    item.name(),
                    x + ITEM_WIDTH_PX as i32 + 4,
                    y + (ITEM_HEIGHT_PX as i32 / 2) - 4,
                );
            });
    }
}
