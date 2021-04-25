use std::collections::VecDeque;

use failure::Fallible;
use ndarray::prelude::*;

use crate::prelude::*;

#[cfg(unix)]
pub mod linux;
#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub use linux::*;
#[cfg(windows)]
pub use windows::*;

pub trait GenericCamera: Sized {
    type Config;

    fn new(config: Self::Config) -> Fallible<(Self, CameraFrames)>;
    fn update(&mut self, frames: &mut CameraFrames, current_t: usize) -> Fallible<()>;
}

pub struct CameraFrames {
    frames: VecDeque<Array2<ByteColor>>,
    fps: f32,
    resolution: (u32, u32),
    current_t: usize,
}

impl CameraFrames {
    pub fn get(&self, pos: SNPoint, t: usize) -> ByteColor {
        let (w, h) = self.resolution;

        self.frames[((t - self.current_t) as f32 * self.fps).round() as usize % self.frames.len()][[
            ((pos.y().to_unsigned().into_inner() * h as f32).round() as usize).min(h as usize - 1),
            ((pos.x().to_unsigned().into_inner() * w as f32).round() as usize).min(w as usize - 1),
        ]]
    }
}
