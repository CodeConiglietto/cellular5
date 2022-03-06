use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use na::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNComplexNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SNComplex },
    #[mutagen(gen_weight = leaf_node_weight)]
    RotatingAboutCenter { radius: SNFloat, rotation_scalar: SNFloat, normaliser: SFloatNormaliser },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNPoint { child_point: NodeBox<SNPointNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    FromSNFloats {
        child_re: NodeBox<SNFloatNodes>,
        child_im: NodeBox<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    AddNormalised {
        child_a: NodeBox<SNComplexNodes>,
        child_b: NodeBox<SNComplexNodes>,
        child_normaliser: SFloatNormaliser,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    MultiplyNormalised {
        child_a: NodeBox<SNComplexNodes>,
        child_b: NodeBox<SNComplexNodes>,
        child_normaliser: SFloatNormaliser,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromIterativeResult {
        child: NodeBox<IterativeFunctionNodes>,
    },
}

impl Node for SNComplexNodes {
    type Output = SNComplex;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNComplexNodes::*;

        match self {
            Constant { value } => *value,
            RotatingAboutCenter { radius, rotation_scalar, normaliser } => {
                let inner = 
                    Complex::new(
                        f64::from(radius.into_inner()),
                        f64::from(rotation_scalar.into_inner() * compute_arg.coordinate_set.t * PI)
                    ).exp();

                SNComplex::from_snfloats(
                    normaliser.normalise(inner.re as f32), 
                    normaliser.normalise(inner.im as f32)
                )
            }
            FromSNPoint { child_point } => {
                SNComplex::from_snpoint(child_point.compute(compute_arg))
            }
            FromSNFloats { child_re, child_im } => SNComplex::from_snfloats(
                child_re.compute(compute_arg.reborrow()),
                child_im.compute(compute_arg.reborrow()),
            ),
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

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
