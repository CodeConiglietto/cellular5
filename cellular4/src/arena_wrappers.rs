use std::marker::PhantomData;

use crate::
{
    data_set::*,
    node_set::*,
    node::continuous_nodes::UNFloatNodes,
};

use generational_arena::*;
use mutagen::*;

pub trait Storage<T> {
    fn insert(&mut self, t: T) -> Index;
    fn get(&self, idx: Index) -> Option<&T>;
}

pub trait Storable<T> {
    fn insert_into(self, storage: &mut T) -> Index;
    fn get_from(idx: Index, storage: &T) -> &Self;
}

impl<T, U> Storable<T> for U
where
    T: Storage<U>,
{
    fn insert_into(self, t: T) -> Index { t.insert(self) }

    fn get_from(idx: Index, storage: &T) -> Option<&T> { storage.get(idx) }
}

pub struct NodeBox<T> {
    index: Index,
    _marker: PhantomData<T>,
}

impl<'a, T> Mutagen<'a> for NodeBox<T> {
    type Arg = &'a NodeSet;
}

impl<'a, T> Generatable<'a> for NodeBox<T>
where
    T: Storable<NodeSet> + Generatable<'a>,
{
    fn generate_rng(arg_nodes: &mut Vec<NodeSet>, arg_data: &DataSet) -> Self {
        let t = T::generate_rng(arg_nodes);
        Self {
            index: t.store(arg_nodes),
            _marker: PhantomData,
        }
    }

impl<'a> Generatable<'a> for SNFloat {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: UpdateState<'a>,
    ) -> Self {
        index: t.store(arg_nodes),
        _marker: PhantomData,
    }
}
}