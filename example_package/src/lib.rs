use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
pub const MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
pub const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
pub const LIGHT_GRAY: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
pub const LIGHT_BLUE: [f32; 4] = [0.5, 0.5, 1.0, 1.0];
pub const LIGHT_GREEN: [f32; 4] = [0.0, 1.0, 0.5, 1.0];
pub const LIGHT_RED: [f32; 4] = [1.0, 0.0, 0.5, 1.0];

pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

pub mod vector;
pub mod particle;
pub mod matrix;

/// Measures Frames Per Second (FPS).
#[derive(Debug)]
pub struct FPSCounter {
    /// The last registered frames.
    last_second_frames: VecDeque<Instant>
}

impl Default for FPSCounter {
    fn default() -> Self {
        FPSCounter::new()
    }
}

impl FPSCounter {
    /// Creates a new FPSCounter.
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second_frames: VecDeque::with_capacity(128)
        }
    }

    /// Updates the FPSCounter and returns number of frames.
    pub fn tick(&mut self) -> usize {
        let now = Instant::now();
        let a_second_ago = now - Duration::from_secs(1);

        while self.last_second_frames.front().map_or(false, |t| *t < a_second_ago) {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}


fn from_rgba(pack: [f32; 4]) -> [f32; 4] {
    let [r, g, b, a] = pack;
    let [r_f, g_f, b_f] = [r / 255.0, g / 255.0, b / 255.0];
    [r_f, g_f, b_f, a]
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub fn random_color() -> [f32; 4] {
    let r = rand::thread_rng().gen_range(0.0..1.0);
    let g = rand::thread_rng().gen_range(0.0..1.0);
    let b = rand::thread_rng().gen_range(0.0..1.0);
    [r, g, b, 1.0]
}

pub fn inverse_color(color: [f32; 4]) -> [f32; 4] {
    [1.0 - color[0], 1.0 - color[1], 1.0 - color[2], 1.0]
}

pub fn u16_to_u8(x: u16) -> u8 {
    if x > 255 {
        255
    } else {
        x as u8
    }
}

use std::convert::TryInto;

use rand::Rng;

pub fn convert_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
