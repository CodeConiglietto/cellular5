use rand::prelude::*;
use std::marker::PhantomData;

use crate::{mutagen_args::*, node::*, node_set::*};

use generational_arena::*;
use mutagen::*;

pub trait Storage<T> {
    fn insert(&mut self, t: T) -> Index;
    fn get(&self, idx: Index) -> Option<&T>;
}

pub trait Storable<T> {
    fn insert_into(self, storage: &mut T) -> Index;
    fn get_from(idx: Index, storage: &T) -> Option<&Self>;
}

impl<T, U> Storable<T> for U
where
    T: Storage<U>,
{
    fn insert_into(self, t: &mut T) -> Index {
        t.insert(self)
    }

    fn get_from(idx: Index, storage: &T) -> Option<&Self> {
        storage.get(idx)
    }
}

pub struct NodeBox<T> {
    index: Index,
    depth: usize,
    _marker: PhantomData<T>,
}

impl<T> Node for NodeBox<T>
where
    T: Node,
    NodeSet: Storage<T>,
{
    type Output = T::Output;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        compute_arg.nodes[self.depth]
            .get(self.index)
            .unwrap()
            .compute(compute_arg.reborrow())
    }
}

impl<'a, T> Generatable<'a> for NodeBox<T>
where
    T: Storable<NodeSet> + Generatable<'a, GenArg = GenArg<'a>>,
{
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        state: mutagen::State,
        mut arg: Self::GenArg,
    ) -> Self {
        let mut gen_arg = GenArg {
            nodes: &mut arg.nodes[1..],
            data: &mut arg.data,
        };
        let t = T::generate_rng(rng, state, gen_arg);
        Self {
            index: t.insert_into(&mut arg.nodes[0]),
            depth: state.depth,
            _marker: PhantomData,
        }
    }
}
