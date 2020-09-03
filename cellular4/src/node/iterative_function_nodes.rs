use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::prelude::*;

use nalgebra::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum IterativeFunctionNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: IterativeResult },
    #[mutagen(gen_weight = pipe_node_weight)]
    Invert { child: Box<IterativeFunctionNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    EscapeTimeSystem {
        child_power: Box<NibbleNodes>,
        child_power_ratio: Box<UNFloatNodes>,
        child_offset: Box<SNPointNodes>,
        child_scale: Box<SNPointNodes>,
        child_iterations: Box<ByteNodes>,
        child_exponentiate: Box<BooleanNodes>,
        child_distance_function: DistanceFunction,
        child_exit_normaliser: Box<SFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IterativeMatrix {
        child_matrix: Box<SNFloatMatrix3Nodes>,
        child_iterations: Box<ByteNodes>,
        child_exit_condition: Box<BooleanNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
        child_exit_normaliser: Box<SFloatNormaliserNodes>,
    },
}

impl Node for IterativeFunctionNodes {
    type Output = IterativeResult;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use IterativeFunctionNodes::*;

        match self {
            Constant { value } => *value,
            Invert { child } => {
                let val = child.compute(compute_arg);

                IterativeResult::new(val.z_final, val.iter_final.invert_wrapped())
            }
            EscapeTimeSystem {
                child_power,
                child_power_ratio,
                child_offset,
                child_scale,
                child_iterations,
                child_exponentiate,
                child_distance_function,
                child_exit_normaliser,
            } => {
                let power = f64::from(
                    (1 + child_power.compute(compute_arg.reborrow()).into_inner()) as f32
                        * UNFloat::new_triangle(
                            child_power_ratio
                                .compute(compute_arg.reborrow())
                                .into_inner()
                                * 2.0,
                        )
                        .into_inner(),
                );
                let offset = child_offset.compute(compute_arg.reborrow()).into_inner();
                let scale = child_scale.compute(compute_arg.reborrow()).into_inner();
                let iterations = 1 + child_iterations
                    .compute(compute_arg.reborrow())
                    .into_inner()
                    / 4;

                // x and y are swapped intentionally
                let c = Complex::new(
                    f64::from(2.0 * scale.y * compute_arg.coordinate_set.y.into_inner()),
                    f64::from(2.0 * scale.x * compute_arg.coordinate_set.x.into_inner()),
                );

                let z_offset =
                    // Complex::new(0.0, 0.0);
                    if child_exponentiate.compute(compute_arg.reborrow()).into_inner()
                    {
                        Complex::new(
                            f64::from(2.0 * scale.y) *
                            f64::from(offset.y),
                            f64::from(2.0 * scale.x) *
                            f64::from(offset.x * PI)
                        ).exp()
                    }else{
                        Complex::new(
                            f64::from(2.0 * scale.y) *
                            f64::from(offset.y),
                            f64::from(2.0 * scale.x) *
                            f64::from(offset.x)
                        )
                    };

                let (z_final, _escape) = escape_time_system(
                    c,
                    iterations as usize,
                    |z, i| z.powf(power) + z_offset * i as f64,
                    |z, _i| {
                        child_distance_function.calculate_point2(
                            Point2::origin(),
                            Point2::new(z.re as f32, z.im as f32),
                        ) > 2.0
                    },
                );

                IterativeResult::new(
                    SNComplex::new_normalised(
                        z_final,
                        child_exit_normaliser.compute(compute_arg.reborrow()),
                    ),
                    Byte::new(iterations),
                )
            }
            IterativeMatrix {
                child_matrix,
                child_iterations,
                child_exit_condition,
                child_normaliser,
                child_exit_normaliser,
            } => {
                let matrix = child_matrix.compute(compute_arg.reborrow()).into_inner();

                let iterations = 1 + child_iterations
                    .compute(compute_arg.reborrow())
                    .into_inner()
                    / 4;

                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                // x and y are swapped intentionally
                let c = Complex::new(
                    f64::from(compute_arg.coordinate_set.y.into_inner()),
                    f64::from(compute_arg.coordinate_set.x.into_inner()),
                );

                let (z_final, _escape) = escape_time_system(
                    c,
                    iterations as usize,
                    |z, _i| {
                        let new_point =
                            matrix.transform_point(&Point2::new(z.re as f32, z.im as f32));
                        Complex::new(new_point.x as f64, new_point.y as f64)
                    },
                    |z, _i| {
                        child_exit_condition
                            .compute(compute_arg.reborrow().replace_coords(
                                &SNPoint::new_normalised(
                                    Point2::new(z.re as f32, z.im as f32),
                                    normaliser,
                                ),
                            ))
                            .into_inner()
                    },
                );

                IterativeResult::new(
                    SNComplex::new_normalised(
                        z_final,
                        child_exit_normaliser.compute(compute_arg.reborrow()),
                    ),
                    Byte::new(iterations),
                )
            }
        }
    }
}

impl<'a> Updatable<'a> for IterativeFunctionNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}
