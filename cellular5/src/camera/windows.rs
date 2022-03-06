use failure::Fallible;

use crate::prelude::*;

pub struct Camera {}

impl GenericCamera for Camera {
    fn new(config: Self::Config) -> Fallible<(Self, CameraFrames)> {
        todo!()
    }

    fn update(&mut self) -> Fallible<()> {
        todo!()
    }

    fn update(&mut self, frames: &mut CameraFrames) -> Fallible<()> {
        todo!()
    }
}
