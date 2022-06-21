#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod combine;
mod game_state;
mod input;
mod inventory;
mod item;
mod ui;
mod wasm4;

use game_state::GameState;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm4::*;

static GAME_STATE: Lazy<Mutex<GameState>> = Lazy::new(|| Mutex::new(GameState::new()));

#[no_mangle]
fn update() {
    let mut game_state = GAME_STATE.lock().unwrap();

    unsafe { *DRAW_COLORS = 3 }

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
    }

    game_state.input.update();

    let pressed = game_state.input.pressed();
    game_state.inventory.update(pressed);
    let selected_item = game_state.inventory.selected_item();
    // TODO: Use result
    let selected_combo_result = game_state.combine.update(pressed, selected_item);

    game_state.combine.draw();
    game_state.inventory.draw();

    // TODO: Clean up selection UI
    unsafe { *DRAW_COLORS = 0x40 }
    rect(0, 0, 160, 8 + 2 + 2);
    unsafe { *DRAW_COLORS = 3 }
    text("Already found", 2, 2);
}
