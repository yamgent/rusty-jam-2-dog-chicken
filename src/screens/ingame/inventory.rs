use crate::input::Input;
use crate::wasm4::{rect, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, DRAW_COLORS};
use crate::{
    item::{Item, STARTING_ITEMS},
    ui,
};

const PAGE_COL_COUNT: usize = 5;
const PAGE_ROW_COUNT: usize = 3;
const PAGE_COUNT: usize = PAGE_COL_COUNT * PAGE_ROW_COUNT;

const PAGE_ITEM_START_X: u32 = 12;
const PAGE_ITEM_START_Y: u32 = 52;
const PAGE_ITEM_GAP: u32 = 4;

pub struct Inventory {
    found: Vec<Item>,
    select_idx: usize,
}

#[derive(PartialEq, Debug)]
pub enum AddResult {
    Success,
    AlreadyFound,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            found: STARTING_ITEMS.clone(),
            select_idx: 0,
        }
    }

    pub fn selected_item(&self) -> Item {
        self.found[self.select_idx]
    }

    fn current_page(&self) -> usize {
        self.select_idx / PAGE_COUNT
    }

    fn total_pages(&self) -> usize {
        if self.found.is_empty() {
            0
        } else {
            ((self.found.len() - 1) / PAGE_COUNT) + 1
        }
    }

    pub fn add_found(&mut self, new_item: Item) -> AddResult {
        if self.found.contains(&new_item) {
            AddResult::AlreadyFound
        } else {
            self.found.push(new_item);
            AddResult::Success
        }
    }

    pub fn update(&mut self, input: &Input) {
        if input.pressed(BUTTON_RIGHT) {
            let new_idx = self.select_idx + 1;

            let old_row = self.select_idx / PAGE_COL_COUNT;
            let new_row = (self.select_idx + 1) / PAGE_COL_COUNT;

            if old_row == new_row {
                if new_idx < self.found.len() {
                    self.select_idx = new_idx;
                }
            } else if self.current_page() + 1 < self.total_pages() {
                let new_idx = self.select_idx + PAGE_COUNT - (PAGE_COL_COUNT - 1);

                if new_idx < self.found.len() {
                    self.select_idx = new_idx;
                } else {
                    // just go to the next page and select the first item
                    self.select_idx = (self.current_page() + 1) * PAGE_COUNT;
                }
            }
        } else if input.pressed(BUTTON_LEFT) {
            if self.select_idx > 0 {
                let new_idx = self.select_idx - 1;

                let old_row = self.select_idx / PAGE_COL_COUNT;
                let new_row = new_idx / PAGE_COL_COUNT;

                if old_row == new_row {
                    self.select_idx = new_idx;
                } else if self.current_page() != 0 {
                    self.select_idx -= PAGE_COUNT - (PAGE_COL_COUNT - 1);
                }
            }
        } else if input.pressed(BUTTON_UP) {
            if self.select_idx >= PAGE_COL_COUNT {
                let new_idx = self.select_idx - PAGE_COL_COUNT;
                let old_page = self.current_page();
                let new_page = new_idx / PAGE_COUNT;

                if old_page == new_page {
                    self.select_idx = new_idx;
                }
            }
        } else if input.pressed(BUTTON_DOWN) {
            let new_idx = self.select_idx + PAGE_COL_COUNT;
            if new_idx < self.found.len() {
                let old_page = self.current_page();
                let new_page = new_idx / PAGE_COUNT;

                if old_page == new_page {
                    self.select_idx = new_idx;
                }
            }
        }
    }

    pub fn draw(&self) {
        let current_page = self.current_page();
        let total_pages = self.total_pages();

        // items
        let first_item_idx = current_page * PAGE_COUNT;
        self.found
            .iter()
            .skip(first_item_idx)
            .take(PAGE_COUNT)
            .enumerate()
            .for_each(|(rel_idx, item)| {
                let x = (PAGE_ITEM_START_X
                    + ((rel_idx % PAGE_COL_COUNT) as u32 * (ui::ITEM_WIDTH_PX + PAGE_ITEM_GAP)))
                    as i32;
                let y = (PAGE_ITEM_START_Y
                    + ((rel_idx / PAGE_COL_COUNT) as u32 * (ui::ITEM_HEIGHT_PX + PAGE_ITEM_GAP)))
                    as i32;
                ui::draw_item(*item, x, y);

                if first_item_idx + rel_idx == self.select_idx {
                    unsafe { *DRAW_COLORS = 0x40 };
                    rect(x - 2, y - 2, ui::ITEM_WIDTH_PX + 4, ui::ITEM_HEIGHT_PX + 4);
                }
            });

        // page nav indicators
        if self.total_pages() > 1 {
            if self.current_page() < self.total_pages() - 1 {
                ui::draw_text(">\n>\n>\n>\n>\n>", 160 - 8, 70);

                unsafe { *DRAW_COLORS = 0x20 }
                rect(160 - 8 - 1, 70 - 2, 7 + 2, 49 + 2);
            }

            if self.current_page() > 0 {
                ui::draw_text("<\n<\n<\n<\n<\n<", 1, 70);

                unsafe { *DRAW_COLORS = 0x20 }
                rect(0, 70 - 2, 7 + 2, 49 + 2);
            }
        }

        // bottom text
        ui::draw_text_bottom_left(self.found[self.select_idx].name(), 2, 160 - 2);
        ui::draw_text_bottom_right(
            format!("Page\n{}/{}", current_page + 1, total_pages),
            160 - 2,
            160 - 2,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn test_selected_item() {
        let mut inv = Inventory {
            found: vec![Item::Dog, Item::Chicken, Item::DogChicken],
            select_idx: 0,
        };

        assert_eq!(inv.selected_item(), Item::Dog);

        inv.select_idx = 1;
        assert_eq!(inv.selected_item(), Item::Chicken);

        inv.select_idx = 2;
        assert_eq!(inv.selected_item(), Item::DogChicken);
    }

    #[test]
    fn test_current_page() {
        let mut inv = Inventory::new();
        inv.select_idx = 0;
        assert_eq!(0, inv.current_page());

        inv.select_idx = PAGE_COUNT - 1;
        assert_eq!(0, inv.current_page());

        inv.select_idx = PAGE_COUNT;
        assert_eq!(1, inv.current_page());
    }

    #[test]
    fn test_total_pages() {
        let mut inv = Inventory::new();
        inv.found.clear();
        assert_eq!(0, inv.total_pages());

        inv.found = vec![Item::Dog];
        assert_eq!(1, inv.total_pages());

        inv.found = iter::repeat(Item::Dog).take(PAGE_COUNT).collect::<Vec<_>>();
        assert_eq!(1, inv.total_pages());

        inv.found.push(Item::Dog);
        assert_eq!(2, inv.total_pages());
    }

    #[test]
    fn test_add_found() {
        let mut inv = Inventory::new();
        inv.found.clear();

        assert_eq!(inv.add_found(Item::Dog), AddResult::Success);
        assert_eq!(inv.found, vec![Item::Dog]);
        assert_eq!(inv.add_found(Item::Dog), AddResult::AlreadyFound);
        assert_eq!(inv.found, vec![Item::Dog]);
    }
}
