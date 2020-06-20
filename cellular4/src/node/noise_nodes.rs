use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{continuous::*, noisefunctions::*},
    node::{continuous_nodes::*, mutagen_functions::*, Node},
    updatestate::*,
};

#[derive(Mutatable, Generatable, Deserialize, Serialize, Debug)]
pub enum NoiseNodes {
    NoiseFunction {
        noise_function: NoiseFunctions,
        scale_x_child: Box<UNFloatNodes>,
        scale_y_child: Box<UNFloatNodes>,
        scale_t_child: Box<UNFloatNodes>,
    },
}

impl<'a> Mutagen<'a> for NoiseNodes {
    type Arg = UpdateState<'a>;
}

impl Node for NoiseNodes {
    type Output = SNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        match self {
            NoiseNodes::NoiseFunction {
                noise_function,
                scale_x_child,
                scale_y_child,
                scale_t_child,
            } => SNFloat::new_clamped(noise_function.compute(
                state.coordinate_set.x.into_inner() as f64
                    * scale_x_child.compute(state).into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * scale_y_child.compute(state).into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * scale_t_child.compute(state).into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ) as f32),
        }
    }
}

impl<'a> Updatable<'a> for NoiseNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for NoiseNodes {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}
