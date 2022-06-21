use crate::wasm4::GAMEPAD1;

pub struct Input {
    previous: u8,
    current: u8,
    pressed: u8,
}

impl Input {
    pub fn new() -> Self {
        Self {
            previous: 0,
            current: 0,
            pressed: 0,
        }
    }

    pub fn update(&mut self) {
        self.previous = self.current;
        self.current = unsafe { *GAMEPAD1 };
        self.pressed = self.current & (self.current ^ self.previous);
    }

    pub fn pressed(&self, buttons: u8) -> bool {
        self.pressed & buttons != 0
    }

    pub fn pressed_any(&self) -> bool {
        self.pressed != 0
    }
}
