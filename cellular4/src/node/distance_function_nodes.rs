use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{continuous::*, distance_functions::*},
    node::{
        point_nodes::*, Node,
    },
    updatestate::UpdateState,
};

#[derive(Mutatable, Generatable, Serialize, Deserialize, Debug)]
pub enum DistanceFunctionNodes {
    Constant {
        value: DistanceFunction,
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
    },
}

impl<'a> Mutagen<'a> for DistanceFunctionNodes {
    type Arg = UpdateState<'a>;
}
impl Node for DistanceFunctionNodes {
    type Output = UNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use DistanceFunctionNodes::*;

        match self {
            Constant {
                value,
                child_a,
                child_b,
            } => value.calculate(child_a.compute(state), child_b.compute(state)),
        }
    }
}

impl<'a> Updatable<'a> for DistanceFunctionNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for DistanceFunctionNodes {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}
