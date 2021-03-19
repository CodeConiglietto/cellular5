use std::marker::PhantomData;

use generational_arena::*;
use log::warn;
use mutagen::*;
use rand::prelude::*;
use rand::{distributions::weighted::WeightedIndex, seq::IteratorRandom};
use serde::{Deserialize, Serialize};

use crate::{constants::*, mutagen_args::*, node::*, node_set::*};

pub trait Storage<T> {
    fn arena(&self) -> &Arena<ArenaSlot<T>>;
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<T>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metarena<T> {
    pub value: Arena<ArenaSlot<T>>,
}

impl<T> Metarena<T> {
    pub fn new() -> Metarena<T> {
        Self {
            value: Arena::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.value.len()
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
            .retain(|_index, value| value.last_accessed + 50 >= arg.current_t);
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

#[derive(Debug, Serialize, Deserialize)]
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

        if slot.last_accessed + 1 < arg.current_t && cfg!(debug_assertions) {
            warn!(
                "NODE SHOULD BE CULLED BUT IS GETTING COMPUTED {:?}",
                std::any::type_name::<T>()
            );
            ldbg!(slot.last_accessed);
            ldbg!(arg.current_t);
        }

        slot.value.compute(ComArg {
            nodes: children,
            data: arg.data,
            depth: self.depth + 1,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            current_t: arg.current_t,
            mic_histograms: arg.mic_histograms,
            gamepads: arg.gamepads,
        })
    }
}

impl<'a, T> Generatable<'a> for NodeBox<T>
where
    NodeSet: Storage<T>,
    T: Generatable<'a, GenArg = GenArg<'a>>
        + Updatable<'a, UpdateArg = UpdArg<'a>>
        + UpdatableRecursively<'a>,
{
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, arg: Self::GenArg) -> Self {
        if arg.nodes.is_empty() {
            panic!(
                "No nodesets left to allocate to at depth {}! Is a node weight mislabeled?",
                arg.depth
            );
        }

        if rng.gen_bool(CONSTS.graph_convergence) {
            if let Some((child_depth, index)) = arg
                .nodes
                .iter()
                .enumerate()
                .skip(1)
                .flat_map(|(d, c)| c.arena().iter().map(move |(idx, _)| (d, idx)))
                .choose(rng)
            {
                let mut child = Self {
                    index,
                    depth: arg.depth + child_depth,
                    _marker: PhantomData,
                };

                // NOTE This is only called to ensure that the last_updated field is updated
                // There's probably a better way to do this, probably by ensuring that the nodes
                // are updated right after mutating and before any compute calls
                child.update_recursively(arg.into());

                return child;
            }
        }

        let GenArg {
            nodes,
            data,
            depth,
            history,
            current_t,
            coordinate_set,
            image_preloader,
            profiler,
            mic_histograms,
            gamepads,
        } = arg;

        let nodes_len = nodes.len();

        assert_eq!(depth + nodes.len(), crate::node::max_node_depth() + 1);

        let depth_skipped = WeightedIndex::new((0..nodes.len()).map(|i| 1.0 / (i + 1) as f32))
            .unwrap()
            .sample(rng);

        let (current, children) = nodes[depth_skipped..].split_first_mut().unwrap();

        if depth > crate::node::max_node_depth()
            || depth + depth_skipped > crate::node::max_node_depth()
            || depth + depth_skipped + 1 + children.len() != crate::node::max_node_depth() + 1
        {
            ldbg!(depth);
            ldbg!(depth_skipped);
            ldbg!(nodes_len);
            ldbg!(children.len());
            ldbg!(crate::node::max_node_depth());

            panic!("SOMETHING'S HAPPENED");
        }

        if children.is_empty() && cfg!(debug_assertions) {
            ldbg!("THIS BETTER GEN A LEAF NODE OR WE HITTIN PANIC TOWN");
            ldbg!(depth);
            ldbg!(depth_skipped);
            ldbg!(crate::node::max_node_depth());
            ldbg!(std::any::type_name::<T>());

            let mut test_arg = GenArg {
                nodes: children,
                data,
                depth: depth + depth_skipped + 1,
                current_t,
                history,
                coordinate_set,
                image_preloader,
                profiler: &mut None,
                mic_histograms,
                gamepads,
            };

            ldbg!(crate::node::mutagen_functions::leaf_node_weight(
                test_arg.reborrow()
            ));
            ldbg!(crate::node::mutagen_functions::pipe_node_weight(
                test_arg.reborrow()
            ));
            ldbg!(crate::node::mutagen_functions::branch_node_weight(
                test_arg.reborrow()
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
                    history,
                    coordinate_set,
                    image_preloader,
                    profiler,
                    mic_histograms,
                    gamepads,
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
    T: Mutatable<'a, MutArg = MutArg<'a>>
        + Generatable<'a, GenArg = GenArg<'a>>
        + Updatable<'a, UpdateArg = UpdArg<'a>>
        + UpdatableRecursively<'a>,
{
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        let depth_skipped = self.depth - arg.depth;

        if rng.gen_bool(CONSTS.node_regenerate_chance) {
            *self = Self::generate_rng(rng, arg.into());
        } else {
            let (current, children) = arg.nodes[depth_skipped..].split_first_mut().unwrap();

            current.arena_mut()[self.index].value.mutate_rng(
                rng,
                MutArg {
                    nodes: children,
                    data: arg.data,
                    depth: self.depth + 1,
                    current_t: arg.current_t,
                    history: arg.history,
                    coordinate_set: arg.coordinate_set,
                    image_preloader: arg.image_preloader,
                    profiler: arg.profiler,
                    mic_histograms: arg.mic_histograms,
                    gamepads: arg.gamepads,
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
                image_preloader: arg.image_preloader,
                profiler: arg.profiler,
                mic_histograms: arg.mic_histograms,
                gamepads: arg.gamepads,
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
                image_preloader: arg.image_preloader,
                profiler: arg.profiler,
                mic_histograms: arg.mic_histograms,
                gamepads: arg.gamepads,
            });
        }
    }
}
