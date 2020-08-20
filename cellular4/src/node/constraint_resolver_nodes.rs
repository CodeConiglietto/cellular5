use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::constraint_resolvers::*,
    mutagen_args::*,
    node::{discrete_nodes::*, Node},
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNFloatNormaliserNodes {
    Random,
    Constant {
        value: SNFloatNormaliser,
    },
    IfElse {
        child_predicate: BooleanNodes,
        child_a: SNFloatNormaliser,
        child_b: SNFloatNormaliser,
    },
}

impl Node for SNFloatNormaliserNodes {
    type Output = SNFloatNormaliser;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNFloatNormaliserNodes::*;

        match self {
            Random => SNFloatNormaliser::generate(()),
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

impl<'a> Updatable<'a> for SNFloatNormaliserNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, mut _arg: UpdArg<'a>) {}
}
