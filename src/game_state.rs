use crate::{input::Input, inventory::Inventory};

pub struct GameState {
    pub inventory: Inventory,
    pub input: Input,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            input: Input::new(),
        }
    }
}
