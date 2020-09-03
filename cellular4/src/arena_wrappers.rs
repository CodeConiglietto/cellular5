use rand::prelude::*;
use std::marker::PhantomData;

use crate::{constants::*, mutagen_args::*, node::*, node_set::*};

use generational_arena::*;
use mutagen::*;
use rand::seq::IteratorRandom;

pub trait Storage<T> {
    fn arena(&self) -> &Arena<ArenaSlot<T>>;
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<T>>;
}

#[derive(Debug)]
pub struct Metarena<T> {
    pub value: Arena<ArenaSlot<T>>,
}

impl<T> Metarena<T> {
    pub fn new() -> Metarena<T> {
        Metarena {
            value: Arena::new(),
        }
    }
}

impl<T> Default for Metarena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Updatable<'a> for Metarena<T>
where
    T: Updatable<'a, UpdateArg = UpdArg<'a>>,
{
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: State, arg: Self::UpdateArg) {
        self.value
            .retain(|_index, value| value.last_accessed == arg.current_t);
    }
}
impl<'a, T> UpdatableRecursively<'a> for Metarena<T>
where
    T: Updatable<'a, UpdateArg = UpdArg<'a>>,
{
    fn update_recursively(&mut self, state: State, arg: Self::UpdateArg) {
        self.update(state, arg)
    }
}

#[derive(Debug)]
pub struct ArenaSlot<T> {
    value: T,
    last_accessed: usize,
}

#[derive(Debug)]
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
        compute_arg.nodes[self.depth].arena()[self.index]
            .value
            .compute(compute_arg.reborrow())
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
        let GenArg {
            nodes,
            data,
            depth,
            current_t,
        } = arg;

        let (current, children) = nodes.split_at_mut(1);
        let current = &mut current[0];

        let (depth, index) = if rng.gen_bool(CONSTS.graph_convergence) {
            children
                .iter()
                .enumerate()
                .flat_map(|(d, c)| c.arena().iter().map(move |(idx, _)| (depth + d, idx)))
                .choose(rng)
        } else {
            None
        }
        .unwrap_or_else(move || {
            (
                depth,
                current.arena_mut().insert(ArenaSlot {
                    value: T::generate_rng(
                        rng,
                        state,
                        GenArg {
                            nodes: children,
                            data,
                            depth: depth + 1,
                            current_t,
                        },
                    ),
                    last_accessed: current_t,
                }),
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
        state: mutagen::State,
        arg: Self::MutArg,
    ) {
        if rng.gen_bool(CONSTS.node_regenerate_chance) {
            *self = Self::generate_rng(rng, state, arg.into());
        } else {
            let (current, children) = arg.nodes.split_at_mut(1);
            let current = &mut current[0];

            current.arena_mut()[self.index].value.mutate_rng(
                rng,
                state,
                MutArg {
                    nodes: children,
                    data: arg.data,
                    depth: arg.depth + 1,
                    current_t: arg.current_t,
                },
            );
        }
    }
}

impl<'a, T> Updatable<'a> for NodeBox<T>
where
    NodeSet: Storage<T>,
    T: Updatable<'a, UpdateArg = UpdArg<'a>>,
{
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, state: mutagen::State, arg: Self::UpdateArg) {
        let (current, children) = arg.nodes.split_at_mut(1);
        let current = &mut current[0];

        let node = &mut current.arena_mut()[self.index];

        if node.last_accessed != arg.current_t {
            node.last_accessed = arg.current_t;
            node.value.update(
                state,
                UpdArg {
                    nodes: children,
                    data: arg.data,
                    depth: arg.depth + 1,
                    coordinate_set: arg.coordinate_set,
                    history: arg.history,
                    current_t: arg.current_t,
                },
            );
        }
    }
}

impl<'a, T> UpdatableRecursively<'a> for NodeBox<T>
where
    NodeSet: Storage<T>,
    T: UpdatableRecursively<'a, UpdateArg = UpdArg<'a>>,
{
    fn update_recursively(&mut self, state: mutagen::State, arg: Self::UpdateArg) {
        let (current, children) = arg.nodes.split_at_mut(1);
        let current = &mut current[0];

        let node = &mut current.arena_mut()[self.index];

        if node.last_accessed != arg.current_t {
            node.last_accessed = arg.current_t;
            node.value.update_recursively(
                state,
                UpdArg {
                    nodes: children,
                    data: arg.data,
                    depth: arg.depth + 1,
                    coordinate_set: arg.coordinate_set,
                    history: arg.history,
                    current_t: arg.current_t,
                },
            );
        }
    }
}
