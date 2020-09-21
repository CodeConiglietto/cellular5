use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Mutatable, Generatable, UpdatableRecursively, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum DistanceFunctionNodes {
    Constant {
        value: DistanceFunction,
        child_a: NodeBox<SNPointNodes>,
        child_b: NodeBox<SNPointNodes>,
        child_normaliser: NodeBox<UFloatNormaliserNodes>,
    },
}

impl Node for DistanceFunctionNodes {
    type Output = UNFloat;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use DistanceFunctionNodes::*;

        match self {
            Constant {
                value,
                child_a,
                child_b,
                child_normaliser,
            } => value.calculate_normalised(
                child_a.compute(compute_arg.reborrow()),
                child_b.compute(compute_arg.reborrow()),
                child_normaliser.compute(compute_arg.reborrow()),
            ),
        }
    }
}

impl<'a> Updatable<'a> for DistanceFunctionNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
