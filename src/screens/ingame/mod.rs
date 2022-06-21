mod combine;
mod found_popup;
mod inventory;
mod status_bar;

use combine::{CombineUi, ComboResult};
use found_popup::FoundPopup;
use inventory::{AddResult, Inventory};
use status_bar::{Status, StatusBar};

use super::{win::WinScreen, Screen};
use crate::{input::Input, item::Item, sounds};

pub struct IngameScreen {
    inventory: Inventory,
    combine_ui: CombineUi,
    status_bar: StatusBar,
    found_popup: FoundPopup,
}

impl IngameScreen {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            combine_ui: CombineUi::new(),
            status_bar: StatusBar::new(),
            found_popup: FoundPopup::new(),
        }
    }

    fn process_combo_selection(
        &mut self,
        combo_result: ComboResult,
    ) -> Option<Box<dyn Screen + Send>> {
        match combo_result.valid_item {
            Some(item) => match self.inventory.add_found(item) {
                AddResult::Success => {
                    if matches!(item, Item::DogChicken) {
                        sounds::play_win();
                        return Some(Box::new(WinScreen));
                    } else {
                        sounds::play_hit();
                        self.status_bar.status = Status::Info("NEW!!".to_string());
                        self.found_popup.show(item);
                    }
                }
                AddResult::AlreadyFound => {
                    sounds::play_miss();
                    self.status_bar.status = Status::Error("Already found combo".to_string());
                }
            },
            None => {
                sounds::play_miss();
                self.status_bar.status = Status::Error("Invalid combo".to_string());
            }
        }

        None
    }
}

impl Screen for IngameScreen {
    fn update(&mut self, input: &Input) -> Option<Box<dyn Screen + Send>> {
        if self.found_popup.will_consume_input() {
            self.found_popup.update(&input);
        } else {
            self.inventory.update(input);

            let selected_item = self.inventory.selected_item();
            let selected_combo_result = self.combine_ui.update(input, selected_item);

            if let Some(combo_result) = selected_combo_result {
                return self.process_combo_selection(combo_result);
            }
        }

        None
    }

    fn draw(&self) {
        self.inventory.draw();
        self.combine_ui.draw();
        self.status_bar.draw();
        self.found_popup.draw();
    }
}
