use crate::{combine::Combine, input::Input, inventory::Inventory};

pub struct GameState {
    pub inventory: Inventory,
    pub input: Input,
    pub combine: Combine,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            input: Input::new(),
            combine: Combine::new(),
        }
    }
}
