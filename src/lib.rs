#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod combine;
mod game;
mod input;
mod inventory;
mod item;
mod screens;
mod sounds;
mod status_bar;
mod textscreen;
mod ui;
mod wasm4;

use game::Game;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static GAME: Lazy<Mutex<Game>> = Lazy::new(|| Mutex::new(Game::new()));

#[no_mangle]
fn update() {
    GAME.lock().unwrap().update();
}
