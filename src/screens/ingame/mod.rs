mod combine;
mod found_popup;
mod inventory;
mod status_bar;

use combine::Combine;
use found_popup::FoundPopup;
use inventory::{AddResult, Inventory};
use status_bar::{Status, StatusBar};

use super::{win::WinScreen, Screen};
use crate::{input::Input, item::Item, sounds};

pub struct IngameScreen {
    inventory: Inventory,
    combine: Combine,
    status_bar: StatusBar,
    found_popup: FoundPopup,
}

impl IngameScreen {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            combine: Combine::new(),
            status_bar: StatusBar::new(),
            found_popup: FoundPopup::new(),
        }
    }
}

impl Screen for IngameScreen {
    fn update(&mut self, input: &Input) -> Option<Box<dyn Screen + Send>> {
        if self.found_popup.will_consume_input() {
            self.found_popup.update(&input);
        } else {
            self.inventory.update(input);
            let selected_item = self.inventory.selected_item();
            let selected_combo_result = self.combine.update(input, selected_item);

            if let Some(combo_result) = selected_combo_result {
                match combo_result.valid_item {
                    Some(item) => match self.inventory.add_found(item) {
                        AddResult::Success => {
                            self.status_bar.status = Status::Info("NEW!!".to_string());
                            if matches!(item, Item::DogChicken) {
                                sounds::play_win();
                                return Some(Box::new(WinScreen));
                            } else {
                                self.found_popup.show(item);
                                sounds::play_hit();
                            }
                        }
                        AddResult::AlreadyFound => {
                            self.status_bar.status =
                                Status::Error("Already found combo".to_string());
                            sounds::play_miss();
                        }
                    },
                    None => {
                        self.status_bar.status = Status::Error("Invalid combo".to_string());
                        sounds::play_miss();
                    }
                }
            }
        }

        None
    }

    fn draw(&self) {
        self.inventory.draw();
        self.combine.draw();
        self.status_bar.draw();
        self.found_popup.draw();
    }
}
