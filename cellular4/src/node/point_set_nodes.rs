use std::sync::Arc;

use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{continuous::*, point_sets::*, points::*},
    mutagen_args::*,
    node::{continuous_nodes::*, discrete_nodes::*, mutagen_functions::*, point_nodes::*, Node},
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum PointSetNodes {
    //TODO: change mutagen weights to not be a hack
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: PointSet },
    #[mutagen(gen_weight = pipe_node_weight)]
    Translating {
        value: PointSet,
        child: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Spreading {
        value: PointSet,
        child: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Polygonal {
        value: PointSet,
        child_radius: Box<SNFloatNodes>,
        child_edges: Box<NibbleNodes>,
    },
}

impl Node for PointSetNodes {
    type Output = PointSet;

    fn compute(&self, _compute_arg: ComArg) -> Self::Output {
        use PointSetNodes::*;

        match self {
            Constant { value } => value.clone(),
            Translating { value, .. } => value.clone(),
            Spreading { value, .. } => value.clone(),
            Polygonal { value, .. } => value.clone(),
        }
    }
}

impl<'a> Updatable<'a> for PointSetNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            PointSetNodes::Translating {
                ref mut value,
                child,
            } => {
                *value = PointSet::new(
                    Arc::new(
                        value
                            .points
                            .par_iter()
                            .map(|p| {
                                p.sawtooth_add(
                                    child
                                        .compute(&arg.replace_coords(p))
                                        .scale_unfloat(UNFloat::new(0.05)),
                                )
                            })
                            .collect(),
                    ),
                    value.generator,
                );
            }
            PointSetNodes::Spreading {
                ref mut value,
                child,
            } => {
                *value = PointSet::new(
                    Arc::new(
                        value
                            .points
                            .par_iter()
                            .map(|p| {
                                p.sawtooth_add(
                                    p.subtract_normalised(value.get_random_point())
                                        .scale_unfloat(
                                            child
                                                .compute(&arg.replace_coords(p))
                                                .multiply(UNFloat::new(0.25)),
                                        ),
                                )
                            })
                            .collect(),
                    ),
                    value.generator,
                );
            }
            PointSetNodes::Polygonal {
                ref mut value,
                child_radius,
                child_edges,
            } => {
                let edges = child_edges.compute(arg).into_inner() + 2;
                let radius = child_radius.compute(arg).into_inner();
                let mut edge_vec = Vec::new();

                for i in 0..=edges {
                    let ratio = 1.0 / edges as f32;

                    edge_vec.push(
                        SNPoint::new(Point2::new((ratio * i as f32) * 2.0 - 1.0, radius))
                            .from_polar(),
                    );
                }

                *value = PointSet::new(Arc::new(edge_vec), value.generator);
            }
            _ => {}
        }
    }
}
