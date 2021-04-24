use failure::Fallible;

use crate::prelude::*;

pub struct Camera {}

impl GenericCamera for Camera {
    fn new(config: Self::Config) -> Fallible<Self> {
        todo!()
    }

    fn update(&mut self) -> Fallible<()> {
        todo!()
    }

    fn get(&self, pos: SNPoint, t: usize) -> ByteColor {
        todo!()
    }
}
