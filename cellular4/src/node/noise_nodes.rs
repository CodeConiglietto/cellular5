use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Mutatable, Generatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum NoiseNodes {
    NoiseFunction {
        noise_function: NoiseFunctions,
        scale_x_child: Box<NibbleNodes>,
        scale_y_child: Box<NibbleNodes>,
        scale_t_child: Box<NibbleNodes>,
    },
    // IterativeMatrixNoiseFunction {//TODO: finish
    //     noise_function: Box<UNFloatNodes>,
    //     child_matrix: Box<SNFloatMatrix3Nodes>,
    //     iterated_matrix: SNFloatMatrix3,
    //     #[mutagen(skip)]
    //     offset_xy: Point2<f32>,
    //     child_offset_t: Box<SNFloatNodes>,
    //     #[mutagen(skip)]
    //     t_offset: f32,
    // },
}

impl Node for NoiseNodes {
    type Output = SNFloat;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        match self {
            NoiseNodes::NoiseFunction {
                noise_function,
                scale_x_child,
                scale_y_child,
                scale_t_child,
            } => SNFloat::new_clamped(
                noise_function.compute(
                    compute_arg.coordinate_set.x.into_inner() as f64
                        * scale_x_child
                            .compute(compute_arg.reborrow())
                            .into_inner() as f64,
                    compute_arg.coordinate_set.y.into_inner() as f64
                        * scale_y_child
                            .compute(compute_arg.reborrow())
                            .into_inner() as f64,
                    compute_arg.coordinate_set.t as f64
                        * scale_t_child.compute(compute_arg.reborrow()).into_inner() as f64,
                ) as f32,
            ),
            // NoiseNodes::IterativeMatrixNoiseFunction {
            //     noise_function,
            //     child_matrix,
            //     iterated_matrix,
            //     child_offset_xy,
            //     child_offset_t,
            //     child_scale_t,
            // } =>
            // {
            //     let transformed_point = Point2::from_homogeneous(child_offset_xy.to_homogeneous() * child_matrix.into_inner());

            //     SNFloat::new_clamped(
            //         noise_function.compute(
            //             compute_arg.coordinate_set.x.into_inner() as f64
            //                 * scale_x_child
            //                     .compute(compute_arg.reborrow())
            //                     .into_inner()
            //                     .powf(2.0) as f64
            //                 * CONSTS.noise_x_scale_factor,
            //             compute_arg.coordinate_set.y.into_inner() as f64
            //                 * scale_y_child
            //                     .compute(compute_arg.reborrow())
            //                     .into_inner()
            //                     .powf(2.0) as f64
            //                 * CONSTS.noise_y_scale_factor,
            //             compute_arg.coordinate_set.t as f64
            //                 * scale_t_child.compute(compute_arg.reborrow()).into_inner() as f64
            //                 * CONSTS.noise_t_scale_factor,
            //         ) as f32,
            //     )
            // }
        }
    }
}

impl<'a> Updatable<'a> for NoiseNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for NoiseNodes {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}
