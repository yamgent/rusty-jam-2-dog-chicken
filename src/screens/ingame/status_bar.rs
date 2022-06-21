use crate::wasm4::{rect, text, DRAW_COLORS};

pub enum Status {
    None,
    Info(String),
    Error(String),
}

pub struct StatusBar {
    pub status: Status,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            status: Status::None,
        }
    }

    pub fn draw(&self) {
        let (border_color, text_color, status) = match &self.status {
            Status::None => (0, 0, ""),
            Status::Info(status) => (0x33, 2, status.as_str()),
            Status::Error(status) => (0x40, 4, status.as_str()),
        };

        unsafe { *DRAW_COLORS = border_color }
        rect(0, 0, 160, 8 + 2 + 2);

        unsafe { *DRAW_COLORS = text_color }
        text(status, 2, 2);
    }
}
