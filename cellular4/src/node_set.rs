use crate::{
    arena_wrappers::*,
    node::{continuous_nodes::*, color_nodes::*}
};
use generational_arena::*;

pub struct NodeSet
{
    unfloat_nodes: Arena<UNFloatNodes>,
    float_color_nodes: Arena<FloatColorNodes>,
}

impl Storage<UNFloatNodes> for NodeSet {
    fn insert(&mut self, t: UNFloatNodes) -> Index {
        self.unfloat_nodes.insert(t)
    }

    fn get(&self, idx: Index) -> Option<&UNFloatNodes> {
        self.unfloat_nodes.get(idx)
    }
}