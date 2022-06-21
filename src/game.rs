use crate::{
    combine::Combine,
    input::Input,
    inventory::{AddResult, Inventory},
    item::Item,
    sounds,
    status_bar::{Status, StatusBar},
    textscreen::TextScreen,
};

pub struct Game {
    pub inventory: Inventory,
    pub input: Input,
    pub combine: Combine,
    pub status_bar: StatusBar,
    pub textscreen: Option<TextScreen>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            input: Input::new(),
            combine: Combine::new(),
            status_bar: StatusBar::new(),
            textscreen: Some(TextScreen::Intro),
        }
    }

    pub fn update(&mut self) {
        self.input.update();

        if let Some(textscreen) = &self.textscreen {
            if textscreen.update(&self.input) {
                self.textscreen = None;
            }
        } else {
            self.inventory.update(&self.input);
            let selected_item = self.inventory.selected_item();
            let selected_combo_result = self.combine.update(&self.input, selected_item);

            if let Some(combo_result) = selected_combo_result {
                match combo_result.valid_item {
                    Some(item) => match self.inventory.add_found(item) {
                        AddResult::Success => {
                            self.status_bar.status = Status::Info("NEW!!".to_string());
                            if matches!(item, Item::DogChicken) {
                                self.textscreen = Some(TextScreen::Win);
                                sounds::play_win();
                            } else {
                                self.textscreen = Some(TextScreen::Found(item));
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

        self.combine.draw();
        self.inventory.draw();
        self.status_bar.draw();

        if let Some(textscreen) = &self.textscreen {
            textscreen.draw();
        }
    }
}
