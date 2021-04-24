use failure::Fallible;

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

    fn new(config: Self::Config) -> Fallible<Self>;
    fn update(&mut self) -> Fallible<()>;
    fn get(&self, pos: SNPoint, t: usize) -> ByteColor;
}
