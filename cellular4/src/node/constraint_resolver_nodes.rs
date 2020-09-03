use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SFloatNormaliserNodes {
    Random,
    Constant {
        value: SFloatNormaliser,
    },
    IfElse {
        child_predicate: BooleanNodes,
        child_a: SFloatNormaliser,
        child_b: SFloatNormaliser,
    },
}

impl Node for SFloatNormaliserNodes {
    type Output = SFloatNormaliser;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SFloatNormaliserNodes::*;

        match self {
            Random => SFloatNormaliser::generate(()),
            Constant { value } => *value,
            IfElse {
                child_predicate,
                child_a,
                child_b,
            } => {
                if child_predicate.compute(compute_arg.reborrow()).into_inner() {
                    *child_a
                } else {
                    *child_b
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for SFloatNormaliserNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, mut _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum UFloatNormaliserNodes {
    Random,
    Constant {
        value: UFloatNormaliser,
    },
    IfElse {
        child_predicate: BooleanNodes,
        child_a: UFloatNormaliser,
        child_b: UFloatNormaliser,
    },
}

impl Node for UFloatNormaliserNodes {
    type Output = UFloatNormaliser;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use UFloatNormaliserNodes::*;

        match self {
            Random => UFloatNormaliser::generate(()),
            Constant { value } => *value,
            IfElse {
                child_predicate,
                child_a,
                child_b,
            } => {
                if child_predicate.compute(compute_arg.reborrow()).into_inner() {
                    *child_a
                } else {
                    *child_b
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for UFloatNormaliserNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, mut _arg: UpdArg<'a>) {}
}
