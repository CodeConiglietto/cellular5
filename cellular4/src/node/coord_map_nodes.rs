use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use nalgebra::{geometry::Point2, geometry::Rotation2};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum CoordMapNodes {
    // #[mutagen(gen_weight = leaf_node_weight)]
    #[mutagen(gen_weight = 20.0)]
    Identity,

    #[mutagen(gen_weight = pipe_node_weight)]
    Replace { child: NodeBox<SNPointNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    ReplaceComplex { child: NodeBox<SNComplexNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Shift {
        x: NodeBox<SNFloatNodes>,
        y: NodeBox<SNFloatNodes>,
        divisor: Nibble,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    Scale {
        x: NodeBox<SNFloatNodes>,
        y: NodeBox<SNFloatNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Rotation {
        child_angle: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    ToPolar,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromPolar,

    #[mutagen(gen_weight = leaf_node_weight)]
    Abs,

    #[mutagen(gen_weight = mic_pipe_node_weight)]
    ShiftAxisSpectrogram {
        axis: Boolean,
        invert_freq_indexing: Boolean,
        use_gamma: Boolean,
        odd_sign: Boolean,
        even_sign: Boolean,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SelectiveAbs {
        child_abs_x: NodeBox<BooleanNodes>,
        child_abs_y: NodeBox<BooleanNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ForceSign {
        child_sign_x: NodeBox<BooleanNodes>,
        child_sign_y: NodeBox<BooleanNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    RotateThenForceSign {
        child_sign_x: NodeBox<BooleanNodes>,
        child_sign_y: NodeBox<BooleanNodes>,
        distance_function: DistanceFunction,
        normaliser: UFloatNormaliser,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ForceSignThenRotate {
        child_sign_x: NodeBox<BooleanNodes>,
        child_sign_y: NodeBox<BooleanNodes>,
        distance_function: DistanceFunction,
        normaliser: UFloatNormaliser,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<CoordMapNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<CoordMapNodes>,
        child_b: NodeBox<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Average {
        child_a: NodeBox<CoordMapNodes>,
        child_b: NodeBox<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Lerp {
        child_lerp_val: NodeBox<UNFloatNodes>,
        child_a: NodeBox<CoordMapNodes>,
        child_b: NodeBox<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ApplyMatrix {
        child_matrix: NodeBox<SNFloatMatrix3Nodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Tessellate {
        child_a: NodeBox<SNPointNodes>,
        child_b: NodeBox<SNPointNodes>,
        child_scale: NodeBox<SNPointNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
        point_a: SNPoint,
        point_b: SNPoint,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    TessellatePerPoint {
        child_a: NodeBox<SNPointNodes>,
        child_b: NodeBox<SNPointNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    TesellatePolarTwoClosestPointSet {
        child_points: NodeBox<PointSetNodes>,
        child_theta: NodeBox<AngleNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    TesellateClosestPointSet { child: NodeBox<PointSetNodes> },
}

impl Node for CoordMapNodes {
    type Output = CoordinateSet;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use CoordMapNodes::*;

        match self {
            Identity => compute_arg.coordinate_set,
            Replace { child } => {
                let coords = child.compute(compute_arg.reborrow());
                compute_arg
                    .reborrow()
                    .replace_coords(&coords)
                    .coordinate_set
            }
            ReplaceComplex { child } => {
                let coords = child.compute(compute_arg.reborrow()).to_snpoint();
                compute_arg
                    .reborrow()
                    .replace_coords(&coords)
                    .coordinate_set
            }
            Shift {
                x,
                y,
                divisor,
                child_normaliser,
            } => compute_arg.coordinate_set.get_coord_shifted(
                SNFloat::new(
                    x.compute(compute_arg.reborrow()).into_inner()
                        / (divisor.into_inner() + 1) as f32,
                ),
                SNFloat::new(
                    y.compute(compute_arg.reborrow()).into_inner()
                        / (divisor.into_inner() + 1) as f32,
                ),
                SNFloat::new(0.0),
                child_normaliser.compute(compute_arg.reborrow()),
            ),
            Scale {
                x,
                y,
                child_normaliser,
            } => compute_arg.coordinate_set.get_coord_scaled(
                x.compute(compute_arg.reborrow()),
                y.compute(compute_arg.reborrow()),
                SNFloat::new(1.0),
                child_normaliser.compute(compute_arg.reborrow()),
            ),
            Rotation {
                child_angle,
                child_normaliser,
            } => {
                let new_pos =
                    Rotation2::new(child_angle.compute(compute_arg.reborrow()).into_inner())
                        .transform_point(&Point2::new(
                            compute_arg.coordinate_set.x.into_inner(),
                            compute_arg.coordinate_set.y.into_inner(),
                        ));

                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                CoordinateSet {
                    x: normaliser.normalise(new_pos.x),
                    y: normaliser.normalise(new_pos.y),
                    t: compute_arg.coordinate_set.t,
                }
            }
            ToPolar => {
                let p = compute_arg.coordinate_set.get_coord_point().to_polar();

                CoordinateSet {
                    x: p.x(),
                    y: p.y(),
                    t: compute_arg.coordinate_set.t,
                }
            }
            FromPolar => {
                let p = compute_arg.coordinate_set.get_coord_point().from_polar();

                CoordinateSet {
                    x: p.x(),
                    y: p.y(),
                    t: compute_arg.coordinate_set.t,
                }
            }
            Abs => {
                let p = compute_arg.coordinate_set.get_coord_point();

                CoordinateSet {
                    x: p.x().abs(),
                    y: p.y().abs(),
                    t: compute_arg.coordinate_set.t,
                }
            }
            ShiftAxisSpectrogram {
                axis,
                invert_freq_indexing,
                use_gamma,
                odd_sign,
                even_sign,
                child_normaliser,
            } => {
                let axis = axis.into_inner();
                let invert_freq_indexing = invert_freq_indexing.into_inner();

                let axis_value = if axis {
                    compute_arg.coordinate_set.x.to_unsigned().into_inner()
                } else {
                    compute_arg.coordinate_set.y.to_unsigned().into_inner()
                };

                let freq = ((axis_value * 256.0) as usize).min(255);

                let mut val = SNFloat::new(
                    compute_arg
                        .mic_spectrograms()
                        .as_ref()
                        .unwrap()
                        .get_spectrogram(use_gamma.into_inner())
                        .get_normalised(if invert_freq_indexing {
                            255 - freq
                        } else {
                            freq
                        })
                        .into_inner(),
                );

                if freq % 2 == 0 && even_sign.into_inner() || freq % 2 == 1 && odd_sign.into_inner()
                {
                    val = val.invert();
                }

                compute_arg.coordinate_set.get_coord_shifted(
                    if axis { SNFloat::ZERO } else { val },
                    if axis { val } else { SNFloat::ZERO },
                    SNFloat::ZERO,
                    child_normaliser.compute(compute_arg.reborrow()),
                )
            }
            SelectiveAbs {
                child_abs_x,
                child_abs_y,
            } => {
                let p = compute_arg.reborrow().coordinate_set.get_coord_point();
                let abs_x = child_abs_x.compute(compute_arg.reborrow());
                let abs_y = child_abs_y.compute(compute_arg.reborrow());

                CoordinateSet {
                    x: if abs_x.into_inner() {
                        p.x().abs()
                    } else {
                        p.x()
                    },
                    y: if abs_y.into_inner() {
                        p.y().abs()
                    } else {
                        p.y()
                    },
                    t: compute_arg.coordinate_set.t,
                }
            }
            ForceSign {
                child_sign_x,
                child_sign_y,
            } => {
                let p = compute_arg.coordinate_set.get_coord_point();
                let sign_x = child_sign_x.compute(compute_arg.reborrow());
                let sign_y = child_sign_y.compute(compute_arg.reborrow());

                CoordinateSet {
                    x: if sign_x.into_inner() {
                        p.x().abs()
                    } else {
                        p.x().abs().invert()
                    },
                    y: if sign_y.into_inner() {
                        p.y().abs()
                    } else {
                        p.y().abs().invert()
                    },
                    t: compute_arg.coordinate_set.t,
                }
            }
            RotateThenForceSign {
                child_sign_x,
                child_sign_y,
                distance_function,
                normaliser,
            } => {
                let p = compute_arg.coordinate_set.get_coord_point();
                let angle = p.to_angle().into_inner()
                    + compute_arg
                        .reborrow()
                        .coordinate_set
                        .get_angle_t()
                        .into_inner();
                let sign_x = child_sign_x.compute(compute_arg.reborrow());
                let sign_y = child_sign_y.compute(compute_arg.reborrow());

                let rho = distance_function
                    .calculate_normalised(SNPoint::zero(), p, normaliser)
                    .into_inner();

                let x = rho * f32::sin(angle);
                let y = rho * f32::cos(angle);

                CoordinateSet {
                    x: if sign_x.into_inner() {
                        SNFloat::new(x.abs())
                    } else {
                        SNFloat::new(x.abs()).invert()
                    },
                    y: if sign_y.into_inner() {
                        SNFloat::new(y.abs())
                    } else {
                        SNFloat::new(y.abs()).invert()
                    },
                    t: compute_arg.coordinate_set.t,
                }
            }
            ForceSignThenRotate {
                child_sign_x,
                child_sign_y,
                distance_function,
                normaliser,
            } => {
                let mut p = compute_arg.coordinate_set.get_coord_point();
                let sign_x = child_sign_x.compute(compute_arg.reborrow());
                let sign_y = child_sign_y.compute(compute_arg.reborrow());

                let mut x = p.x().into_inner();
                let mut y = p.y().into_inner();

                x = if sign_x.into_inner() {
                    x.abs()
                } else {
                    x.abs() * -1.0
                };
                y = if sign_y.into_inner() {
                    y.abs()
                } else {
                    y.abs() * -1.0
                };

                p = SNPoint::new(Point2::new(x, y));

                let angle = p.to_angle().into_inner()
                    + compute_arg
                        .reborrow()
                        .coordinate_set
                        .get_angle_t()
                        .into_inner();

                let rho = distance_function
                    .calculate_normalised(SNPoint::zero(), p, normaliser)
                    .into_inner();

                CoordinateSet {
                    x: SNFloat::new(rho * f32::sin(angle)),
                    y: SNFloat::new(rho * f32::cos(angle)),

                    t: compute_arg.coordinate_set.t,
                }
            }
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
            Average { child_a, child_b } => {
                let a = child_a.compute(compute_arg.reborrow());
                let b = child_b.compute(compute_arg.reborrow());

                CoordinateSet {
                    x: SNFloat::new(a.x.into_inner() * 0.5 + b.x.into_inner() * 0.5),
                    y: SNFloat::new(a.y.into_inner() * 0.5 + b.y.into_inner() * 0.5),
                    t: compute_arg.coordinate_set.t,
                }
            }
            Lerp {
                child_lerp_val,
                child_a,
                child_b,
            } => {
                let lerp_val = child_lerp_val.compute(compute_arg.reborrow()).into_inner();
                let inv_lerp_val = 1.0 - lerp_val;
                let a = child_a.compute(compute_arg.reborrow());
                let b = child_b.compute(compute_arg.reborrow());

                CoordinateSet {
                    x: SNFloat::new(a.x.into_inner() * lerp_val + b.x.into_inner() * inv_lerp_val),
                    y: SNFloat::new(a.y.into_inner() * lerp_val + b.y.into_inner() * inv_lerp_val),
                    t: compute_arg.coordinate_set.t,
                }
            }
            ApplyMatrix {
                child_matrix,
                child_normaliser,
            } => {
                let point = Point2::new(
                    compute_arg.coordinate_set.x.into_inner(),
                    compute_arg.coordinate_set.y.into_inner(),
                )
                .to_homogeneous();

                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                let result = Point2::from_homogeneous(
                    child_matrix.compute(compute_arg.reborrow()).into_inner() * point,
                )
                .unwrap();

                CoordinateSet {
                    x: normaliser.normalise(result.coords.x),
                    y: normaliser.normalise(result.coords.y),
                    t: compute_arg.coordinate_set.t,
                }
            }
            Tessellate {
                point_a, point_b, ..
            } => {
                let a = point_a.into_inner();
                let b = point_b.into_inner();

                let w = b.x - a.x;
                let h = b.y - a.y;

                let mut x_scale = 2.0 / w;
                if !x_scale.is_normal() {
                    x_scale = 10000.0;
                }

                let mut y_scale = 2.0 / h;
                if !y_scale.is_normal() {
                    y_scale = 10000.0;
                }

                let xc = 0.5 * (a.x + b.x);
                let yc = 0.5 * (a.y + b.y);

                CoordinateSet {
                    x: SNFloat::new_triangle(
                        SNFloat::new_triangle(
                            compute_arg.coordinate_set.x.into_inner() * x_scale + xc,
                        )
                        .into_inner()
                            * 0.5
                            * w
                            - xc,
                    ),
                    y: SNFloat::new_triangle(
                        SNFloat::new_triangle(
                            compute_arg.coordinate_set.y.into_inner() * y_scale + yc,
                        )
                        .into_inner()
                            * 0.5
                            * h
                            - yc,
                    ),
                    t: compute_arg.coordinate_set.t,
                }
            }
            TessellatePerPoint { child_a, child_b } => {
                let a = child_a.compute(compute_arg.reborrow()).into_inner();
                let b = child_b.compute(compute_arg.reborrow()).into_inner();

                let w = b.x - a.x;
                let h = b.y - a.y;

                let mut x_scale = 2.0 / w;
                if !x_scale.is_normal() {
                    x_scale = 10000.0;
                }

                let mut y_scale = 2.0 / h;
                if !y_scale.is_normal() {
                    y_scale = 10000.0;
                }

                let xc = 0.5 * (a.x + b.x);
                let yc = 0.5 * (a.y + b.y);

                CoordinateSet {
                    x: SNFloat::new_triangle(
                        SNFloat::new_triangle(
                            compute_arg.coordinate_set.x.into_inner() * x_scale + xc,
                        )
                        .into_inner()
                            * 0.5
                            * w
                            - xc,
                    ),
                    y: SNFloat::new_triangle(
                        SNFloat::new_triangle(
                            compute_arg.coordinate_set.y.into_inner() * y_scale + yc,
                        )
                        .into_inner()
                            * 0.5
                            * h
                            - yc,
                    ),
                    t: compute_arg.coordinate_set.t,
                }
            }

            TesellateClosestPointSet { child } => {
                let p = compute_arg.coordinate_set.get_coord_point();
                let closest = child.compute(compute_arg.reborrow()).get_closest_point(p);

                let offset =
                    SNPoint::new(Point2::from(p.into_inner() - closest.into_inner()) * 0.5);

                CoordinateSet {
                    x: offset.x(),
                    y: offset.y(),
                    t: compute_arg.coordinate_set.t,
                }
            }

            TesellatePolarTwoClosestPointSet {
                child_points,
                child_theta,
            } => {
                let p = compute_arg.coordinate_set.get_coord_point();
                let theta = child_theta.compute(compute_arg.reborrow());
                let mut point_set = child_points.compute(compute_arg.reborrow());
                let closest = point_set.get_n_closest_points(p, 2);

                let polar_1 = SNPoint::new(
                    Point2::from(p.into_inner() - closest.get(0).unwrap_or(&p).into_inner()) * 0.5,
                )
                .to_polar();

                let polar_2 = SNPoint::new(
                    Point2::from(p.into_inner() - closest.get(1).unwrap_or(&p).into_inner()) * 0.5,
                )
                .to_polar();

                let mut y_result =
                    polar_1.y().to_unsigned().into_inner() / polar_2.y().to_unsigned().into_inner();

                if !y_result.is_normal() {
                    y_result = 1.0;
                }

                let offset = SNPoint::from_snfloats(
                    (polar_1.x().to_angle() + theta).to_signed(),
                    UNFloat::new(y_result).to_signed(),
                )
                .from_polar();

                CoordinateSet {
                    x: offset.x(),
                    y: offset.y(),
                    t: compute_arg.coordinate_set.t,
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for CoordMapNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, mut arg: UpdArg<'a>) {
        // TODO Remove when more match arms are added
        #[allow(clippy::single_match)]
        match self {
            CoordMapNodes::Tessellate {
                child_a,
                child_b,
                child_scale,
                child_normaliser,
                ref mut point_a,
                ref mut point_b,
            } => {
                let translation_scale = child_scale
                    .compute(arg.reborrow().into())
                    .scale_unfloat(UNFloat::new(0.025));

                let normaliser = child_normaliser.compute(arg.reborrow().into());

                let mut state_a = arg.reborrow();
                state_a.coordinate_set.x = point_a.x();
                state_a.coordinate_set.y = point_a.y();

                *point_a = point_a.normalised_add(
                    child_a
                        .compute(state_a.into())
                        .scale_point(translation_scale),
                    normaliser,
                );

                let mut state_b = arg.reborrow();
                state_b.coordinate_set.x = point_b.x();
                state_b.coordinate_set.y = point_b.y();

                *point_b = point_b.normalised_add(
                    child_b
                        .compute(state_b.into())
                        .scale_point(translation_scale),
                    normaliser,
                );
            }
            _ => (),
        }
    }
}
