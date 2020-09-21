use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNFloatMatrix3Nodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Identity,
    #[mutagen(gen_weight = branch_node_weight)]
    Translation {
        child_a: NodeBox<SNFloatNodes>,
        child_b: NodeBox<SNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    Rotation { child: NodeBox<AngleNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    Scaling {
        child_a: NodeBox<SNFloatNodes>,
        child_b: NodeBox<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Shear {
        child_a: NodeBox<SNFloatNodes>,
        child_b: NodeBox<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: NodeBox<SNFloatMatrix3Nodes>,
        child_b: NodeBox<SNFloatMatrix3Nodes>,
    },
}

impl Node for SNFloatMatrix3Nodes {
    type Output = SNFloatMatrix3;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNFloatMatrix3Nodes::*;

        match self {
            Identity => SNFloatMatrix3::identity(),
            Translation { child_a, child_b } => SNFloatMatrix3::new_translation(
                child_a.compute(compute_arg.reborrow()),
                child_b.compute(compute_arg.reborrow()),
            ),
            Rotation { child } => {
                SNFloatMatrix3::new_rotation(child.compute(compute_arg.reborrow()))
            }
            Scaling { child_a, child_b } => SNFloatMatrix3::new_scaling(
                child_a.compute(compute_arg.reborrow()),
                child_b.compute(compute_arg.reborrow()),
            ),
            Shear { child_a, child_b } => SNFloatMatrix3::new_shear(
                child_a.compute(compute_arg.reborrow()),
                child_b.compute(compute_arg.reborrow()),
            ),
            Multiply { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .multiply(child_b.compute(compute_arg.reborrow())),
        }
    }
}

impl<'a> Updatable<'a> for SNFloatMatrix3Nodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
