#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod wasm4;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

const OBJECT_ALTAS_COL_COUNT: u32 = 8;
const SINGLE_OBJ_PIXELS: u32 = assets::objects::OBJECTS_PNG_WIDTH / OBJECT_ALTAS_COL_COUNT;

fn draw_object(idx: u32, x: i32, y: i32) {
    let src_x = (idx % OBJECT_ALTAS_COL_COUNT) * SINGLE_OBJ_PIXELS;
    let src_y = (idx / OBJECT_ALTAS_COL_COUNT) * SINGLE_OBJ_PIXELS;
    blit_sub(
        &assets::objects::OBJECTS_PNG,
        x,
        y,
        SINGLE_OBJ_PIXELS,
        SINGLE_OBJ_PIXELS,
        src_x,
        src_y,
        assets::objects::OBJECTS_PNG_WIDTH,
        assets::objects::OBJECTS_PNG_FLAGS,
    );
}

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 0x0123 }
    text("Hello from Rust!", 10, 10);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
    }

    blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    draw_object(0, 0, 0);
    text("Press X to blink", 16, 90);
}
