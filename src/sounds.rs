use crate::wasm4::{tone, TONE_NOISE, TONE_PULSE1, TONE_TRIANGLE};

pub fn play_bad() {
    tone(440 | (230 << 16), 4 << 8, 100, TONE_PULSE1);
}

pub fn play_good() {
    tone(
        430 | (560 << 16),
        (36 << 24) | 10 | (18 << 8),
        100,
        TONE_TRIANGLE,
    );
}

pub fn play_win() {
    tone(
        320 | (590 << 16),
        (36 << 24) | (52 << 16) | 10 | (18 << 8),
        30 | (30 << 8),
        TONE_NOISE,
    );
}
