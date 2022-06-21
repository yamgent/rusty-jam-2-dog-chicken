use crate::{combine::Combine, input::Input, inventory::Inventory, status_bar::StatusBar};

pub struct GameState {
    pub inventory: Inventory,
    pub input: Input,
    pub combine: Combine,
    pub status_bar: StatusBar,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            input: Input::new(),
            combine: Combine::new(),
            status_bar: StatusBar::new(),
        }
    }
}
