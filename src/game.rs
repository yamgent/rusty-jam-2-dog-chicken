use crate::{
    input::Input,
    screens::{intro::IntroScreen, Screen},
};

pub struct Game {
    input: Input,
    screen: Box<dyn Screen + Send>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            screen: Box::new(IntroScreen),
        }
    }

    pub fn update(&mut self) {
        self.input.update();

        if let Some(new_screen) = self.screen.update(&self.input) {
            self.screen = new_screen;
        }

        self.screen.draw();
    }
}
