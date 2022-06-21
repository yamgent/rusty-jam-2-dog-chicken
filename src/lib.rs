#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod game_state;
mod inventory;
mod item;
mod wasm4;

use game_state::GameState;
use item::SINGLE_OBJ_PIXELS;
use once_cell::sync::Lazy;
use wasm4::*;

static GAME_STATE: Lazy<GameState> = Lazy::new(GameState::new);

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 3 }

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
    }

    unsafe { *DRAW_COLORS = 0x123 };

    // TODO: Clean up selection UI
    item::draw_item(item::Item::Cow, 18, 16);
    item::draw_item(item::Item::Cow, 66, 16);
    item::draw_item(item::Item::Cow, 116, 16);

    GAME_STATE.inventory.draw();

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

    // TODO: Clean up inventory UI
    text("Water", 2, 160 - (8 + 2));
    text("Page", 160 - (8 * 4) - 2, 160 - ((8 + 2) * 2));
    text("1/1", 160 - (8 * 3) - 2, 160 - (8 + 2));
}
