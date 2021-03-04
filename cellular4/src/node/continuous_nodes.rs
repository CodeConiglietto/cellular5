use std::f64::consts::PI;

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use nalgebra::*;
use num::signum;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum AngleNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,

    #[mutagen(gen_weight = pipe_node_weight)]
    ArcSin { theta: NodeBox<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    ArcCos { theta: NodeBox<SNFloatNodes> },

    // #[mutagen(gen_weight = leaf_node_weight)]
    // #[mutagen(mut_reroll = 0.9)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCoordinate,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: Angle },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNPoint { child: NodeBox<SNPointNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNComplex { child: NodeBox<SNComplexNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat { child: NodeBox<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: NodeBox<UNFloatNodes> },

    //TODO: figure out if this actually works
    #[mutagen(gen_weight = pipe_node_weight)]
    MirrorOverYAxis { child: NodeBox<AngleNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: NodeBox<AngleNodes>,
        child_b: NodeBox<AngleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    MultiplyUNFloat {
        child_a: NodeBox<AngleNodes>,
        child_b: NodeBox<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    MultiplySNFloat {
        child_a: NodeBox<AngleNodes>,
        child_b: NodeBox<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<AngleNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<AngleNodes>,
        child_b: NodeBox<AngleNodes>,
    },
}

impl Node for AngleNodes {
    type Output = Angle;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use AngleNodes::*;

        match self {
            FromGametic => compute_arg.coordinate_set.get_angle_t(),
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
            FromSNComplex { child } => child.compute(compute_arg.reborrow()).to_angle(),
            FromSNFloat { child } => child.compute(compute_arg.reborrow()).to_angle(),
            FromUNFloat { child } => child.compute(compute_arg.reborrow()).to_angle(),
            MirrorOverYAxis { child } => Angle::new(
                child.compute(compute_arg.reborrow()).into_inner()
                    * signum(compute_arg.coordinate_set.x.into_inner()),
            ),
            Add { child_a, child_b } => Angle::new(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    + child_b.compute(compute_arg.reborrow()).into_inner(),
            ),
            MultiplyUNFloat { child_a, child_b } => Angle::new(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    * child_b.compute(compute_arg.reborrow()).into_inner(),
            ),
            MultiplySNFloat { child_a, child_b } => Angle::new(
                child_a.compute(compute_arg.reborrow()).into_inner()
                    * child_b.compute(compute_arg.reborrow()).into_inner(),
            ),
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

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNFloatNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Sin { child: NodeBox<AngleNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    Cos { child: NodeBox<AngleNodes> },

    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SNFloat },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromAngle { child: NodeBox<AngleNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: NodeBox<UNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromBoolean { child: NodeBox<BooleanNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    FromUNFloatAndBoolean {
        child_float: NodeBox<UNFloatNodes>,
        child_bool: NodeBox<BooleanNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: NodeBox<SNFloatNodes>,
        child_b: NodeBox<SNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Abs { child: NodeBox<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    Invert { child: NodeBox<SNFloatNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    XRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    YRatio,

    #[mutagen(gen_weight = pipe_node_weight)]
    Relu { child: NodeBox<SNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Elu {
        child_alpha: NodeBox<UNFloatNodes>,
        child: NodeBox<SNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    LeakyRelu {
        child_alpha: NodeBox<UNFloatNodes>,
        child: NodeBox<SNFloatNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,

    #[mutagen(gen_weight = pipe_node_weight)]
    FromGameticNormalised {
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
        #[mutagen(skip)]
        offset_t: Option<f32>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    NoiseFunction {
        noise_function: NoiseFunctions,
        scale_x: Nibble,
        scale_y: Nibble,
        scale_t: UNFloat,
    },

    // IterativeMatrixNoiseFunction {//TODO: finish
    //     noise_function:NodeBox<UNFloatNodes>,
    //     child_matrix:NodeBox<SNFloatMatrix3Nodes>,
    //     iterated_matrix: SNFloatMatrix3,
    //     #[mutagen(skip)]
    //     offset_xy: Point2<f32>,
    //     child_offset_t:NodeBox<SNFloatNodes>,
    //     #[mutagen(skip)]
    //     t_offset: f32,
    // },
    #[mutagen(gen_weight = branch_node_weight)]
    SubDivide {
        child_a: NodeBox<SNFloatNodes>,
        child_b: NodeBox<NibbleNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<SNFloatNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    NormalisedAdd {
        child_a: NodeBox<SNFloatNodes>,
        child_b: NodeBox<SNFloatNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    ComplexReal {
        child_complex: NodeBox<SNComplexNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    ComplexImaginary {
        child_complex: NodeBox<SNComplexNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<SNFloatNodes>,
        child_b: NodeBox<SNFloatNodes>,
    },

    #[mutagen(gen_weight = gamepad_node_weight)]
    FromGamepadAxis { axis: GamepadAxis, id: GamepadId },
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
            FromBoolean { child } => {
                SNFloat::new(if child.compute(compute_arg.reborrow()).into_inner() {
                    1.0
                } else {
                    -1.0
                })
            }
            FromUNFloatAndBoolean {
                child_float,
                child_bool,
            } => SNFloat::new(
                child_float.compute(compute_arg.reborrow()).into_inner()
                    * if child_bool.compute(compute_arg.reborrow()).into_inner() {
                        1.0
                    } else {
                        -1.0
                    },
            ),
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

            Relu { child } => SNFloat::new(child.compute(compute_arg).into_inner().max(0.0)),
            Elu { child_alpha, child } => {
                let value = child.compute(compute_arg.reborrow());
                if value.into_inner() < 0.0 {
                    SNFloat::new(
                        child_alpha.compute(compute_arg.reborrow()).into_inner()
                            * (value.into_inner().exp() - 1.0),
                    )
                } else {
                    value
                }
            }
            LeakyRelu { child_alpha, child } => {
                let value = child.compute(compute_arg.reborrow()).into_inner();
                SNFloat::new(
                    value.max(child_alpha.compute(compute_arg.reborrow()).into_inner() * value),
                )
            }

            FromGametic => SNFloat::new(
                (compute_arg.coordinate_set.t - compute_arg.coordinate_set.t.floor()) * 2.0 - 1.0,
            ),
            FromGameticNormalised {
                child_normaliser,
                offset_t,
            } => {
                let offset_t_value = offset_t.unwrap_or(0.0);

                child_normaliser
                    .compute(compute_arg.reborrow())
                    .normalise(compute_arg.coordinate_set.reborrow().t - offset_t_value)
            }
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow()),
                ..compute_arg.reborrow()
            }),

            NoiseFunction {
                noise_function,
                scale_x,
                scale_y,
                scale_t,
            } => SNFloat::new_clamped(noise_function.compute(
                compute_arg.coordinate_set.x.into_inner() as f64 * scale_x.into_inner() as f64,
                compute_arg.coordinate_set.y.into_inner() as f64 * scale_y.into_inner() as f64,
                compute_arg.coordinate_set.t as f64 * (scale_t.into_inner() / 4.0) as f64,
            ) as f32),
            // IterativeMatrixNoiseFunction {
            //     noise_function,
            //     child_matrix,
            //     iterated_matrix,
            //     child_offset_xy,
            //     child_offset_t,
            //     child_scale_t,
            // } =>
            // {
            //     let transformed_point = Point2::from_homogeneous(child_offset_xy.to_homogeneous() * child_matrix.into_inner());

            //     SNFloat::new_clamped(
            //         noise_function.compute(
            //             compute_arg.coordinate_set.x.into_inner() as f64
            //                 * scale_x_child
            //                     .compute(compute_arg.reborrow())
            //                     .into_inner()
            //                     .powf(2.0) as f64
            //                 * CONSTS.noise_x_scale_factor,
            //             compute_arg.coordinate_set.y.into_inner() as f64
            //                 * scale_y_child
            //                     .compute(compute_arg.reborrow())
            //                     .into_inner()
            //                     .powf(2.0) as f64
            //                 * CONSTS.noise_y_scale_factor,
            //             compute_arg.coordinate_set.t as f64
            //                 * scale_t_child.compute(compute_arg.reborrow()).into_inner() as f64
            //                 * CONSTS.noise_t_scale_factor,
            //         ) as f32,
            //     )
            // }
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

            FromGamepadAxis { axis, id } => {
                SNFloat::new(compute_arg.gamepads[*id].axis_states.get(*axis).value)
            }
        }
    }
}

impl<'a> Updatable<'a> for SNFloatNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, arg: UpdArg<'a>) {
        use SNFloatNodes::*;

        match self {
            FromGamepadAxis { axis, id } => {
                arg.gamepads[*id].axis_states.get_mut(*axis).in_use = true
            }

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
    FromAngle { child: NodeBox<AngleNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromBoolean { child: NodeBox<BooleanNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat { child: NodeBox<SNFloatNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    AbsSNFloat { child: NodeBox<SNFloatNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    SquareSNFloat { child: NodeBox<SNFloatNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    NoiseFunction {
        //Putting one here and converting to brute force more interesting behaviour- This will more readily convert to a colour than the SNFloat version
        noise_function: NoiseFunctions,
        scale_x: Nibble,
        scale_y: Nibble,
        scale_t: UNFloat,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    CircularAdd {
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<UNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    InvertNormalised { child: NodeBox<UNFloatNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    ColorAverage {
        child: NodeBox<FloatColorNodes>,
        child_r: NodeBox<BooleanNodes>,
        child_g: NodeBox<BooleanNodes>,
        child_b: NodeBox<BooleanNodes>,
        child_a: NodeBox<BooleanNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentH { child: NodeBox<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentS { child: NodeBox<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentV { child: NodeBox<FloatColorNodes> },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = pipe_node_weight)]
    FromGameticNormalised {
        child_normaliser: NodeBox<UFloatNormaliserNodes>,
        #[mutagen(skip)]
        offset_t: Option<f32>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    EscapeTimeSystem {
        child_power: NodeBox<NibbleNodes>,
        child_power_ratio: NodeBox<UNFloatNodes>,
        child_offset: NodeBox<SNPointNodes>,
        child_scale: NodeBox<SNPointNodes>,
        child_iterations: NodeBox<ByteNodes>,
        child_exponentiate: NodeBox<BooleanNodes>,
        child_distance_function: DistanceFunction,
        child_exit_normaliser: NodeBox<UFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    IterativeMatrix {
        child_matrix: NodeBox<SNFloatMatrix3Nodes>,
        child_iterations: NodeBox<ByteNodes>,
        child_exit_condition: NodeBox<BooleanNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
        child_exit_normaliser: NodeBox<UFloatNormaliserNodes>,
    },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // LastRotation,
    #[mutagen(gen_weight = branch_node_weight)]
    SubDivideSawtooth {
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    SubDivideTriangle {
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<NibbleNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    DistanceFunction {
        value: DistanceFunction,
        child_a: NodeBox<SNPointNodes>,
        child_b: NodeBox<SNPointNodes>,
        child_normaliser: NodeBox<UFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Average {
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<UNFloatNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SawtoothAdd {
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TriangleAdd {
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<UNFloatNodes>,
        child_b: NodeBox<UNFloatNodes>,
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
            FromBoolean { child } => {
                UNFloat::new(if child.compute(compute_arg.reborrow()).into_inner() {
                    1.0
                } else {
                    0.0
                })
            }
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
            NoiseFunction {
                noise_function,
                scale_x,
                scale_y,
                scale_t,
            } => SNFloat::new_clamped(noise_function.compute(
                compute_arg.coordinate_set.x.into_inner() as f64 * scale_x.into_inner() as f64,
                compute_arg.coordinate_set.y.into_inner() as f64 * scale_y.into_inner() as f64,
                compute_arg.coordinate_set.t as f64 * (scale_t.into_inner() / 4.0) as f64,
            ) as f32)
            .to_unsigned(),
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
            ColorComponentS { child } => child
                .compute(compute_arg.reborrow())
                .get_saturation_unfloat(),
            ColorComponentV { child } => child.compute(compute_arg.reborrow()).get_value_unfloat(),
            FromGametic => compute_arg.coordinate_set.get_unfloat_t(),
            FromGameticNormalised {
                child_normaliser,
                offset_t,
            } => {
                let offset_t_value = offset_t.unwrap_or(0.0);

                child_normaliser
                    .compute(compute_arg.reborrow())
                    .normalise(compute_arg.coordinate_set.reborrow().t - offset_t_value)
            }
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

                let (_z_final, escape) = escape_time_system(
                    c,
                    iterations as usize,
                    |z, i| z.powf(power) + z_offset * i as f64,
                    |z, _i| {
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

                let (_z_final, escape) = escape_time_system(
                    c,
                    iterations as usize,
                    |z, _i| {
                        let new_point =
                            matrix.transform_point(&Point2::new(z.re as f32, z.im as f32));
                        Complex::new(new_point.x as f64, new_point.y as f64)
                    },
                    |z, _i| {
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

            DistanceFunction {
                value,
                child_a,
                child_b,
                child_normaliser,
            } => value.calculate_normalised(
                child_a.compute(compute_arg.reborrow()),
                child_b.compute(compute_arg.reborrow()),
                &child_normaliser.compute(compute_arg.reborrow()),
            ),

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

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
