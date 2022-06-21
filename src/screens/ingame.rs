use super::{win::WinScreen, Screen};
use crate::{
    combine::Combine,
    input::Input,
    inventory::{AddResult, Inventory},
    item::Item,
    sounds,
    status_bar::{Status, StatusBar},
    textscreen::TextScreen,
};

pub struct IngameScreen {
    inventory: Inventory,
    combine: Combine,
    status_bar: StatusBar,
    textscreen: Option<TextScreen>,
}

impl IngameScreen {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            combine: Combine::new(),
            status_bar: StatusBar::new(),
            textscreen: None,
        }
    }
}

impl Screen for IngameScreen {
    fn update(&mut self, input: &Input) -> Option<Box<dyn Screen + Send>> {
        if let Some(textscreen) = &self.textscreen {
            if textscreen.update(input) {
                self.textscreen = None;
            }
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
                                self.textscreen = Some(TextScreen { found_item: item });
                                sounds::play_good();
                            }
                        }
                        AddResult::AlreadyFound => {
                            self.status_bar.status =
                                Status::Error("Already found combo".to_string());
                            sounds::play_bad();
                        }
                    },
                    None => {
                        self.status_bar.status = Status::Error("Invalid combo".to_string());
                        sounds::play_bad();
                    }
                }
            }
        }

        None
    }

    fn draw(&self) {
        self.combine.draw();
        self.inventory.draw();
        self.status_bar.draw();

        if let Some(textscreen) = &self.textscreen {
            textscreen.draw();
        }
    }
}
