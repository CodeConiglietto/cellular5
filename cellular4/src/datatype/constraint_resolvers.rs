use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{datatype::continuous::*, mutagen_args::*};

#[derive(
    Clone, Copy, Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug,
)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum SFloatNormaliser {
    Sawtooth,
    Triangle,
    TanH,
    Clamp,
    Fractional,
}

impl SFloatNormaliser {
    pub fn normalise(self, value: f32) -> SNFloat {
        use SFloatNormaliser::*;

        match self {
            Sawtooth => SNFloat::new_sawtooth(value),
            Triangle => SNFloat::new_triangle(value),
            TanH => SNFloat::new_tanh(value),
            Clamp => SNFloat::new_clamped(value),
            Fractional => SNFloat::new_fractional(value),
        }
    }
}

impl<'a> Updatable<'a> for SFloatNormaliser {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, mut _arg: UpdArg<'a>) {}
}

#[derive(
    Clone, Copy, Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug,
)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum UFloatNormaliser {
    //TODO: Add sigmoid function
    Sawtooth,
    Triangle,
    Clamp,
}

impl UFloatNormaliser {
    pub fn normalise(self, value: f32) -> UNFloat {
        use UFloatNormaliser::*;

        match self {
            Sawtooth => UNFloat::new_sawtooth(value),
            Triangle => UNFloat::new_triangle(value),
            Clamp => UNFloat::new_clamped(value),
        }
    }
}

impl<'a> Updatable<'a> for UFloatNormaliser {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, mut _arg: UpdArg<'a>) {}
}
