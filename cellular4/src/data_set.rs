use crate::{
    arena_wrappers::*,
    datatype::{colors::*, continuous::*},
};

use generational_arena::*;

#[derive(Debug)]
pub struct DataSet {
    unfloats: Arena<UNFloat>,
    float_colors: Arena<FloatColor>,
}

impl DataSet {
    pub fn new() -> Self {
        DataSet {
            unfloats: Arena::new(),
            float_colors: Arena::new(),
        }
    }
}

impl Storage<UNFloat> for DataSet {
    fn arena(&self) -> &Arena<UNFloat> {
        &self.unfloats
    }

    fn arena_mut(&mut self) -> &mut Arena<UNFloat> {
        &mut self.unfloats
    }
}

impl Storage<FloatColor> for DataSet {
    fn arena(&self) -> &Arena<FloatColor> {
        &self.float_colors
    }

    fn arena_mut(&mut self) -> &mut Arena<FloatColor> {
        &mut self.float_colors
    }
}
