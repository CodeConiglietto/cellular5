use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(
    Generatable,
    Mutatable,
    UpdatableRecursively,
    Deserialize,
    Serialize,
    Clone,
    Copy,
    Debug,
    Default,
)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub struct IterativeResult {
    pub z_final: SNComplex,
    pub iter_final: Byte,
}

impl IterativeResult {
    pub fn new(z_final: SNComplex, iter_final: Byte) -> Self {
        Self {
            z_final,
            iter_final,
        }
    }
}

impl<'a> Updatable<'a> for IterativeResult {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
