use crate::wasm4::*;

const SINGLE_CHAR_PIXEL: u32 = 8;

pub fn draw_text<T: AsRef<str>>(content: T, x: i32, y: i32) {
    unsafe { *DRAW_COLORS = 3 }
    text(content, x, y);
}

pub fn draw_text_bottom_left<T: AsRef<str>>(content: T, x: i32, y: i32) {
    let total_lines = (content.as_ref().chars().filter(|c| *c == '\n').count() + 1) as i32;
    let y = y - total_lines * SINGLE_CHAR_PIXEL as i32;
    draw_text(content, x, y);
}

pub fn draw_text_bottom_right<T: AsRef<str>>(content: T, x: i32, y: i32) {
    let total_lines = (content.as_ref().chars().filter(|c| *c == '\n').count() + 1) as i32;

    if total_lines == 1 {
        let x = x - content.as_ref().len() as i32 * SINGLE_CHAR_PIXEL as i32;
        draw_text(content, x, y - SINGLE_CHAR_PIXEL as i32);
    } else {
        content
            .as_ref()
            .lines()
            .enumerate()
            .for_each(|(idx, line)| {
                let x = x - line.len() as i32 * SINGLE_CHAR_PIXEL as i32;
                let y = y
                    - (total_lines - idx as i32) * SINGLE_CHAR_PIXEL as i32
                    - (total_lines - idx as i32 - 1) * 2;
                draw_text(line, x, y);
            });
    }
}
