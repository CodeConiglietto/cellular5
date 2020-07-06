use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively, Reborrow};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{continuous::*, noisefunctions::*},
    mutagen_args::*,
    node::{continuous_nodes::*, Node},
};

#[derive(Mutatable, Generatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum NoiseNodes {
    NoiseFunction {
        noise_function: NoiseFunctions,
        scale_x_child: Box<UNFloatNodes>,
        scale_y_child: Box<UNFloatNodes>,
        scale_t_child: Box<UNFloatNodes>,
    },
}

impl Node for NoiseNodes {
    type Output = SNFloat;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
        match self {
            NoiseNodes::NoiseFunction {
                noise_function,
                scale_x_child,
                scale_y_child,
                scale_t_child,
            } => SNFloat::new_clamped(noise_function.compute(
                compute_arg.coordinate_set.x.into_inner() as f64
                    * scale_x_child.compute(compute_arg.reborrow()).into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                compute_arg.coordinate_set.y.into_inner() as f64
                    * scale_y_child.compute(compute_arg.reborrow()).into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                compute_arg.coordinate_set.t as f64
                    * scale_t_child.compute(compute_arg.reborrow()).into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ) as f32),
        }
    }
}

impl<'a> Updatable<'a> for NoiseNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for NoiseNodes {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}
