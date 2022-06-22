use crate::{assets, item::Item, wasm4::*};

const SINGLE_CHAR_WIDTH_PX: u32 = 8;
const SINGLE_CHAR_HEIGHT_PX: u32 = 8;

// text
pub fn draw_text<T: AsRef<str>>(content: T, x: i32, y: i32) {
    unsafe { *DRAW_COLORS = 0x04 }
    text(content, x, y);
}

pub fn draw_text_top_center<T: AsRef<str>>(content: T, x: i32, y: i32) {
    content
        .as_ref()
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            let x = x - ((line.len() as i32 * SINGLE_CHAR_WIDTH_PX as i32) / 2);
            let y = y + (idx as i32) * (SINGLE_CHAR_HEIGHT_PX as i32 + 2);
            draw_text(line, x, y);
        });
}

pub fn draw_text_bottom_left<T: AsRef<str>>(content: T, x: i32, y: i32) {
    let total_lines = (content.as_ref().chars().filter(|c| *c == '\n').count() + 1) as i32;
    let y = y - total_lines * SINGLE_CHAR_HEIGHT_PX as i32;
    draw_text(content, x, y);
}

pub fn draw_text_bottom_right<T: AsRef<str>>(content: T, x: i32, y: i32) {
    let total_lines = (content.as_ref().chars().filter(|c| *c == '\n').count() + 1) as i32;

    content
        .as_ref()
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            let x = x - line.len() as i32 * SINGLE_CHAR_WIDTH_PX as i32;
            let y = y
                - (total_lines - idx as i32) * SINGLE_CHAR_HEIGHT_PX as i32
                - (total_lines - idx as i32 - 1) * 2;
            draw_text(line, x, y);
        });
}

// items
const OBJECTS_PNG_ALTAS_COL_COUNT: u32 = 8;
const OBJECTS_PNG_ALTAS_ROW_COUNT: u32 = 5;
pub const ITEM_WIDTH_PX: u32 = assets::objects_png::OBJECTS_PNG_WIDTH / OBJECTS_PNG_ALTAS_COL_COUNT;
pub const ITEM_HEIGHT_PX: u32 =
    assets::objects_png::OBJECTS_PNG_HEIGHT / OBJECTS_PNG_ALTAS_ROW_COUNT;

pub fn draw_item(item_type: Item, x: i32, y: i32) {
    unsafe { *DRAW_COLORS = 0x324 };

    let src_x = ((item_type as u32) % OBJECTS_PNG_ALTAS_COL_COUNT) * ITEM_WIDTH_PX;
    let src_y = ((item_type as u32) / OBJECTS_PNG_ALTAS_COL_COUNT) * ITEM_HEIGHT_PX;

    blit_sub(
        &assets::objects_png::OBJECTS_PNG,
        x,
        y,
        ITEM_WIDTH_PX,
        ITEM_HEIGHT_PX,
        src_x,
        src_y,
        assets::objects_png::OBJECTS_PNG_WIDTH,
        assets::objects_png::OBJECTS_PNG_FLAGS,
    );
}
