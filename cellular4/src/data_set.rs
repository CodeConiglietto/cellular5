use crate::{
    arena_wrappers::*,
    datatype::{continuous::*, colors::*},
    node_set::*,
};

use generational_arena::*;

pub struct DataSet
{
    unfloats: Arena<UNFloat>,
    float_colors: Arena<FloatColor>,
}

impl DataSet{
    pub fn new() -> Self
    {
        DataSet{
            unfloats: Arena::new(),
            float_colors: Arena::new(),
        }
    }
}

impl Storage<UNFloat> for NodeSet {
    fn insert(&mut self, t: UNFloat) -> Index {
        self.unfloats.insert(t)
    }

    fn get(&self, idx: Index) -> Option<&UNFloat> {
        self.unfloats.get(idx)
    }
}