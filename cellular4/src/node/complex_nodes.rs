use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{continuous::*, discrete::*},
    mutagen_args::*,
    node::{constraint_resolver_nodes::*, discrete_nodes::*, point_set_nodes::*, Node},
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNComplexNodes {
    Constant{value: SNComplex},
    FromSNPoint{child_point: Box<SNPointNodes>},
    Add{child_a: Box<SNComplexNodes>, child_b: Box<SNComplexNodes>},
    Multiply{child_a: Box<SNComplexNodes>, child_b: Box<SNComplexNodes>},
}

impl Node for SNComplexNodes {
    type Output = SNComplex;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNComplexNodes::*;

        match self {
            Constant{value} => *value,
            FromSNPoint{child_point} => SNComplex::new_from_snpoint(child_point.compute(compute_arg)),
            Add{child_a: Box<SNComplexNodes>, child_b: Box<SNComplexNodes>},
            Multiply{child_a: Box<SNComplexNodes>, child_b: Box<SNComplexNodes>},
        }
    }
}

impl<'a> Updatable<'a> for SNComplexNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}
