use std::marker::PhantomData;

use generational_arena::*;
use log::warn;
use mutagen::*;
use rand::prelude::*;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use crate::{constants::*, mutagen_args::*, node::*, node_set::*};

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
        Self {
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

    fn update(&mut self, arg: Self::UpdateArg) {
        self.value
            .retain(|_index, value| value.last_accessed + 1 >= arg.current_t);
    }
}
impl<'a, T> UpdatableRecursively<'a> for Metarena<T>
where
    T: Updatable<'a, UpdateArg = UpdArg<'a>>,
{
    fn update_recursively(&mut self, arg: Self::UpdateArg) {
        self.update(arg)
    }
}

#[derive(Debug)]
pub struct ArenaSlot<T> {
    value: T,
    last_accessed: usize,
}

#[derive(Debug, Serialize, Deserialize)]
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

    fn compute(&self, arg: ComArg) -> Self::Output {
        let depth_skipped = self.depth - arg.depth;
        let (current, children) = arg.nodes[depth_skipped..].split_first().unwrap();

        let slot = &current.arena()[self.index];

        if slot.last_accessed + 1 < arg.current_t {
            warn!(
                "NODE SHOULD BE CULLED BUT IS GETTING COMPUTED {:?}",
                std::any::type_name::<T>()
            );
        }

        slot.value.compute(ComArg {
            nodes: children,
            data: arg.data,
            depth: self.depth + 1,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            current_t: arg.current_t,
        })
    }
}

impl<'a, T> Generatable<'a> for NodeBox<T>
where
    NodeSet: Storage<T>,
    T: Generatable<'a, GenArg = GenArg<'a>>,
{
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, arg: Self::GenArg) -> Self {
        let GenArg {
            nodes,
            data,
            depth,
            current_t,
        } = arg;

        if nodes.is_empty() {
            dbg!(depth);
            panic!("No nodesets left to allocate to! Is a node weight mislabeled?");
        }

        let nodes_len = nodes.len();

        if rng.gen_bool(CONSTS.graph_convergence) {
            if let Some((child_depth, index)) = nodes[1..]
                .iter()
                .enumerate()
                .flat_map(|(d, c)| c.arena().iter().map(move |(idx, _)| (d, idx)))
                .choose(rng)
            {
                return Self {
                    index,
                    depth: depth + child_depth + 1,
                    _marker: PhantomData,
                };
            }
        }

        assert_eq!(depth + nodes.len(), crate::node::max_node_depth() + 1);

        let depth_skipped: usize = rng.gen_range(0, nodes.len());
        let (current, children) = nodes[depth_skipped..].split_first_mut().unwrap();

        if depth > crate::node::max_node_depth()
            || depth + depth_skipped > crate::node::max_node_depth()
            || depth + depth_skipped + 1 + children.len() != crate::node::max_node_depth() + 1
        {
            dbg!(depth);
            dbg!(depth_skipped);
            dbg!(nodes_len);
            dbg!(children.len());
            dbg!(crate::node::max_node_depth());

            panic!("IT HAPPENED");
        }

        if children.is_empty() {
            dbg!("THIS BETTER GEN A LEAF NODE OR WE HITTIN PANIC TOWN");
            dbg!(depth);
            dbg!(depth_skipped);
            dbg!(crate::node::max_node_depth());
            dbg!(std::any::type_name::<T>());

            let test_arg = GenArg {
                nodes: children,
                data,
                depth: depth + depth_skipped + 1,
                current_t,
            };

            dbg!(crate::node::mutagen_functions::leaf_node_weight(&test_arg));
            dbg!(crate::node::mutagen_functions::pipe_node_weight(&test_arg));
            dbg!(crate::node::mutagen_functions::branch_node_weight(
                &test_arg
            ));
        }

        let index = current.arena_mut().insert(ArenaSlot {
            value: T::generate_rng(
                rng,
                GenArg {
                    nodes: children,
                    data,
                    depth: depth + depth_skipped + 1,
                    current_t,
                },
            ),
            last_accessed: current_t,
        });

        Self {
            index,
            depth: depth + depth_skipped,
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

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        let depth_skipped = self.depth - arg.depth;

        if rng.gen_bool(CONSTS.node_regenerate_chance) {
            *self = Self::generate_rng(
                rng,
                GenArg {
                    nodes: &mut arg.nodes[depth_skipped..],
                    data: arg.data,
                    depth: self.depth,
                    current_t: arg.current_t,
                },
            );
        } else {
            let (current, children) = arg.nodes[depth_skipped..].split_first_mut().unwrap();

            current.arena_mut()[self.index].value.mutate_rng(
                rng,
                MutArg {
                    nodes: children,
                    data: arg.data,
                    depth: self.depth + 1,
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

    fn update(&mut self, arg: Self::UpdateArg) {
        let depth_skipped = self.depth - arg.depth;
        let (current, children) = arg.nodes[depth_skipped..].split_first_mut().unwrap();

        let node = &mut current.arena_mut()[self.index];

        if node.last_accessed != arg.current_t {
            node.last_accessed = arg.current_t;
            node.value.update(UpdArg {
                nodes: children,
                data: arg.data,
                depth: self.depth + 1,
                coordinate_set: arg.coordinate_set,
                history: arg.history,
                current_t: arg.current_t,
            });
        }
    }
}

impl<'a, T> UpdatableRecursively<'a> for NodeBox<T>
where
    NodeSet: Storage<T>,
    T: UpdatableRecursively<'a, UpdateArg = UpdArg<'a>>,
{
    fn update_recursively(&mut self, arg: Self::UpdateArg) {
        let depth_skipped = self.depth - arg.depth;
        let (current, children) = arg.nodes[depth_skipped..].split_first_mut().unwrap();

        let node = &mut current.arena_mut()[self.index];

        if node.last_accessed != arg.current_t {
            node.last_accessed = arg.current_t;
            node.value.update_recursively(UpdArg {
                nodes: children,
                data: arg.data,
                depth: self.depth + 1,
                coordinate_set: arg.coordinate_set,
                history: arg.history,
                current_t: arg.current_t,
            });
        }
    }
}
