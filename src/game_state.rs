use crate::inventory::Inventory;

pub struct GameState {
    pub inventory: Inventory,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
        }
    }
}
