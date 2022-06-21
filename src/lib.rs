#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod game_state;
mod input;
mod inventory;
mod item;
mod ui;
mod wasm4;

use game_state::GameState;
use item::SINGLE_OBJ_PIXELS;
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

    // TODO: Clean up selection UI
    item::draw_item(item::Item::Cow, 18, 16);
    item::draw_item(item::Item::Cow, 66, 16);
    item::draw_item(item::Item::Cow, 116, 16);

    game_state.inventory.draw();

    // TODO: Clean up selection UI
    unsafe { *DRAW_COLORS = 0x40 }
    rect(18 - 2, 16 - 2, SINGLE_OBJ_PIXELS + 4, SINGLE_OBJ_PIXELS + 4);
    rect(66 - 2, 16 - 2, SINGLE_OBJ_PIXELS + 4, SINGLE_OBJ_PIXELS + 4);
    rect(
        116 - 2,
        16 - 2,
        SINGLE_OBJ_PIXELS + 4,
        SINGLE_OBJ_PIXELS + 4,
    );
    rect(0, 0, 160, 8 + 2 + 2);
    unsafe { *DRAW_COLORS = 3 }
    text("+", 50, 16 + 8);
    text("=", 100, 16 + 8);
    text("Already found", 2, 2);
}
