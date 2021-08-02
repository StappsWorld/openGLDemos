pub mod fluid;
pub mod vector;

use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub const N : u32 = 256;
pub const WINDOW_WIDTH: u32 = N;
pub const WINDOW_HEIGHT: u32 = N;

pub const iter : usize = 4;
pub const scl : usize = 4;
pub const NX : usize = 128;
pub const NY : usize = 128;

#[derive(Debug)]
pub struct FPSCounter {
    last_second_frames: VecDeque<Instant>
}

impl Default for FPSCounter {
    fn default() -> Self {
        FPSCounter::new()
    }
}

impl FPSCounter {
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second_frames: VecDeque::with_capacity(128)
        }
    }

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

pub fn IX(x: u32, y : u32) -> usize {
    (x + y * N) as usize
}