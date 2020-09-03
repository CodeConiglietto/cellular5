use std::f64::consts::PI;

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use nalgebra::*;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{continuous::*, distance_functions::*, points::*},
    mutagen_args::*,
    node::{
        color_nodes::*, complex_nodes::*, constraint_resolver_nodes::*, coord_map_nodes::*,
        discrete_nodes::*, distance_function_nodes::*, matrix_nodes::*, mutagen_functions::*,
        noise_nodes::*, point_nodes::*, Node,
    },
    util::*,
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
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

impl Node for AngleNodes {
    type Output = Angle;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use AngleNodes::*;

        match self {
            FromGametic => Angle::new(compute_arg.coordinate_set.t * 0.1),
            ArcSin { theta } => Angle::new(f32::asin(
                theta.compute(compute_arg.reborrow()).into_inner(),
            )),
            ArcCos { theta } => Angle::new(f32::acos(
                theta.compute(compute_arg.reborrow()).into_inner(),
            )),
            FromCoordinate => Angle::new(f32::atan2(
                -compute_arg.coordinate_set.x.into_inner(),
                compute_arg.coordinate_set.y.into_inner(),
            )),
            // Random => Angle::generate(state),
            Constant { value } => *value,
            FromSNPoint { child } => child.compute(compute_arg.reborrow()).to_angle(),
            FromSNFloat { child } => child.compute(compute_arg.reborrow()).to_angle(),
            FromUNFloat { child } => child.compute(compute_arg.reborrow()).to_angle(),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow()),
                ..compute_arg.reborrow()
            }),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for AngleNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNFloatNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Sin {
        child: Box<AngleNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Cos {
        child: Box<AngleNodes>,
    },

    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant {
        value: SNFloat,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromAngle {
        child: Box<AngleNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat {
        child: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Abs {
        child: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Invert {
        child: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    XRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    YRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,

    #[mutagen(gen_weight = leaf_node_weight)]
    NoiseFunction {
        child: Box<NoiseNodes>,
    },

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
    NormalisedAdd {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },

    ComplexReal {
        child_complex: Box<SNComplexNodes>,
    },

    ComplexImaginary {
        child_complex: Box<SNComplexNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
}

impl Node for SNFloatNodes {
    type Output = SNFloat;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNFloatNodes::*;

        match self {
            Sin { child } => {
                SNFloat::new(f32::sin(child.compute(compute_arg.reborrow()).into_inner()))
            }
            Cos { child } => {
                SNFloat::new(f32::cos(child.compute(compute_arg.reborrow()).into_inner()))
            }
            // Random => SNFloat::generate(state),
            FromAngle { child } => child.compute(compute_arg.reborrow()).to_signed(),
            FromUNFloat { child } => child.compute(compute_arg.reborrow()).to_signed(),
            Constant { value } => *value,
            Multiply { child_a, child_b } => SNFloat::new(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    * child_b.compute(compute_arg.reborrow()).into_inner(),
            ),
            Abs { child } => SNFloat::new(child.compute(compute_arg.reborrow()).into_inner().abs()),
            Invert { child } => {
                SNFloat::new(child.compute(compute_arg.reborrow()).into_inner() * -1.0)
            }
            XRatio => compute_arg.coordinate_set.x,
            YRatio => compute_arg.coordinate_set.y,
            FromGametic => SNFloat::new(
                (compute_arg.coordinate_set.t - compute_arg.coordinate_set.t.floor()) * 2.0 - 1.0,
            ),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow()),
                ..compute_arg.reborrow()
            }),
            NoiseFunction { child } => child.compute(compute_arg.reborrow()),
            SubDivide { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .subdivide(child_b.compute(compute_arg.reborrow())),
            NormalisedAdd {
                child_a,
                child_b,
                child_normaliser,
            } => child_a.compute(compute_arg.reborrow()).normalised_add(
                child_b.compute(compute_arg.reborrow()),
                child_normaliser.compute(compute_arg.reborrow()),
            ),
            ComplexReal { child_complex } => child_complex.compute(compute_arg).re(),
            ComplexImaginary { child_complex } => child_complex.compute(compute_arg).im(),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for SNFloatNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
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
        child_distance_function: DistanceFunction,
        child_exit_normaliser: Box<UFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    IterativeMatrix {
        child_matrix: Box<SNFloatMatrix3Nodes>,
        child_iterations: Box<ByteNodes>,
        child_exit_condition: Box<BooleanNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
        child_exit_normaliser: Box<UFloatNormaliserNodes>,
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

impl Node for UNFloatNodes {
    type Output = UNFloat;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use UNFloatNodes::*;

        match self {
            // Random => UNFloat::generate(state),
            Constant { value } => *value,
            FromAngle { child } => child.compute(compute_arg.reborrow()).to_unsigned(),
            FromSNFloat { child } => child.compute(compute_arg.reborrow()).to_unsigned(),
            AbsSNFloat { child } => {
                UNFloat::new(child.compute(compute_arg.reborrow()).into_inner().abs())
            }
            SquareSNFloat { child } => {
                UNFloat::new(child.compute(compute_arg.reborrow()).into_inner().powf(2.0))
            }
            Multiply { child_a, child_b } => UNFloat::new(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    * child_b.compute(compute_arg.reborrow()).into_inner(),
            ),
            CircularAdd { child_a, child_b } => {
                let value = child_a.compute(compute_arg.reborrow()).into_inner()
                    + child_b.compute(compute_arg.reborrow()).into_inner();
                UNFloat::new(value - (value.floor()))
            }
            InvertNormalised { child } => {
                UNFloat::new(1.0 - child.compute(compute_arg.reborrow()).into_inner())
            }
            ColorAverage {
                child,
                child_r,
                child_g,
                child_b,
                child_a,
            } => {
                let color = child.compute(compute_arg.reborrow());
                let r = child_r.compute(compute_arg.reborrow()).into_inner();
                let g = child_g.compute(compute_arg.reborrow()).into_inner();
                let b = child_b.compute(compute_arg.reborrow()).into_inner();
                let a = child_a.compute(compute_arg.reborrow()).into_inner();

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
            ColorComponentH { child } => child.compute(compute_arg.reborrow()).get_hue_unfloat(),
            FromGametic => compute_arg.coordinate_set.get_unfloat_t(),
            FromGameticTriangle => UNFloat::new_triangle(compute_arg.coordinate_set.t * 0.1),
            EscapeTimeSystem {
                child_power,
                child_power_ratio,
                child_offset,
                child_scale,
                child_iterations,
                child_exponentiate,
                child_distance_function,
                child_exit_normaliser,
            } => {
                let power = f64::from(
                    (1 + child_power.compute(compute_arg.reborrow()).into_inner()) as f32
                        * UNFloat::new_triangle(
                            child_power_ratio
                                .compute(compute_arg.reborrow())
                                .into_inner()
                                * 2.0,
                        )
                        .into_inner(),
                );
                let offset = child_offset.compute(compute_arg.reborrow()).into_inner();
                let scale = child_scale.compute(compute_arg.reborrow()).into_inner();
                let iterations = 1 + child_iterations
                    .compute(compute_arg.reborrow())
                    .into_inner()
                    / 4;

                // x and y are swapped intentionally
                let c = Complex::new(
                    f64::from(2.0 * scale.y * compute_arg.coordinate_set.y.into_inner()),
                    f64::from(2.0 * scale.x * compute_arg.coordinate_set.x.into_inner()),
                );

                let z_offset =
                    // Complex::new(0.0, 0.0);
                    if child_exponentiate.compute(compute_arg.reborrow()).into_inner()
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
                    |z, i| {
                        child_distance_function.calculate_point2(
                            Point2::origin(),
                            Point2::new(z.re as f32, z.im as f32),
                        ) > 2.0
                    },
                );

                child_exit_normaliser
                    .compute(compute_arg.reborrow())
                    .normalise((escape as f32 / iterations as f32) * 4.0)
            }
            IterativeMatrix {
                child_matrix,
                child_iterations,
                child_exit_condition,
                child_normaliser,
                child_exit_normaliser,
            } => {
                let matrix = child_matrix.compute(compute_arg.reborrow()).into_inner();

                let iterations = 1 + child_iterations
                    .compute(compute_arg.reborrow())
                    .into_inner()
                    / 4;

                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                // x and y are swapped intentionally
                let c = Complex::new(
                    f64::from(compute_arg.coordinate_set.y.into_inner()),
                    f64::from(compute_arg.coordinate_set.x.into_inner()),
                );

                let (z_final, escape) = escape_time_system(
                    c,
                    iterations as usize,
                    |z, i| {
                        let new_point =
                            matrix.transform_point(&Point2::new(z.re as f32, z.im as f32));
                        Complex::new(new_point.x as f64, new_point.y as f64)
                    },
                    |z, i| {
                        child_exit_condition
                            .compute(compute_arg.reborrow().replace_coords(
                                &SNPoint::new_normalised(
                                    Point2::new(z.re as f32, z.im as f32),
                                    normaliser,
                                ),
                            ))
                            .into_inner()
                    },
                );

                child_exit_normaliser
                    .compute(compute_arg.reborrow())
                    .normalise((escape as f32 / iterations as f32) * 4.0)
            }
            SubDivideSawtooth { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .subdivide_sawtooth(child_b.compute(compute_arg.reborrow())),
            SubDivideTriangle { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .subdivide_triangle(child_b.compute(compute_arg.reborrow())),
            Distance { child_function } => child_function.compute(compute_arg.reborrow()),
            Average { child_a, child_b } => UNFloat::new(
                (child_a.compute(compute_arg.reborrow()).into_inner()
                    + child_b.compute(compute_arg.reborrow()).into_inner())
                    / 2.0,
            ),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow()),
                ..compute_arg.reborrow()
            }),
            SawtoothAdd { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .sawtooth_add(child_b.compute(compute_arg.reborrow())),
            TriangleAdd { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .triangle_add(child_b.compute(compute_arg.reborrow())),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg.reborrow()).into_inner() {
                    child_a.compute(compute_arg.reborrow())
                } else {
                    child_b.compute(compute_arg.reborrow())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for UNFloatNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        // use UNFloatNodes::*;

        match self {
            _ => {}
        }
    }
}
