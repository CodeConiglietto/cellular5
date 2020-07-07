use rand::prelude::*;
use std::marker::PhantomData;

use crate::{constants::*, mutagen_args::*, node::*, node_set::*};

use generational_arena::*;
use mutagen::*;
use rand::seq::IteratorRandom;

pub trait Storage<T> {
    fn arena(&self) -> &Arena<T>;
    fn arena_mut(&mut self) -> &mut Arena<T>;
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
        compute_arg.nodes[self.depth].arena()[self.index].compute(compute_arg.reborrow())
    }
}

impl<'a, T> Generatable<'a> for NodeBox<T>
where
    NodeSet: Storage<T>,
    T: Generatable<'a, GenArg = GenArg<'a>>,
{
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        state: mutagen::State,
        arg: Self::GenArg,
    ) -> Self {
        let (current, children) = arg.nodes.split_at_mut(1);
        let current = &mut current[0];
        let data = arg.data;

        let (depth, index) = if rng.gen_bool(CONSTS.graph_convergence) {
            children
                .iter()
                .enumerate()
                .flat_map(|(d, c)| c.arena().iter().map(move |(idx, _)| (state.depth + d, idx)))
                .choose(rng)
        } else {
            None
        }
        .unwrap_or_else(move || {
            (
                state.depth,
                current.arena_mut().insert(T::generate_rng(
                    rng,
                    state,
                    GenArg {
                        nodes: children,
                        data,
                    },
                )),
            )
        });

        Self {
            index,
            depth,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Mutatable<'a> for NodeBox<T>
where
    NodeSet: Storage<T>,
    T: Mutatable<'a, MutArg = MutArg<'a>> + Generatable<'a, GenArg = GenArg<'a>>,
{
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        mut state: mutagen::State,
        arg: Self::MutArg,
    ) {
        state.depth = self.depth;

        if rng.gen_bool(CONSTS.node_regenerate_chance) {
            *self = Self::generate_rng(rng, state, arg.into());
        } else {
            let (current, children) = arg.nodes.split_at_mut(1);
            let current = &mut current[0];

            current.arena_mut()[self.index].mutate_rng(
                rng,
                state,
                MutArg {
                    nodes: children,
                    data: arg.data,
                },
            );
        }
    }
}
