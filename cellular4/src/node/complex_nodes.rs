use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNComplexNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SNComplex },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNPoint { child_point: Box<SNPointNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    FromSNFloats { child_re: Box<SNFloatNodes>, child_im: Box<SNFloatNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    AddNormalised {
        child_a: Box<SNComplexNodes>,
        child_b: Box<SNComplexNodes>,
        child_normaliser: SFloatNormaliser,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    MultiplyNormalised {
        child_a: Box<SNComplexNodes>,
        child_b: Box<SNComplexNodes>,
        child_normaliser: SFloatNormaliser,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromIterativeResult { child: Box<IterativeFunctionNodes> },
}

impl Node for SNComplexNodes {
    type Output = SNComplex;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNComplexNodes::*;

        match self {
            Constant { value } => *value,
            FromSNPoint { child_point } => {
                SNComplex::from_snpoint(child_point.compute(compute_arg))
            }
            FromSNFloats { child_re, child_im } => {
                SNComplex::from_snfloats(child_re.compute(compute_arg.reborrow()), child_im.compute(compute_arg.reborrow()))
            }
            AddNormalised {
                child_a,
                child_b,
                child_normaliser,
            } => SNComplex::new_normalised(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    + child_b.compute(compute_arg.reborrow()).into_inner(),
                *child_normaliser,
            ),
            MultiplyNormalised {
                child_a,
                child_b,
                child_normaliser,
            } => SNComplex::new_normalised(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    * child_b.compute(compute_arg.reborrow()).into_inner(),
                *child_normaliser,
            ),
            FromIterativeResult { child } => child.compute(compute_arg).z_final,
        }
    }
}

impl<'a> Updatable<'a> for SNComplexNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}
