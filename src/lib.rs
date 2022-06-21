#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod combine;
mod game_state;
mod input;
mod inventory;
mod item;
mod status_bar;
mod textscreen;
mod ui;
mod wasm4;

use game_state::GameState;
use item::Item;
use once_cell::sync::Lazy;
use status_bar::Status;
use std::sync::Mutex;
use textscreen::TextScreen;

static GAME_STATE: Lazy<Mutex<GameState>> = Lazy::new(|| Mutex::new(GameState::new()));

#[no_mangle]
fn update() {
    let mut game_state = GAME_STATE.lock().unwrap();

    game_state.input.update();
    let pressed = game_state.input.pressed();

    if let Some(textscreen) = &game_state.textscreen {
        if textscreen.update(pressed) {
            game_state.textscreen = None;
        }
    } else {
        game_state.inventory.update(pressed);
        let selected_item = game_state.inventory.selected_item();
        let selected_combo_result = game_state.combine.update(pressed, selected_item);

        if let Some(combo_result) = selected_combo_result {
            match combo_result.valid_item {
                Some(item) => {
                    // TODO: handle this
                    match game_state.inventory.add_found(item) {
                        inventory::AddResult::Success => {
                            game_state.status_bar.status = Status::Info("NEW!!".to_string());
                            if matches!(item, Item::DogChicken) {
                                game_state.textscreen = Some(TextScreen::Win);
                            } else {
                                game_state.textscreen = Some(TextScreen::Found(item));
                            }
                        }
                        inventory::AddResult::AlreadyFound => {
                            game_state.status_bar.status =
                                Status::Error("Already found combo".to_string());
                        }
                    }
                }
                None => {
                    game_state.status_bar.status = Status::Error("Invalid combo".to_string());
                }
            }
        }
    }

    game_state.combine.draw();
    game_state.inventory.draw();
    game_state.status_bar.draw();

    if let Some(textscreen) = &game_state.textscreen {
        textscreen.draw();
    }
}
