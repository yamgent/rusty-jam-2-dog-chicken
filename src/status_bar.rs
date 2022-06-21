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
        match &self.status {
            Status::Info(status) => {
                unsafe { *DRAW_COLORS = 0x33 }
                rect(0, 0, 160, 8 + 2 + 2);

                unsafe { *DRAW_COLORS = 2 }
                text(status, 2, 2);
            }
            Status::Error(status) => {
                unsafe { *DRAW_COLORS = 0x40 }
                rect(0, 0, 160, 8 + 2 + 2);

                unsafe { *DRAW_COLORS = 4 }
                text(status, 2, 2);
            }
            Status::None => {}
        }
    }
}
