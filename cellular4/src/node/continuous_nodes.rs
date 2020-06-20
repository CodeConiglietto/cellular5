use std::f64::consts::PI;

use bresenham::Bresenham;
use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{buffers::*, continuous::*, points::*},
    node::{
        color_nodes::*, coord_map_nodes::*, discrete_nodes::*, distance_function_nodes::*,
        mutagen_functions::*, noise_nodes::*, point_nodes::*, Node,
    },
    updatestate::*,
    util::*,
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum AngleNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,

    #[mutagen(gen_weight = pipe_node_weight)]
    ArcSin { theta: Box<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    ArcCos { theta: Box<SNFloatNodes> },

    // #[mutagen(gen_weight = leaf_node_weight)]
    // #[mutagen(mut_reroll = 0.9)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCoordinate,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: Angle },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNPoint { child: Box<SNPointNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat { child: Box<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<AngleNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<AngleNodes>,
        child_b: Box<AngleNodes>,
    },
}

impl<'a> Mutagen<'a> for AngleNodes {
    type Arg = UpdateState<'a>;
}
impl Node for AngleNodes {
    type Output = Angle;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use AngleNodes::*;

        match self {
            FromGametic => Angle::new(state.coordinate_set.t * 0.1),
            ArcSin { theta } => Angle::new(f32::asin(theta.compute(state).into_inner())),
            ArcCos { theta } => Angle::new(f32::acos(theta.compute(state).into_inner())),
            FromCoordinate => Angle::new(f32::atan2(
                -state.coordinate_set.x.into_inner(),
                state.coordinate_set.y.into_inner(),
            )),
            // Random => Angle::generate(state),
            Constant { value } => *value,
            FromSNPoint { child } => child.compute(state).to_angle(),
            FromSNFloat { child } => child.compute(state).to_angle(),
            FromUNFloat { child } => child.compute(state).to_angle(),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(state).into_inner() {
                    child_a.compute(state)
                } else {
                    child_b.compute(state)
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for AngleNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum SNFloatNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Sin { child: Box<AngleNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    Cos { child: Box<AngleNodes> },

    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SNFloat },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromAngle { child: Box<AngleNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Abs { child: Box<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    Invert { child: Box<SNFloatNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    XRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    YRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,

    #[mutagen(gen_weight = leaf_node_weight)]
    NoiseFunction { child: Box<NoiseNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    SubDivide {
        child_a: Box<SNFloatNodes>,
        child_b: Box<NibbleNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<SNFloatNodes>,
        child_state: Box<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SawtoothAdd {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TriangleAdd {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
}

impl<'a> Mutagen<'a> for SNFloatNodes {
    type Arg = UpdateState<'a>;
}
impl Node for SNFloatNodes {
    type Output = SNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNFloatNodes::*;

        match self {
            Sin { child } => SNFloat::new(f32::sin(child.compute(state).into_inner())),
            Cos { child } => SNFloat::new(f32::cos(child.compute(state).into_inner())),
            // Random => SNFloat::generate(state),
            FromAngle { child } => child.compute(state).to_signed(),
            FromUNFloat { child } => child.compute(state).to_signed(),
            Constant { value } => *value,
            Multiply { child_a, child_b } => SNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            Abs { child } => SNFloat::new(child.compute(state).into_inner().abs()),
            Invert { child } => SNFloat::new(child.compute(state).into_inner() * -1.0),
            XRatio => state.coordinate_set.x,
            YRatio => state.coordinate_set.y,
            FromGametic => {
                SNFloat::new((state.coordinate_set.t - state.coordinate_set.t.floor()) * 2.0 - 1.0)
            }
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            NoiseFunction { child } => child.compute(state),
            SubDivide { child_a, child_b } => {
                child_a.compute(state).subdivide(child_b.compute(state))
            }
            SawtoothAdd { child_a, child_b } => {
                child_a.compute(state).sawtooth_add(child_b.compute(state))
            }
            TriangleAdd { child_a, child_b } => {
                child_a.compute(state).triangle_add(child_b.compute(state))
            }
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(state).into_inner() {
                    child_a.compute(state)
                } else {
                    child_b.compute(state)
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for SNFloatNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum UNFloatNodes {
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: UNFloat },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromAngle { child: Box<AngleNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat { child: Box<SNFloatNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    AbsSNFloat { child: Box<SNFloatNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    SquareSNFloat { child: Box<SNFloatNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    CircularAdd {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    InvertNormalised { child: Box<UNFloatNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    ColorAverage {
        child: Box<FloatColorNodes>,
        child_r: Box<BooleanNodes>,
        child_g: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
        child_a: Box<BooleanNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentH { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGameticTriangle,
    #[mutagen(gen_weight = pipe_node_weight)]
    EscapeTimeSystem {
        child_power: Box<NibbleNodes>,
        child_power_ratio: Box<UNFloatNodes>,
        child_offset: Box<SNPointNodes>,
        child_scale: Box<SNPointNodes>,
        child_iterations: Box<ByteNodes>,
        child_exponentiate: Box<BooleanNodes>,
    },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // LastRotation,
    #[mutagen(gen_weight = branch_node_weight)]
    SubDivideSawtooth {
        child_a: Box<UNFloatNodes>,
        child_b: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    SubDivideTriangle {
        child_a: Box<UNFloatNodes>,
        child_b: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    Distance {
        child_function: Box<DistanceFunctionNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Average {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<UNFloatNodes>,
        child_state: Box<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SawtoothAdd {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TriangleAdd {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
}

impl<'a> Mutagen<'a> for UNFloatNodes {
    type Arg = UpdateState<'a>;
}

impl Node for UNFloatNodes {
    type Output = UNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use UNFloatNodes::*;

        match self {
            // Random => UNFloat::generate(state),
            Constant { value } => *value,
            FromAngle { child } => child.compute(state).to_unsigned(),
            FromSNFloat { child } => child.compute(state).to_unsigned(),
            AbsSNFloat { child } => UNFloat::new(child.compute(state).into_inner().abs()),
            SquareSNFloat { child } => UNFloat::new(child.compute(state).into_inner().powf(2.0)),
            Multiply { child_a, child_b } => UNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            CircularAdd { child_a, child_b } => {
                let value =
                    child_a.compute(state).into_inner() + child_b.compute(state).into_inner();
                UNFloat::new(value - (value.floor()))
            }
            InvertNormalised { child } => UNFloat::new(1.0 - child.compute(state).into_inner()),
            ColorAverage {
                child,
                child_r,
                child_g,
                child_b,
                child_a,
            } => {
                let color = child.compute(state);
                let r = child_r.compute(state).into_inner();
                let g = child_g.compute(state).into_inner();
                let b = child_b.compute(state).into_inner();
                let a = child_a.compute(state).into_inner();

                let mut components_total = 0;
                let mut value_total = 0.0;
                if r {
                    components_total += 1;
                    value_total += color.r.into_inner();
                }
                if g {
                    components_total += 1;
                    value_total += color.r.into_inner();
                }
                if b {
                    components_total += 1;
                    value_total += color.r.into_inner();
                }
                if a {
                    components_total += 1;
                    value_total += color.r.into_inner();
                }

                if components_total == 0 {
                    UNFloat::ZERO
                } else {
                    UNFloat::new(value_total / components_total as f32)
                }
            }
            ColorComponentH { child } => child.compute(state).get_hue_unfloat(),
            FromGametic => state.coordinate_set.get_unfloat_t(),
            FromGameticTriangle => UNFloat::new_triangle(state.coordinate_set.t * 0.1),
            EscapeTimeSystem {
                child_power,
                child_power_ratio,
                child_offset,
                child_scale,
                child_iterations,
                child_exponentiate,
            } => {
                let power = f64::from(
                    (1 + child_power.compute(state).into_inner()) as f32
                        * UNFloat::new_triangle(
                            child_power_ratio.compute(state).into_inner() * 2.0,
                        )
                        .into_inner(),
                );
                let offset = child_offset.compute(state).into_inner();
                let scale = child_scale.compute(state).into_inner();
                let iterations = 1 + child_iterations.compute(state).into_inner() / 8;

                // x and y are swapped intentionally
                let c = Complex::new(
                    f64::from(2.0 * scale.y * state.coordinate_set.y.into_inner()),
                    f64::from(2.0 * scale.x * state.coordinate_set.x.into_inner()),
                );

                let z_offset =
                    // Complex::new(0.0, 0.0);
                    if child_exponentiate.compute(state).into_inner()
                    {
                        Complex::new(
                            f64::from(2.0 * scale.y) *
                            f64::from(offset.y),
                            f64::from(2.0 * scale.x) *
                            f64::from(offset.x) * PI
                        ).exp()
                    }else{
                        Complex::new(
                            f64::from(2.0 * scale.y) *
                            f64::from(offset.y),
                            f64::from(2.0 * scale.x) *
                            f64::from(offset.x)
                        )
                    };

                let (z_final, escape) = escape_time_system(
                    c,
                    iterations as usize,
                    |z, i| z.powf(power) + z_offset * i as f64,
                    |z, i| z.norm_sqr() > 4.0,
                );

                UNFloat::new(((escape as f32 / iterations as f32) * 4.0).fract())
            }
            SubDivideSawtooth { child_a, child_b } => child_a
                .compute(state)
                .subdivide_sawtooth(child_b.compute(state)),
            SubDivideTriangle { child_a, child_b } => child_a
                .compute(state)
                .subdivide_triangle(child_b.compute(state)),
            Distance { child_function } => child_function.compute(state),
            Average { child_a, child_b } => UNFloat::new(
                (child_a.compute(state).into_inner() + child_b.compute(state).into_inner()) / 2.0,
            ),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            SawtoothAdd { child_a, child_b } => {
                child_a.compute(state).sawtooth_add(child_b.compute(state))
            }
            TriangleAdd { child_a, child_b } => {
                child_a.compute(state).triangle_add(child_b.compute(state))
            }
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(state).into_inner() {
                    child_a.compute(state)
                } else {
                    child_b.compute(state)
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for UNFloatNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        use UNFloatNodes::*;

        match self {
            _ => {}
        }
    }
}
