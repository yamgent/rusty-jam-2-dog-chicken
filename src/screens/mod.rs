use crate::input::Input;

pub mod debug_gallery;
pub mod ingame;
pub mod intro;
pub mod win;

pub trait Screen {
    fn update(&mut self, input: &Input) -> Option<Box<dyn Screen + Send>>;
    fn draw(&self);
}
