use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::matrices::*,
    mutagen_args::*,
    node::{continuous_nodes::*, mutagen_functions::*, Node},
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
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

impl Node for SNFloatMatrix3Nodes {
    type Output = SNFloatMatrix3;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
        use SNFloatMatrix3Nodes::*;

        match self {
            Identity => SNFloatMatrix3::identity(),
            Translation { child_a, child_b } => SNFloatMatrix3::new_translation(
                child_a.compute(compute_arg),
                child_b.compute(compute_arg),
            ),
            Rotation { child } => SNFloatMatrix3::new_rotation(child.compute(compute_arg)),
            Scaling { child_a, child_b } => SNFloatMatrix3::new_scaling(
                child_a.compute(compute_arg),
                child_b.compute(compute_arg),
            ),
            Shear { child_a, child_b } => SNFloatMatrix3::new_shear(
                child_a.compute(compute_arg),
                child_b.compute(compute_arg),
            ),
            Multiply { child_a, child_b } => child_a
                .compute(compute_arg)
                .multiply(child_b.compute(compute_arg)),
        }
    }
}

impl<'a> Updatable<'a> for SNFloatMatrix3Nodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}
