use crate::wasm4::{tone, TONE_NOISE, TONE_PULSE1, TONE_TRIANGLE};

struct ToneParam {
    freq: (u16, u16),
    attack: u8,
    decay: u8,
    sustain: u8,
    release: u8,
    volume: u8,
    // if passed in 0, it will default to 100%
    peak: u8,
    channel: u32,
}

fn play_tone(param: ToneParam) {
    tone(
        (param.freq.0 as u32) | ((param.freq.1 as u32) << 16),
        ((param.attack as u32) << 24)
            | ((param.decay as u32) << 16)
            | (param.sustain as u32)
            | ((param.release as u32) << 8),
        (param.volume as u32) | ((param.peak as u32) << 8),
        param.channel,
    );
}

pub fn play_miss() {
    play_tone(ToneParam {
        freq: (440, 230),
        attack: 0,
        decay: 0,
        sustain: 0,
        release: 4,
        volume: 100,
        peak: 100,
        channel: TONE_PULSE1,
    });
}

pub fn play_hit() {
    play_tone(ToneParam {
        freq: (430, 560),
        attack: 36,
        decay: 0,
        sustain: 10,
        release: 18,
        volume: 100,
        peak: 100,
        channel: TONE_TRIANGLE,
    });
}

pub fn play_win() {
    play_tone(ToneParam {
        freq: (320, 590),
        attack: 36,
        decay: 52,
        sustain: 10,
        release: 18,
        volume: 30,
        peak: 30,
        channel: TONE_NOISE,
    });
}
