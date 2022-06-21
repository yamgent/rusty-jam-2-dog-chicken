use crate::input::Input;
use crate::wasm4::{rect, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, DRAW_COLORS};
use crate::{
    item::{Item, STARTING_ITEMS},
    ui,
};

const PAGE_COL_COUNT: u32 = 5;
const PAGE_ROW_COUNT: u32 = 3;
const PAGE_COUNT: u32 = PAGE_COL_COUNT * PAGE_ROW_COUNT;

pub struct Inventory {
    found: Vec<Item>,
    select_idx: u32,
}

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
        self.found[self.select_idx as usize]
    }

    fn current_page(&self) -> u32 {
        self.select_idx / PAGE_COUNT
    }

    fn total_pages(&self) -> u32 {
        if self.found.len() == 0 {
            0
        } else {
            (((self.found.len() as u32) - 1) / PAGE_COUNT) + 1
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
            let old_row = self.select_idx / PAGE_COL_COUNT;
            let new_row = (self.select_idx + 1) / PAGE_COL_COUNT;

            if old_row == new_row {
                if self.select_idx + 1 < self.found.len() as u32 {
                    self.select_idx += 1;
                }
            } else {
                let new_idx = self.select_idx + PAGE_COUNT - (PAGE_COL_COUNT - 1);
                if self.current_page() + 1 < self.total_pages() {
                    if new_idx < self.found.len() as u32 {
                        self.select_idx = new_idx;
                    } else {
                        self.select_idx = (self.current_page() + 1) * PAGE_COUNT;
                    }
                }
            }
        } else if input.pressed(BUTTON_LEFT) {
            if self.select_idx > 0 {
                let old_row = self.select_idx / PAGE_COL_COUNT;
                let new_row = (self.select_idx - 1) / PAGE_COL_COUNT;

                if old_row == new_row {
                    if self.select_idx != 0 {
                        self.select_idx -= 1;
                    }
                } else {
                    if self.current_page() != 0 {
                        self.select_idx -= PAGE_COUNT - (PAGE_COL_COUNT - 1);
                    }
                }
            }
        } else if input.pressed(BUTTON_UP) {
            if self.select_idx >= PAGE_COL_COUNT {
                let old_page = self.current_page();
                let new_page = (self.select_idx - PAGE_COL_COUNT) / PAGE_COUNT;

                if old_page == new_page {
                    self.select_idx -= PAGE_COL_COUNT;
                }
            }
        } else if input.pressed(BUTTON_DOWN) {
            if self.select_idx + PAGE_COL_COUNT < self.found.len() as u32 {
                let old_page = self.current_page();
                let new_page = (self.select_idx + PAGE_COL_COUNT) / PAGE_COUNT;

                if old_page == new_page {
                    self.select_idx += PAGE_COL_COUNT;
                }
            }
        }
    }

    pub fn draw(&self) {
        let current_page = self.current_page();
        let total_pages = self.total_pages();
        // items
        let start_x = 12u32;
        let start_y = 52u32;
        let item_gap = 4u32;

        self.found
            .iter()
            .skip((current_page * PAGE_COUNT) as usize)
            .take(PAGE_COUNT as usize)
            .enumerate()
            .for_each(|(idx, item)| {
                let x = (start_x + ((idx as u32 % PAGE_COL_COUNT) * (ui::ITEM_WIDTH_PX + item_gap)))
                    as i32;
                let y = (start_y
                    + ((idx as u32 / PAGE_COL_COUNT) * (ui::ITEM_HEIGHT_PX + item_gap)))
                    as i32;
                ui::draw_item(*item, x, y);

                if (idx as u32 + (current_page * PAGE_COUNT)) == self.select_idx {
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
        ui::draw_text_bottom_left(self.found[self.select_idx as usize].name(), 2, 160 - 2);
        ui::draw_text_bottom_right(
            format!("Page\n{}/{}", current_page + 1, total_pages),
            160 - 2,
            160 - 2,
        );
    }
}
