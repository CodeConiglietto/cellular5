use std::sync::Arc;

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use nalgebra::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

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
        child_normaliser: Box<SFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Spreading {
        value: PointSet,
        child: Box<UNFloatNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
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
        child_normaliser: Box<SFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    MatrixGrid {
        value: PointSet,
        child_matrix: Box<SNFloatMatrix3Nodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Line {
        value: PointSet,
        child_points: Box<ByteNodes>,
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IterativePolarLine {
        #[mutagen(skip)]
        value: PointSet,
        // TODO Replace child_theta and child_rho with a polar coordinate node when they're implemented
        child_n: Box<ByteNodes>,
        child_theta: Box<AngleNodes>,
        child_rho: Box<UNFloatNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
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
            MatrixGrid { value, .. } => value.clone(),
            Line { value, .. } => value.clone(),
            IterativePolarLine { value, .. } => value.clone(),
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
                child_normaliser,
            } => {
                let normaliser = child_normaliser.compute(arg.reborrow().into());
                let compute_arg = ComArg::from(arg.reborrow());

                value.replace(Arc::new(
                    value
                        .points()
                        .par_iter()
                        .map(|p| {
                            p.normalised_add(
                                child
                                    .compute(compute_arg.clone().replace_coords(p))
                                    .scale_unfloat(UNFloat::new(0.05)), //magic number makes things translate at a not-insane rate
                                normaliser,
                            )
                        })
                        .collect(),
                ));
            }

            PointSetNodes::Spreading {
                ref mut value,
                child,
                child_normaliser,
            } => {
                let normaliser = child_normaliser.compute(arg.reborrow().into());
                let compute_arg = ComArg::from(arg.reborrow());

                value.replace(Arc::new(
                    value
                        .points()
                        .par_iter()
                        .map(|p| {
                            // TODO Attempt to refactor to use normalised_add instead of sawtooth_add
                            p.normalised_add(
                                p.subtract_normalised(value.get_random_point())
                                    .scale_unfloat(
                                        child
                                            .compute(compute_arg.clone().replace_coords(p))
                                            .multiply(UNFloat::new(0.25)),
                                    ),
                                normaliser,
                            )
                        })
                        .collect(),
                ));
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

                value.replace(Arc::new(edge_vec));
            }
            //TODO: Something is funky here, give it a second pass.
            //TODO: Maybe swap this out for a matrix point grid
            PointSetNodes::ShearGrid {
                ref mut value,
                child_x_scalar,
                child_y_scalar,
                child_normaliser,
            } => {
                let x_scalar = child_x_scalar.compute(arg.reborrow().into()).into_inner();
                let y_scalar = child_y_scalar.compute(arg.reborrow().into()).into_inner();
                let normaliser = child_normaliser.compute(arg.reborrow().into());
                let mut edge_vec = Vec::new();

                for x in 0..=8 {
                    for y in 0..=8 {
                        let ratio = 0.5 / 8.0;

                        edge_vec.push(SNPoint::new_normalised(
                            Point2::new(
                                ratio * x as f32 + (x_scalar * y as f32),
                                ratio * y as f32 + (y_scalar * x as f32),
                            ),
                            normaliser,
                        ));
                        edge_vec.push(SNPoint::new_normalised(
                            Point2::new(
                                -ratio * x as f32 + (x_scalar * y as f32),
                                ratio * y as f32 + (y_scalar * x as f32),
                            ),
                            normaliser,
                        ));
                        edge_vec.push(SNPoint::new_normalised(
                            Point2::new(
                                ratio * x as f32 + (x_scalar * y as f32),
                                -ratio * y as f32 + (y_scalar * x as f32),
                            ),
                            normaliser,
                        ));
                        edge_vec.push(SNPoint::new_normalised(
                            Point2::new(
                                -ratio * x as f32 + (x_scalar * y as f32),
                                -ratio * y as f32 + (y_scalar * x as f32),
                            ),
                            normaliser,
                        ));
                    }
                }

                value.replace(Arc::new(edge_vec));
            }
            PointSetNodes::MatrixGrid {
                ref mut value,
                child_matrix,
                child_normaliser,
            } => {
                let normaliser = child_normaliser.compute(arg.reborrow().into());

                let ratio = 0.5 / 8.0;

                let mut edge_vec = Vec::new();

                for x in 0..8 {
                    for y in 0..8 {
                        for sx in &[1.0, -1.0] {
                            for sy in &[1.0, -1.0] {
                                let grid_point =
                                    Point2::new(sx * ratio * x as f32, sy * ratio * y as f32);

                                let compute_arg: ComArg<'_> = arg.reborrow().into();

                                let matrix = child_matrix
                                    .compute(compute_arg.replace_coords(&SNPoint::new(grid_point)));

                                edge_vec.push(SNPoint::new_normalised(
                                    Point2::from_homogeneous(
                                        matrix.into_inner() * grid_point.to_homogeneous(),
                                    )
                                    .unwrap(),
                                    normaliser,
                                ));
                            }
                        }
                    }
                }

                value.replace(Arc::new(edge_vec));
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

                let point_count = child_points
                    .compute(arg.reborrow().into())
                    .into_inner()
                    .max(1);

                let mut edge_vec = Vec::new();
                for i in 0..point_count {
                    let ratio = 1.0 / point_count as f32;

                    edge_vec.push(SNPoint::new(point_a + point_difference * ratio * i as f32));
                }

                value.replace(Arc::new(edge_vec));
            }

            PointSetNodes::IterativePolarLine {
                ref mut value,
                child_n,
                child_rho,
                child_theta,
                child_normaliser,
            } => {
                let n = child_n.compute(arg.reborrow().into()).into_inner().max(1);
                let rho = child_rho.compute(arg.reborrow().into()).into_inner() / n as f32;
                let theta_diff = child_theta.compute(arg.reborrow().into());

                value.replace(Arc::new(
                    (0..n)
                        .scan(
                            (SNPoint::zero(), Angle::ZERO),
                            |(ref mut point, ref mut theta), _| {
                                let new_theta = *theta + theta_diff;
                                let normaliser = child_normaliser.compute(arg.reborrow().into());

                                let new_point = point.normalised_add(
                                    SNPoint::from_snfloats(
                                        SNFloat::new(rho * f32::sin(new_theta.into_inner())),
                                        SNFloat::new(rho * f32::cos(new_theta.into_inner())),
                                    ),
                                    normaliser,
                                );

                                *point = new_point;
                                *theta = new_theta;

                                Some(new_point)
                            },
                        )
                        .collect(),
                ));
            }

            _ => {}
        }
    }
}
