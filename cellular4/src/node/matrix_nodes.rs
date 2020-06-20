use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::matrices::*,
    node::{continuous_nodes::*, mutagen_functions::*, Node},
    updatestate::*,
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum SNFloatMatrix3Nodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Identity,
    #[mutagen(gen_weight = branch_node_weight)]
    Translation {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    Rotation { child: Box<AngleNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    Scaling {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Shear {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<SNFloatMatrix3Nodes>,
        child_b: Box<SNFloatMatrix3Nodes>,
    },
}

impl<'a> Mutagen<'a> for SNFloatMatrix3Nodes {
    type Arg = UpdateState<'a>;
}
impl Node for SNFloatMatrix3Nodes {
    type Output = SNFloatMatrix3;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNFloatMatrix3Nodes::*;

        match self {
            Identity => SNFloatMatrix3::identity(),
            Translation { child_a, child_b } => {
                SNFloatMatrix3::new_translation(child_a.compute(state), child_b.compute(state))
            }
            Rotation { child } => SNFloatMatrix3::new_rotation(child.compute(state)),
            Scaling { child_a, child_b } => {
                SNFloatMatrix3::new_scaling(child_a.compute(state), child_b.compute(state))
            }
            Shear { child_a, child_b } => {
                SNFloatMatrix3::new_shear(child_a.compute(state), child_b.compute(state))
            }
            Multiply { child_a, child_b } => {
                child_a.compute(state).multiply(child_b.compute(state))
            }
        }
    }
}

impl<'a> Updatable<'a> for SNFloatMatrix3Nodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}
