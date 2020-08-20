use std::sync::Arc;

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
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
    #[mutagen(gen_weight = branch_node_weight)]
    ShearGrid {
        value: PointSet,
        child_x_scalar: Box<SNFloatNodes>,
        child_y_scalar: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Line {
        value: PointSet,
        child_points: Box<ByteNodes>,
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
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
            ShearGrid { value, .. } => value.clone(),
            Line { value, .. } => value.clone(),
        }
    }
}

impl<'a> Updatable<'a> for PointSetNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, mut arg: UpdArg<'a>) {
        match self {
            PointSetNodes::Translating {
                ref mut value,
                child,
            } => {
                let compute_arg = ComArg::from(arg);

                *value = PointSet::new(
                    Arc::new(
                        value
                            .points
                            .par_iter()
                            .map(|p| {
                                p.sawtooth_add(
                                    child
                                        .compute(compute_arg.clone().replace_coords(p))
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
                let compute_arg = ComArg::from(arg);

                *value = PointSet::new(
                    Arc::new(
                        value
                            .points
                            .par_iter()
                            .map(|p| {
                                // TODO Attempt to refactor to use normalised_add instead of sawtooth_add
                                p.sawtooth_add(
                                    p.subtract_normalised(value.get_random_point())
                                        .scale_unfloat(
                                            child
                                                .compute(compute_arg.clone().replace_coords(p))
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
                let edges = child_edges.compute(arg.reborrow().into()).into_inner() + 2;
                let radius = child_radius.compute(arg.reborrow().into()).into_inner();
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
            //TODO: Something is funky here, give it a second pass.
            //TODO: Maybe swap this out for a matrix point grid
            PointSetNodes::ShearGrid {
                ref mut value,
                child_x_scalar,
                child_y_scalar,
            } => {
                let x_scalar = child_x_scalar.compute(arg.reborrow().into()).into_inner();
                let y_scalar = child_y_scalar.compute(arg.reborrow().into()).into_inner();
                let mut edge_vec = Vec::new();

                for x in 0..=8 {
                    for y in 0..=8 {
                        let ratio = 0.5 / 8 as f32;

                        edge_vec.push(SNPoint::new_sawtooth(Point2::new(
                            ratio * x as f32 + (x_scalar * y as f32),
                            ratio * y as f32 + (y_scalar * x as f32),
                        )));
                        edge_vec.push(SNPoint::new_sawtooth(Point2::new(
                            -ratio * x as f32 + (x_scalar * y as f32),
                            ratio * y as f32 + (y_scalar * x as f32),
                        )));
                        edge_vec.push(SNPoint::new_sawtooth(Point2::new(
                            ratio * x as f32 + (x_scalar * y as f32),
                            -ratio * y as f32 + (y_scalar * x as f32),
                        )));
                        edge_vec.push(SNPoint::new_sawtooth(Point2::new(
                            -ratio * x as f32 + (x_scalar * y as f32),
                            -ratio * y as f32 + (y_scalar * x as f32),
                        )));
                    }
                }

                *value = PointSet::new(Arc::new(edge_vec), value.generator);
            }
            PointSetNodes::Line {
                ref mut value,
                child_points,
                child_a,
                child_b,
            } => {
                let point_a = child_a.compute(arg.reborrow().into()).into_inner();
                let point_b = child_b.compute(arg.reborrow().into()).into_inner();

                let point_difference = point_b - point_a;

                let point_count = child_points.compute(arg.reborrow().into()).into_inner();

                let mut edge_vec = Vec::new();
                for i in 0..point_count {
                    let ratio = 1.0 / point_count as f32;

                    edge_vec.push(SNPoint::new(point_a + point_difference * ratio * i as f32));
                }
            }
            _ => {}
        }
    }
}
