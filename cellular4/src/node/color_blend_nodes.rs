use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{colors::*, continuous::*},
    mutagen_args::*,
    node::{
        color_nodes::*, continuous_nodes::*, coord_map_nodes::*, discrete_nodes::*,
        mutagen_functions::*, Node,
    },
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum ColorBlendNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Gray,

    #[mutagen(gen_weight = pipe_node_weight)]
    Invert { child: Box<FloatColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Dissolve {
        color_a: Box<FloatColorNodes>,
        color_b: Box<FloatColorNodes>,
        value: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Overlay {
        color_a: Box<FloatColorNodes>,
        color_b: Box<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ScreenDodge {
        color_a: Box<FloatColorNodes>,
        color_b: Box<FloatColorNodes>,
    },

    // #[mutagen(gen_weight = branch_node_weight)]
    // ColorDodge {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // LinearDodge {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Multiply {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // ColorBurn {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // LinearBurn {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // VividLight {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // LinearLight {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Subtract {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Divide {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Lerp {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<ColorBlendNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<ColorBlendNodes>,
        child_b: Box<ColorBlendNodes>,
    },
}

impl Node for ColorBlendNodes {
    type Output = FloatColor;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
        use ColorBlendNodes::*;

        match self {
            Gray => FloatColor {
                r: UNFloat::new_unchecked(0.5),
                g: UNFloat::new_unchecked(0.5),
                b: UNFloat::new_unchecked(0.5),
                a: UNFloat::ONE,
            },
            Invert { child } => {
                let col = child.compute(compute_arg);
                FloatColor {
                    r: UNFloat::new(1.0 - col.r.into_inner()),
                    g: UNFloat::new(1.0 - col.g.into_inner()),
                    b: UNFloat::new(1.0 - col.b.into_inner()),
                    a: UNFloat::new(1.0 - col.a.into_inner()),
                }
            }
            Dissolve {
                color_a,
                color_b,
                value,
            } => {
                if UNFloat::random(&mut rand::thread_rng()).into_inner()
                    < value.compute(compute_arg).into_inner()
                {
                    color_a.compute(compute_arg)
                } else {
                    color_b.compute(compute_arg)
                }
            }
            Overlay { color_a, color_b } => {
                let a = color_a.compute(compute_arg);
                let ar = a.r.into_inner();
                let ag = a.g.into_inner();
                let ab = a.b.into_inner();

                let b = color_b.compute(compute_arg);
                let br = b.r.into_inner();
                let bg = b.g.into_inner();
                let bb = b.b.into_inner();

                FloatColor {
                    r: UNFloat::new(if ar < 0.5 {
                        (2.0 * ar * br).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - ar) * (1.0 - br)))
                    }),
                    g: UNFloat::new(if ag < 0.5 {
                        (2.0 * ag * bg).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - ag) * (1.0 - bg)))
                    }),
                    b: UNFloat::new(if ab < 0.5 {
                        (2.0 * ab * bb).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - ab) * (1.0 - bb)))
                    }),
                    a: UNFloat::ONE,
                }
            }
            ScreenDodge { color_a, color_b } => {
                let a = color_a.compute(compute_arg);
                let ar = a.r.into_inner();
                let ag = a.g.into_inner();
                let ab = a.b.into_inner();

                let b = color_b.compute(compute_arg);
                let br = b.r.into_inner();
                let bg = b.g.into_inner();
                let bb = b.b.into_inner();

                FloatColor {
                    r: UNFloat::new(1.0 - ((1.0 - ar) * (1.0 - br))),
                    g: UNFloat::new(1.0 - ((1.0 - ag) * (1.0 - bg))),
                    b: UNFloat::new(1.0 - ((1.0 - ab) * (1.0 - bb))),
                    a: UNFloat::ONE,
                }
            }
            // ColorDodge {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // LinearDodge {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // Multiply {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // ColorBurn {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // LinearBurn {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // VividLight {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // LinearLight {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // Subtract {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // Divide {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            // Lerp {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(compute_arg).into_inner() {color_a.compute(compute_arg)}else{color_b.compute(compute_arg)}},
            ModifyState { child, child_state } => child.compute(&UpdateState {
                coordinate_set: child_state.compute(compute_arg),
                ..*compute_arg
            },
            compute_arg),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg).into_inner() {
                    child_a.compute(compute_arg)
                } else {
                    child_b.compute(compute_arg)
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for ColorBlendNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}
