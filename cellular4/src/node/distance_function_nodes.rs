use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively, Reborrow};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{continuous::*, distance_functions::*},
    mutagen_args::*,
    node::{point_nodes::*, Node},
};

#[derive(Mutatable, Generatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum DistanceFunctionNodes {
    Constant {
        value: DistanceFunction,
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
    },
}

impl Node for DistanceFunctionNodes {
    type Output = UNFloat;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
        use DistanceFunctionNodes::*;

        match self {
            Constant {
                value,
                child_a,
                child_b,
            } => value.calculate(child_a.compute(compute_arg.reborrow()), child_b.compute(compute_arg.reborrow())),
        }
    }
}

impl<'a> Updatable<'a> for DistanceFunctionNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for DistanceFunctionNodes {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}
