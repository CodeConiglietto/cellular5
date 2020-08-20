use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{datatype::continuous::*, mutagen_args::*};

#[derive(
    Clone, Copy, Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug,
)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum SNFloatNormaliser {
    Sawtooth,
    Triangle,
    Sigmoid,
    Clamp,
    Fractional,
}

impl SNFloatNormaliser {
    pub fn normalise(self, value: f32) -> SNFloat {
        use SNFloatNormaliser::*;

        match self {
            Sawtooth => SNFloat::new_sawtooth(value),
            Triangle => SNFloat::new_triangle(value),
            Sigmoid => SNFloat::new_tanh(value),
            Clamp => SNFloat::new_clamped(value),
            Fractional => SNFloat::new_fractional(value),
        }
    }
}

impl<'a> Updatable<'a> for SNFloatNormaliser {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, mut _arg: UpdArg<'a>) {}
}