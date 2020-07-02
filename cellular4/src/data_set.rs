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
    fn insert(&mut self, t: UNFloat) -> Index {
        self.unfloats.insert(t)
    }

    fn get(&self, idx: Index) -> Option<&UNFloat> {
        self.unfloats.get(idx)
    }
}
