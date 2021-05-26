#![allow(clippy::many_single_char_names)]

use std::{collections::VecDeque, f32::consts::PI, iter};

use itertools::izip;
use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

use nalgebra::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Debug, Serialize, Deserialize)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum FloatColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: FloatColor },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = pipe_node_weight)]
    Grayscale { child: NodeBox<UNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromGenericColor { child: NodeBox<GenericColorNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    InvertXBlendChild { child: NodeBox<FloatColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    GrayscaleWithAlpha {
        child: NodeBox<UNFloatNodes>,
        child_alpha: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SetSaturation {
        child: NodeBox<FloatColorNodes>,
        child_saturation_value: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    RGB {
        r: NodeBox<UNFloatNodes>,
        g: NodeBox<UNFloatNodes>,
        b: NodeBox<UNFloatNodes>,
        a: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    RGBFromNormalisedSNFloats {
        r: NodeBox<SNFloatNodes>,
        r_norm: UFloatNormaliser,
        g: NodeBox<SNFloatNodes>,
        g_norm: UFloatNormaliser,
        b: NodeBox<SNFloatNodes>,
        b_norm: UFloatNormaliser,
        a: NodeBox<SNFloatNodes>,
        a_norm: UFloatNormaliser,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromHSVColor { child: NodeBox<HSVColorNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromCMYKColor { child: NodeBox<CMYKColorNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromLABColor { child: NodeBox<LABColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ColorBlend {
        child_a: NodeBox<ColorBlendNodes>,
        child_b: NodeBox<ColorBlendNodes>,
        blend_function: ColorBlendFunctions,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromBitColor { child: NodeBox<BitColorNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromByteColor { child: NodeBox<ByteColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<FloatColorNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    LineAberration {
        child: NodeBox<FloatColorNodes>,
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TriangleAberration {
        child: NodeBox<FloatColorNodes>,
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    FlowAutomata {
        rho_divisor: Nibble,
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TriangleBlendAutomata {
        reseed_stable: Boolean,
        rho_divisor: Byte,
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PolygonBlendAutomata {
        reseed_stable: Boolean,
        child_point_count: NodeBox<NibbleNodes>,
        rho_divisor: Byte,
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PolygonAddSubtractAutomata {
        reseed_stable: Boolean,
        child_point_count: NodeBox<NibbleNodes>,
        rho_divisor: Byte,
        enforce_same_rho: Boolean,
        child_rho_a: NodeBox<UNFloatNodes>,
        child_rho_b: NodeBox<UNFloatNodes>,
        enforce_offset_theta: Boolean,
        child_theta_a: NodeBox<AngleNodes>,
        child_theta_b: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
        child_r_normaliser: UFloatNormaliser,
        child_g_normaliser: UFloatNormaliser,
        child_b_normaliser: UFloatNormaliser,
        child_a_normaliser: UFloatNormaliser,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<FloatColorNodes>,
        child_b: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    RemoveAlpha {
        child_a: NodeBox<FloatColorNodes>,
        child_b: NodeBox<BooleanNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SetAlpha {
        child_a: NodeBox<FloatColorNodes>,
        child_b: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PointDrawingBuffer {
        #[serde(skip)]
        #[mutagen(skip)]
        buffer: Buffer<FloatColor>,
        child: NodeBox<SNPointNodes>,
        child_color: NodeBox<FloatColorNodes>,
        points_len_child: NodeBox<NibbleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
        #[mutagen(skip)]
        #[serde(skip)]
        points: VecDeque<SNPoint>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PointSetLineBuffer {
        #[serde(skip)]
        #[mutagen(skip)]
        buffer: Buffer<FloatColor>,
        child_point_set: NodeBox<PointSetNodes>,
        child_point: NodeBox<SNPointNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PointSetDotBuffer {
        #[serde(skip)]
        #[mutagen(skip)]
        buffer: Buffer<FloatColor>,
        child_point_set: NodeBox<PointSetNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IterativeCenteredPolarLineBuffer {
        #[serde(skip)]
        #[mutagen(skip)]
        buffer: Buffer<FloatColor>,
        // TODO Replace child_theta and child_rho with a polar coordinate node when they're implemented
        theta: Angle,
        theta_delta: Angle,
        rho: UNFloat,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IterativePolarLineBuffer {
        #[serde(skip)]
        #[mutagen(skip)]
        buffer: Buffer<FloatColor>,
        // TODO Replace child_theta and child_rho with a polar coordinate node when they're implemented
        child_theta: NodeBox<AngleNodes>,
        child_rho: NodeBox<UNFloatNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
        child_color: NodeBox<FloatColorNodes>,
        point: SNPoint,
        theta: Angle,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ClosestPointLineBuffer {
        #[serde(skip)]
        #[mutagen(skip)]
        buffer: Buffer<FloatColor>,
        child_a: NodeBox<PointSetNodes>,
        child_b: NodeBox<PointSetNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    NextPointLineBuffer {
        #[serde(skip)]
        #[mutagen(skip)]
        buffer: Buffer<FloatColor>,
        child_set: NodeBox<PointSetNodes>,
        child_index: NodeBox<ByteNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },
}

impl Node for FloatColorNodes {
    type Output = FloatColor;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use FloatColorNodes::*;

        match self {
            Constant { value } => *value,
            FromImage { image } => image
                .get_pixel_normalised(
                    compute_arg.coordinate_set.x,
                    compute_arg.coordinate_set.y,
                    compute_arg.coordinate_set.t,
                )
                .into(),
            FromCellArray => compute_arg
                .history
                .get_normalised(
                    compute_arg.coordinate_set.get_coord_point(),
                    compute_arg.reborrow().current_t.saturating_sub(1),
                )
                .into(),
            Grayscale { child } => {
                let value = child.compute(compute_arg.reborrow());
                FloatColor {
                    r: value,
                    g: value,
                    b: value,
                    a: value,
                }
            }
            GrayscaleWithAlpha { child, child_alpha } => {
                let value = child.compute(compute_arg.reborrow());
                FloatColor {
                    r: value,
                    g: value,
                    b: value,
                    a: child_alpha.compute(compute_arg.reborrow()),
                }
            }
            FromGenericColor { child } => child.compute(compute_arg).into(),
            InvertXBlendChild { child } => {
                let new_point = compute_arg.coordinate_set.get_coord_point().invert_x();
                let color_a = child.compute(compute_arg.reborrow());
                let color_b = child.compute(compute_arg.reborrow().replace_coords(&new_point));

                color_a.lerp(color_b, UNFloat::new(0.5))
            }
            SetSaturation {
                child,
                child_saturation_value,
            } => {
                let color = child.compute(compute_arg.reborrow());
                let saturation_value = child_saturation_value.compute(compute_arg.reborrow());

                let hsv_tuple = rgb_tuple_to_hsv_tuple(
                    color.r.into_inner(),
                    color.g.into_inner(),
                    color.b.into_inner(),
                );

                let rgb_tuple =
                    hsv_tuple_to_rgb_tuple(hsv_tuple.0, saturation_value.into_inner(), hsv_tuple.2);

                FloatColor {
                    r: UNFloat::new(rgb_tuple.0),
                    g: UNFloat::new(rgb_tuple.1),
                    b: UNFloat::new(rgb_tuple.2),
                    a: color.a,
                }
            }
            RGB { r, g, b, a } => FloatColor {
                r: r.compute(compute_arg.reborrow()),
                g: g.compute(compute_arg.reborrow()),
                b: b.compute(compute_arg.reborrow()),
                a: a.compute(compute_arg.reborrow()),
            },
            RGBFromNormalisedSNFloats {
                r,
                r_norm,
                g,
                g_norm,
                b,
                b_norm,
                a,
                a_norm,
            } => FloatColor {
                r: r_norm.normalise(r.compute(compute_arg.reborrow()).into_inner()),
                g: g_norm.normalise(g.compute(compute_arg.reborrow()).into_inner()),
                b: b_norm.normalise(b.compute(compute_arg.reborrow()).into_inner()),
                a: a_norm.normalise(a.compute(compute_arg.reborrow()).into_inner()),
            },
            FromHSVColor { child } => child.compute(compute_arg).into(),
            FromCMYKColor { child } => child.compute(compute_arg).into(),
            FromLABColor { child } => child.compute(compute_arg).into(),
            ColorBlend {
                child_a,
                child_b,
                blend_function,
            } => blend_function.blend(
                child_a.compute(compute_arg.reborrow()),
                child_b.compute(compute_arg.reborrow()),
            ),
            FromBitColor { child } => FloatColor::from(child.compute(compute_arg.reborrow())),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow()),
                ..compute_arg.reborrow()
            }),
            LineAberration {
                child,
                child_rho,
                child_theta,
                child_normaliser,
            } => {
                let rho = child_rho.compute(compute_arg.reborrow());
                let theta = child_theta.compute(compute_arg.reborrow());
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                // TODO rewrite this once we have a polar type node
                let offset = SNPoint::from_snfloats(
                    SNFloat::new(rho.into_inner() * f32::sin(theta.into_inner())),
                    SNFloat::new(rho.into_inner() * f32::cos(theta.into_inner())),
                );

                let middle = compute_arg.coordinate_set.get_coord_point();

                let r = child
                    .compute(
                        compute_arg
                            .reborrow()
                            .replace_coords(&middle.normalised_add(offset, normaliser)),
                    )
                    .r;

                let mid_color = child.compute(compute_arg.reborrow());
                let g = mid_color.g;

                let b = child
                    .compute(
                        compute_arg
                            .reborrow()
                            .replace_coords(&middle.normalised_sub(offset, normaliser)),
                    )
                    .b;

                let a = mid_color.a;

                FloatColor { r, g, b, a }
            }
            TriangleAberration {
                child,
                child_rho,
                child_theta,
                child_normaliser,
            } => {
                let rho = child_rho.compute(compute_arg.reborrow());
                let theta = child_theta.compute(compute_arg.reborrow());
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                let theta_r = theta;
                let theta_g = theta + Angle::new(2.0 / 3.0 * PI);
                let theta_b = theta - Angle::new(2.0 / 3.0 * PI);

                // TODO rewrite this once we have a polar type node
                let middle = compute_arg.coordinate_set.get_coord_point();

                let r = child
                    .compute(
                        compute_arg
                            .reborrow()
                            .replace_coords(&middle.normalised_add(
                                SNPoint::from_snfloats(
                                    SNFloat::new(rho.into_inner() * f32::sin(theta_r.into_inner())),
                                    SNFloat::new(rho.into_inner() * f32::cos(theta_r.into_inner())),
                                ),
                                normaliser,
                            )),
                    )
                    .r;

                let g = child
                    .compute(
                        compute_arg
                            .reborrow()
                            .replace_coords(&middle.normalised_add(
                                SNPoint::from_snfloats(
                                    SNFloat::new(rho.into_inner() * f32::sin(theta_g.into_inner())),
                                    SNFloat::new(rho.into_inner() * f32::cos(theta_g.into_inner())),
                                ),
                                normaliser,
                            )),
                    )
                    .g;

                let b = child
                    .compute(
                        compute_arg
                            .reborrow()
                            .replace_coords(&middle.normalised_add(
                                SNPoint::from_snfloats(
                                    SNFloat::new(rho.into_inner() * f32::sin(theta_b.into_inner())),
                                    SNFloat::new(rho.into_inner() * f32::cos(theta_b.into_inner())),
                                ),
                                normaliser,
                            )),
                    )
                    .b;

                let a = child.compute(compute_arg.reborrow()).a;

                FloatColor { r, g, b, a }
            }
            FlowAutomata {
                rho_divisor,
                child_rho,
                child_theta,
                child_normaliser,
                ..
            } => {
                let rho = UNFloat::new(
                    child_rho.compute(compute_arg.reborrow()).into_inner()
                        / rho_divisor.into_inner().saturating_add(1) as f32,
                );
                let theta = child_theta.compute(compute_arg.reborrow());
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                // TODO rewrite this once we have a polar type node
                let middle = compute_arg.coordinate_set.get_coord_point();

                let hist_t = compute_arg.reborrow().current_t.saturating_sub(1);

                compute_arg.reborrow().history.get_normalised(
                    middle.normalised_add(
                        SNPoint::from_snfloats(
                            SNFloat::new(rho.into_inner() * f32::sin(theta.into_inner())),
                            SNFloat::new(rho.into_inner() * f32::cos(theta.into_inner())),
                        ),
                        normaliser,
                    ),
                    hist_t,
                )
            }
            TriangleBlendAutomata {
                reseed_stable,
                rho_divisor,
                child_rho,
                child_theta,
                child_normaliser,
                ..
            } => {
                let rho = UNFloat::new(
                    child_rho.compute(compute_arg.reborrow()).into_inner()
                        / rho_divisor.into_inner().saturating_add(1) as f32,
                );
                let theta = child_theta.compute(compute_arg.reborrow());
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                let theta_a = theta;
                let theta_b = theta + Angle::new(2.0 / 3.0 * PI);
                let theta_c = theta - Angle::new(2.0 / 3.0 * PI);

                // TODO rewrite this once we have a polar type node
                let middle = compute_arg.coordinate_set.get_coord_point();

                let hist_t = compute_arg.reborrow().current_t.saturating_sub(1);

                let color_a = compute_arg.reborrow().history.get_normalised(
                    middle.normalised_add(
                        SNPoint::from_snfloats(
                            SNFloat::new(rho.into_inner() * f32::sin(theta_a.into_inner())),
                            SNFloat::new(rho.into_inner() * f32::cos(theta_a.into_inner())),
                        ),
                        normaliser,
                    ),
                    hist_t,
                );

                let color_b = compute_arg.reborrow().history.get_normalised(
                    middle.normalised_add(
                        SNPoint::from_snfloats(
                            SNFloat::new(rho.into_inner() * f32::sin(theta_b.into_inner())),
                            SNFloat::new(rho.into_inner() * f32::cos(theta_b.into_inner())),
                        ),
                        normaliser,
                    ),
                    hist_t,
                );

                let color_c = compute_arg.reborrow().history.get_normalised(
                    middle.normalised_add(
                        SNPoint::from_snfloats(
                            SNFloat::new(rho.into_inner() * f32::sin(theta_c.into_inner())),
                            SNFloat::new(rho.into_inner() * f32::cos(theta_c.into_inner())),
                        ),
                        normaliser,
                    ),
                    hist_t,
                );

                let r_majority =
                    color_a.r.into_inner() + color_b.r.into_inner() + color_c.r.into_inner() > 1.5;
                let r = UNFloat::new(if r_majority {
                    color_a
                        .r
                        .into_inner()
                        .max(color_b.r.into_inner().max(color_c.r.into_inner()))
                } else {
                    color_a
                        .r
                        .into_inner()
                        .min(color_b.r.into_inner().min(color_c.r.into_inner()))
                });

                let g_majority =
                    color_a.g.into_inner() + color_b.g.into_inner() + color_c.g.into_inner() > 1.5;
                let g = UNFloat::new(if g_majority {
                    color_a
                        .g
                        .into_inner()
                        .max(color_b.g.into_inner().max(color_c.g.into_inner()))
                } else {
                    color_a
                        .g
                        .into_inner()
                        .min(color_b.g.into_inner().min(color_c.g.into_inner()))
                });

                let b_majority =
                    color_a.b.into_inner() + color_b.b.into_inner() + color_c.b.into_inner() > 1.5;
                let b = UNFloat::new(if b_majority {
                    color_a
                        .b
                        .into_inner()
                        .max(color_b.b.into_inner().max(color_c.b.into_inner()))
                } else {
                    color_a
                        .b
                        .into_inner()
                        .min(color_b.b.into_inner().min(color_c.b.into_inner()))
                });

                let a_majority =
                    color_a.a.into_inner() + color_b.a.into_inner() + color_c.a.into_inner() > 1.5;
                let a = UNFloat::new(if a_majority {
                    color_a
                        .a
                        .into_inner()
                        .max(color_b.a.into_inner().max(color_c.a.into_inner()))
                } else {
                    color_a
                        .a
                        .into_inner()
                        .min(color_b.a.into_inner().min(color_c.a.into_inner()))
                });

                let result = FloatColor { r, g, b, a };

                if (reseed_stable.into_inner() || true)
                    && (result.get_average() == 1.0 || result.get_average() == 0.0)
                {
                    FloatColor::random(&mut thread_rng())
                } else {
                    result
                }
            }
            PolygonAddSubtractAutomata {
                reseed_stable,
                rho_divisor,
                child_point_count,
                enforce_same_rho,
                child_rho_a,
                child_rho_b,
                enforce_offset_theta,
                child_theta_a,
                child_theta_b,
                child_normaliser,
                child_r_normaliser,
                child_g_normaliser,
                child_b_normaliser,
                child_a_normaliser,
                ..
            } => {
                let point_count = (child_point_count
                    .compute(compute_arg.reborrow())
                    .into_inner()
                    / 3)
                    + 1;
                let rho_a = UNFloat::new(
                    child_rho_a.compute(compute_arg.reborrow()).into_inner()
                        / rho_divisor.into_inner().saturating_add(1) as f32,
                );
                let rho_b = if enforce_same_rho.into_inner() {
                    rho_a
                } else {
                    UNFloat::new(
                        child_rho_b.compute(compute_arg.reborrow()).into_inner()
                            / rho_divisor.into_inner().saturating_add(1) as f32,
                    )
                };
                let theta_a = child_theta_a.compute(compute_arg.reborrow());
                let theta_b = if enforce_offset_theta.into_inner() {
                    Angle::new(theta_a.into_inner() + PI / point_count as f32)
                } else {
                    child_theta_b.compute(compute_arg.reborrow())
                };
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                // TODO rewrite this once we have a polar type node
                let middle = compute_arg.coordinate_set.get_coord_point();

                let hist_t = compute_arg.reborrow().current_t.saturating_sub(1);

                let coordinate_set = compute_arg.reborrow().coordinate_set;

                let current_color = compute_arg
                    .reborrow()
                    .history
                    .get_normalised(coordinate_set.get_coord_point(), hist_t);

                let mut ra_average = 0.0;
                let mut ga_average = 0.0;
                let mut ba_average = 0.0;
                let mut aa_average = 0.0;

                let mut rb_average = 0.0;
                let mut gb_average = 0.0;
                let mut bb_average = 0.0;
                let mut ab_average = 0.0;

                for i in 0..point_count {
                    let theta_a_offset =
                        theta_a + Angle::new((i as f32 / point_count as f32) * 2.0 * PI);
                    let theta_b_offset =
                        theta_b + Angle::new((i as f32 / point_count as f32) * 2.0 * PI);

                    let color_a = compute_arg.reborrow().history.get_normalised(
                        middle.normalised_add(
                            SNPoint::from_snfloats(
                                SNFloat::new(
                                    rho_a.into_inner() * f32::sin(theta_a_offset.into_inner()),
                                ),
                                SNFloat::new(
                                    rho_a.into_inner() * f32::cos(theta_a_offset.into_inner()),
                                ),
                            ),
                            normaliser,
                        ),
                        hist_t,
                    );

                    let color_b = compute_arg.reborrow().history.get_normalised(
                        middle.normalised_add(
                            SNPoint::from_snfloats(
                                SNFloat::new(
                                    rho_b.into_inner() * f32::sin(theta_b_offset.into_inner()),
                                ),
                                SNFloat::new(
                                    rho_b.into_inner() * f32::cos(theta_b_offset.into_inner()),
                                ),
                            ),
                            normaliser,
                        ),
                        hist_t,
                    );

                    ra_average += color_a.r.into_inner();
                    ga_average += color_a.g.into_inner();
                    ba_average += color_a.b.into_inner();
                    aa_average += color_a.a.into_inner();

                    rb_average += color_b.r.into_inner();
                    gb_average += color_b.g.into_inner();
                    bb_average += color_b.b.into_inner();
                    ab_average += color_b.a.into_inner();
                }

                ra_average = ra_average / point_count as f32;
                ga_average = ga_average / point_count as f32;
                ba_average = ba_average / point_count as f32;
                aa_average = aa_average / point_count as f32;

                rb_average = rb_average / point_count as f32;
                gb_average = gb_average / point_count as f32;
                bb_average = bb_average / point_count as f32;
                ab_average = ab_average / point_count as f32;

                let r = child_r_normaliser
                    .normalise(current_color.r.into_inner() + ra_average - rb_average);

                let g = child_g_normaliser
                    .normalise(current_color.g.into_inner() + ga_average - gb_average);

                let b = child_b_normaliser
                    .normalise(current_color.b.into_inner() + ba_average - bb_average);

                let a = child_a_normaliser
                    .normalise(current_color.a.into_inner() + aa_average - ab_average);

                let result = FloatColor { r, g, b, a };

                if (reseed_stable.into_inner() || true)
                    && (result.get_average() == 1.0 || result.get_average() == 0.0)
                {
                    FloatColor::random(&mut thread_rng())
                } else {
                    result
                }
            }
            PolygonBlendAutomata {
                reseed_stable,
                rho_divisor,
                child_point_count,
                child_rho,
                child_theta,
                child_normaliser,
                ..
            } => {
                let point_count = (child_point_count
                    .compute(compute_arg.reborrow())
                    .into_inner()
                    + 1)
                    / 2;
                let rho = UNFloat::new(
                    child_rho.compute(compute_arg.reborrow()).into_inner()
                        / (8 - point_count + 1) as f32
                        / rho_divisor.into_inner().saturating_add(1) as f32,
                );
                let theta = child_theta.compute(compute_arg.reborrow());
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                // TODO rewrite this once we have a polar type node
                let middle = compute_arg.coordinate_set.get_coord_point();

                let hist_t = compute_arg.reborrow().current_t.saturating_sub(1);

                let mut r_total = 0.0;
                let mut g_total = 0.0;
                let mut b_total = 0.0;
                let mut a_total = 0.0;

                let mut r_max = 0.0;
                let mut g_max = 0.0;
                let mut b_max = 0.0;
                let mut a_max = 0.0;

                let mut r_min = 1.0;
                let mut g_min = 1.0;
                let mut b_min = 1.0;
                let mut a_min = 1.0;

                for i in 0..point_count {
                    let theta_offset =
                        theta + Angle::new((i as f32 / point_count as f32) * 2.0 * PI);

                    let color = compute_arg.reborrow().history.get_normalised(
                        middle.normalised_add(
                            SNPoint::from_snfloats(
                                SNFloat::new(
                                    rho.into_inner() * f32::sin(theta_offset.into_inner()),
                                ),
                                SNFloat::new(
                                    rho.into_inner() * f32::cos(theta_offset.into_inner()),
                                ),
                            ),
                            normaliser,
                        ),
                        hist_t,
                    );

                    let r = color.r.into_inner();
                    let g = color.g.into_inner();
                    let b = color.b.into_inner();
                    let a = color.a.into_inner();

                    r_total += r;
                    g_total += g;
                    b_total += b;
                    a_total += a;

                    r_max = r_max.max(r);
                    g_max = g_max.max(g);
                    b_max = b_max.max(b);
                    a_max = a_max.max(a);

                    r_min = r_min.min(r);
                    g_min = g_min.min(g);
                    b_min = b_min.min(b);
                    a_min = a_min.min(a);
                }

                let r_majority = r_total > point_count as f32 * 0.5;
                let r = UNFloat::new(if r_majority { r_max } else { r_min });

                let g_majority = g_total > point_count as f32 * 0.5;
                let g = UNFloat::new(if g_majority { g_max } else { g_min });

                let b_majority = b_total > point_count as f32 * 0.5;
                let b = UNFloat::new(if b_majority { b_max } else { b_min });

                let a_majority = a_total > point_count as f32 * 0.5;
                let a = UNFloat::new(if a_majority { a_max } else { a_min });

                let result = FloatColor { r, g, b, a };

                if (reseed_stable.into_inner() || true)
                    && (result.get_average() == 1.0 || result.get_average() == 0.0)
                {
                    FloatColor::random(&mut thread_rng())
                } else {
                    result
                }
            }
            FromByteColor { child } => FloatColor::from(child.compute(compute_arg.reborrow())),
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
            RemoveAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow());

                if child_b.compute(compute_arg.reborrow()).into_inner() {
                    color.a = UNFloat::ZERO;
                }

                color
            }
            SetAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow());

                color.a = child_b.compute(compute_arg.reborrow());

                color
            }
            PointDrawingBuffer { buffer, .. } => {
                buffer[compute_arg.coordinate_set.get_coord_point()]
            }
            PointSetLineBuffer { buffer, .. } => {
                buffer[compute_arg.coordinate_set.get_coord_point()]
            }
            IterativeCenteredPolarLineBuffer { buffer, .. } => {
                buffer[compute_arg.coordinate_set.get_coord_point()]
            }
            IterativePolarLineBuffer { buffer, .. } => {
                buffer[compute_arg.coordinate_set.get_coord_point()]
            }
            PointSetDotBuffer { buffer, .. } => {
                buffer[compute_arg.coordinate_set.get_coord_point()]
            }
            ClosestPointLineBuffer { buffer, .. } => {
                buffer[compute_arg.coordinate_set.get_coord_point()]
            }
            NextPointLineBuffer { buffer, .. } => {
                buffer[compute_arg.coordinate_set.get_coord_point()]
            }
        }
    }
}

impl<'a> Updatable<'a> for FloatColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, mut arg: UpdArg<'a>) {
        use FloatColorNodes::*;
        match self {
            PointDrawingBuffer {
                buffer,
                child,
                child_color,
                points_len_child,
                child_normaliser,
                points,
            } => {
                let last_point = points.back().copied().unwrap_or_else(SNPoint::zero);

                arg.coordinate_set.x = last_point.x();
                arg.coordinate_set.y = last_point.y();

                let new_point = last_point.normalised_add(
                    child.compute(arg.reborrow().into()),
                    child_normaliser.compute(arg.reborrow().into()),
                );

                let points_len = points_len_child.compute(arg.reborrow().into());

                let color = child_color.compute(arg.reborrow().into());

                buffer[new_point] = color;

                for prev_point in points.iter() {
                    buffer.draw_line(*prev_point, new_point, color);
                }

                points.pop_front();

                while points.len() < points_len.into_inner() as usize + 1 {
                    points.push_back(new_point);
                }
            }

            PointSetLineBuffer {
                buffer,
                child_point_set,
                child_point,
                child_color,
            } => {
                let source = child_point.compute(arg.reborrow().into());

                for dest in child_point_set
                    .compute(arg.reborrow().into())
                    .points()
                    .iter()
                {
                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();
                    let color = child_color.compute(arg.reborrow().into());

                    buffer.draw_line(source, *dest, color)
                }
            }

            PointSetDotBuffer {
                buffer,
                child_point_set,
                child_color,
            } => {
                for dest in child_point_set
                    .compute(arg.reborrow().into())
                    .points()
                    .iter()
                {
                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();

                    let color = child_color.compute(arg.reborrow().into());

                    buffer.draw_dot(*dest, color);
                }
            }

            // RandomWalkBuffer {
            //     buffer,
            //     x,
            //     y,
            //     point,
            //     neighbourhood,
            //     child_color,
            // } => {

            //     let offsets = neighbourhood.offsets();
            //     let offset = offsets[thread_rng().gen::<usize>() % offsets.len()];
            //     *point = point + offset

            //     *x = x + offset[0];
            //     *y = y + offset[1];

            //     arg.coordinate_set.x = point.x();
            //     arg.coordinate_set.y = point.y();

            //     let color = child_color.compute(arg.reborrow().into());

            //     buffer.draw_dot(*dest, color);
            // }
            IterativeCenteredPolarLineBuffer {
                buffer,
                ref mut theta,
                theta_delta,
                rho,
                child_color,
            } => {
                let new_theta = *theta + *theta_delta;
                let color = child_color.compute(arg.reborrow().into());

                let point = SNPoint::from_snfloats(
                    SNFloat::new(rho.into_inner() * f32::sin(theta.into_inner())),
                    SNFloat::new(rho.into_inner() * f32::cos(theta.into_inner())),
                );

                // TODO rewrite this once we have a polar type node
                let new_point = SNPoint::from_snfloats(
                    SNFloat::new(rho.into_inner() * f32::sin(new_theta.into_inner())),
                    SNFloat::new(rho.into_inner() * f32::cos(new_theta.into_inner())),
                );

                buffer.draw_line(point, new_point, color);

                *theta = new_theta;
            }

            IterativePolarLineBuffer {
                buffer,
                child_theta,
                child_rho,
                child_normaliser,
                child_color,
                ref mut point,
                ref mut theta,
            } => {
                let new_theta = *theta + child_theta.compute(arg.reborrow().into());
                let rho = child_rho.compute(arg.reborrow().into());
                let normaliser = child_normaliser.compute(arg.reborrow().into());
                let color = child_color.compute(arg.reborrow().into());

                // TODO rewrite this once we have a polar type node
                let new_point = point.normalised_add(
                    SNPoint::from_snfloats(
                        SNFloat::new(rho.into_inner() * f32::sin(new_theta.into_inner())),
                        SNFloat::new(rho.into_inner() * f32::cos(new_theta.into_inner())),
                    ),
                    normaliser,
                );

                buffer.draw_line(*point, new_point, color);

                *point = new_point;
                *theta = new_theta;
            }

            ClosestPointLineBuffer {
                buffer,
                child_a,
                child_b,
                child_color,
            } => {
                let set_a = child_a.compute(arg.reborrow().into());
                let set_b = child_b.compute(arg.reborrow().into());

                for source in set_a.points().iter() {
                    let dest = set_b.get_closest_point(*source);

                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();
                    let color = child_color.compute(arg.reborrow().into());

                    buffer.draw_line(*source, dest, color)
                }
            }

            NextPointLineBuffer {
                buffer,
                child_set,
                child_index,
                child_color,
            } => {
                let set = child_set.compute(arg.reborrow().into());

                let index = usize::from(child_index.compute(arg.reborrow().into()).into_inner());

                let source = set[index % set.len()];
                let dest = set[(index + 1) % set.len()];

                arg.coordinate_set.x = dest.x();
                arg.coordinate_set.y = dest.y();
                let color = child_color.compute(arg.reborrow().into());

                buffer.draw_line(source, dest, color);
            }

            _ => {}
        }
    }
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum BitColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: BitColor },

    #[mutagen(gen_weight = branch_node_weight)]
    GiveColor {
        child_a: NodeBox<BitColorNodes>,
        child_b: NodeBox<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TakeColor {
        child_a: NodeBox<BitColorNodes>,
        child_b: NodeBox<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    XorColor {
        child_a: NodeBox<BitColorNodes>,
        child_b: NodeBox<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    EqColor {
        child_a: NodeBox<BitColorNodes>,
        child_b: NodeBox<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComponents {
        r: NodeBox<BooleanNodes>,
        g: NodeBox<BooleanNodes>,
        b: NodeBox<BooleanNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: NodeBox<UNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromGenericColor { child: NodeBox<GenericColorNodes> },

    // #[mutagen(gen_weight = pipe_node_weight)]
    // FromByteColor { child: NodeBox<ByteColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromNibbleIndex { child: NodeBox<NibbleNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    NeighbourCountAutomata { rule: NeighbourCountAutomataRule },

    #[mutagen(gen_weight = leaf_node_weight)]
    LifeLikeAutomata { rule: LifeLikeAutomataRule },

    #[mutagen(gen_weight = leaf_node_weight)]
    CyclingLifeLikeAutomata { rule: LifeLikeAutomataRule },

    //This is a total hack
    #[mutagen(gen_weight = branch_node_weight)]
    ReactionDiffusionLikeAutomata {
        rule: LifeLikeAutomataRule,
        rho_divisor: Byte,
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<BitColorNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<BitColorNodes>,
        child_b: NodeBox<BitColorNodes>,
    },
}

impl Node for BitColorNodes {
    type Output = BitColor;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use BitColorNodes::*;
        match self {
            Constant { value } => *value,
            GiveColor { child_a, child_b } => BitColor::from_components(
                child_a
                    .compute(compute_arg.reborrow())
                    .give_color(child_b.compute(compute_arg.reborrow())),
            ),
            TakeColor { child_a, child_b } => BitColor::from_components(
                child_a
                    .compute(compute_arg.reborrow())
                    .take_color(child_b.compute(compute_arg.reborrow())),
            ),
            XorColor { child_a, child_b } => BitColor::from_components(
                child_a
                    .compute(compute_arg.reborrow())
                    .xor_color(child_b.compute(compute_arg.reborrow())),
            ),
            EqColor { child_a, child_b } => BitColor::from_components(
                child_a
                    .compute(compute_arg.reborrow())
                    .eq_color(child_b.compute(compute_arg.reborrow())),
            ),
            FromComponents { r, g, b } => BitColor::from_components([
                r.compute(compute_arg.reborrow()).into_inner(),
                g.compute(compute_arg.reborrow()).into_inner(),
                b.compute(compute_arg.reborrow()).into_inner(),
            ]),
            FromImage { image } => image
                .get_pixel_normalised(
                    compute_arg.coordinate_set.x,
                    compute_arg.coordinate_set.y,
                    compute_arg.coordinate_set.t,
                )
                .into(),
            FromCellArray => compute_arg
                .history
                .get_normalised(
                    compute_arg.coordinate_set.get_coord_point(),
                    compute_arg.reborrow().current_t.saturating_sub(1),
                )
                .into(),
            FromUNFloat { child } => BitColor::from_index(
                (child.compute(compute_arg.reborrow()).into_inner()
                    * 0.99
                    * (CONSTS.max_colors) as f32) as usize,
            ),
            FromGenericColor { child } => child.compute(compute_arg).into(),
            // FromByteColor { child } => child.compute(compute_arg.reborrow()).into(),
            FromNibbleIndex { child } => BitColor::from_index(
                child.compute(compute_arg.reborrow()).into_inner() as usize % 8,
            ),

            NeighbourCountAutomata { rule } => {
                let x = (compute_arg.coordinate_set.x.to_unsigned().into_inner()
                    * CONSTS.cell_array_width as f32)
                    .round() as isize;
                let y = (compute_arg.coordinate_set.y.to_unsigned().into_inner()
                    * CONSTS.cell_array_height as f32)
                    .round() as isize;
                let prev_t = compute_arg.current_t.saturating_sub(1);

                let mut rc = 0;
                let mut gc = 0;
                let mut bc = 0;

                for (dx, dy) in rule.neighbourhood.offsets() {
                    let [r, g, b] = BitColor::from(compute_arg.history.get(
                        (x + dx).rem_euclid(CONSTS.cell_array_width as isize) as usize,
                        (y + dy).rem_euclid(CONSTS.cell_array_height as isize) as usize,
                        prev_t,
                    ))
                    .to_components();

                    if r {
                        rc += 1;
                    }

                    if g {
                        gc += 1;
                    }

                    if b {
                        bc += 1;
                    }
                }

                rule.truth_table[[rc, gc, bc]]
            }

            LifeLikeAutomata { rule } => {
                let x = (compute_arg.coordinate_set.x.to_unsigned().into_inner()
                    * CONSTS.cell_array_width as f32)
                    .round() as usize;
                let y = (compute_arg.coordinate_set.y.to_unsigned().into_inner()
                    * CONSTS.cell_array_height as f32)
                    .round() as usize;
                let prev_t = compute_arg.current_t.saturating_sub(1);

                let mut neighbour_counts = [0; 8];

                for (color, neighbour_count, rule) in izip!(
                    rule.color_order.iter(),
                    neighbour_counts.iter_mut(),
                    rule.color_rules.iter()
                ) {
                    for (dx, dy) in rule.neighbourhood.offsets() {
                        let neighbour = BitColor::from(compute_arg.history.get(
                            (x as isize + dx).rem_euclid(CONSTS.cell_array_width as isize) as usize,
                            (y as isize + dy).rem_euclid(CONSTS.cell_array_height as isize)
                                as usize,
                            prev_t,
                        ));

                        if neighbour.has_color(*color) {
                            *neighbour_count += 1;
                        }
                    }
                }

                let mut new_color = BitColor::from(compute_arg.history.get(x, y, prev_t));

                for (color, neighbour_count, rule) in izip!(
                    rule.color_order.iter(),
                    neighbour_counts.iter(),
                    rule.color_rules.iter()
                ) {
                    let table = &rule.rules[*neighbour_count];

                    if new_color.has_color(*color) {
                        if !table.survival.into_inner() {
                            new_color = BitColor::from_components(new_color.take_color(*color));
                        }
                    } else {
                        if table.birth.into_inner() {
                            new_color = BitColor::from_components(new_color.give_color(*color));
                        }
                    }
                }

                new_color
            }

            CyclingLifeLikeAutomata { rule } => {
                let x = (compute_arg.coordinate_set.x.to_unsigned().into_inner()
                    * CONSTS.cell_array_width as f32)
                    .round() as usize;
                let y = (compute_arg.coordinate_set.y.to_unsigned().into_inner()
                    * CONSTS.cell_array_height as f32)
                    .round() as usize;
                let prev_t = compute_arg.current_t.saturating_sub(1);

                let mut neighbour_counts = [0; 8];

                for (color, neighbour_count, rule) in izip!(
                    rule.color_order.iter(),
                    neighbour_counts.iter_mut(),
                    rule.color_rules.iter()
                ) {
                    for (dx, dy) in rule.neighbourhood.offsets() {
                        let neighbour = BitColor::from(compute_arg.history.get(
                            (x as isize + dx).rem_euclid(CONSTS.cell_array_width as isize) as usize,
                            (y as isize + dy).rem_euclid(CONSTS.cell_array_height as isize)
                                as usize,
                            prev_t,
                        ));

                        if neighbour.has_color(*color) {
                            *neighbour_count += 1;
                        }
                    }
                }

                let mut new_color = BitColor::from(compute_arg.history.get(x, y, prev_t));

                let (first_color_order, rest_color_order) = rule.color_order.split_first().unwrap();

                let color_order_cycled =
                    rest_color_order.iter().chain(iter::once(first_color_order));

                for (color, neighbour_count, rule) in izip!(
                    color_order_cycled,
                    neighbour_counts.iter(),
                    rule.color_rules.iter()
                ) {
                    let table = &rule.rules[*neighbour_count];

                    if new_color.has_color(*color) {
                        if !table.survival.into_inner() {
                            new_color = BitColor::from_components(new_color.take_color(*color));
                        }
                    } else {
                        if table.birth.into_inner() {
                            new_color = BitColor::from_components(new_color.give_color(*color));
                        }
                    }
                }

                new_color
            }

            ReactionDiffusionLikeAutomata {
                rule,
                rho_divisor,
                child_rho,
                child_theta,
                child_normaliser,
            } => {
                let rho = child_rho.compute(compute_arg.reborrow()).into_inner()
                    / rho_divisor.into_inner().saturating_add(1) as f32;
                let theta = child_theta.compute(compute_arg.reborrow());
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                let middle = compute_arg.coordinate_set.get_coord_point();

                let hist_t = compute_arg.reborrow().current_t.saturating_sub(1);

                let mut neighbour_counts = [0; 8];

                for (color, neighbour_count, rule) in izip!(
                    rule.color_order.iter(),
                    neighbour_counts.iter_mut(),
                    rule.color_rules.iter()
                ) {
                    let point_count = rule.neighbourhood.offsets().len();
                    for i in 0..point_count {
                        let theta_offset =
                            theta + Angle::new((i as f32 / point_count as f32) * 2.0 * PI - PI);

                        let neighbour: BitColor = compute_arg
                            .reborrow()
                            .history
                            .get_normalised(
                                middle.normalised_add(
                                    SNPoint::from_snfloats(
                                        SNFloat::new(rho * f32::sin(theta_offset.into_inner())),
                                        SNFloat::new(rho * f32::cos(theta_offset.into_inner())),
                                    ),
                                    normaliser,
                                ),
                                hist_t,
                            )
                            .into();

                        if neighbour.has_color(*color) {
                            *neighbour_count += 1;
                        }
                    }
                }

                let mut new_color =
                    BitColor::from(compute_arg.history.get_normalised(middle, hist_t));

                for (color, neighbour_count, rule) in izip!(
                    rule.color_order.iter(),
                    neighbour_counts.iter(),
                    rule.color_rules.iter()
                ) {
                    let table = &rule.rules[*neighbour_count];

                    if new_color.has_color(*color) {
                        if !table.survival.into_inner() {
                            new_color = BitColor::from_components(new_color.take_color(*color));
                        }
                    } else {
                        if table.birth.into_inner() {
                            new_color = BitColor::from_components(new_color.give_color(*color));
                        }
                    }
                }

                new_color
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
        }
    }
}

impl<'a> Updatable<'a> for BitColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum ByteColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: ByteColor },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = [leaf_node_weight, camera_node_weight])]
    FromCamera,

    #[mutagen(gen_weight = pipe_node_weight)]
    FromGenericColor { child: NodeBox<GenericColorNodes> },

    // #[mutagen(gen_weight = pipe_node_weight)]
    // FromBitColor { child: NodeBox<FloatColorNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    Decompose {
        r: NodeBox<ByteNodes>,
        g: NodeBox<ByteNodes>,
        b: NodeBox<ByteNodes>,
        a: NodeBox<ByteNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    RemoveAlpha {
        child_a: NodeBox<ByteColorNodes>,
        child_b: NodeBox<BooleanNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SetAlpha {
        child_a: NodeBox<ByteColorNodes>,
        child_b: NodeBox<ByteNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    LifeLikeDeltaAutomata {
        r_inc: Nibble,
        r_dec: Nibble,
        g_inc: Nibble,
        g_dec: Nibble,
        b_inc: Nibble,
        b_dec: Nibble,
        r_clamp: Boolean, //TODO: change these to normalisers
        g_clamp: Boolean,
        b_clamp: Boolean,
        rule: LifeLikeAutomataRule,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<ByteColorNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<ByteColorNodes>,
        child_b: NodeBox<ByteColorNodes>,
    },
}

impl Node for ByteColorNodes {
    type Output = ByteColor;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use ByteColorNodes::*;

        match self {
            Constant { value } => *value,
            FromImage { image } => image.get_pixel_normalised(
                compute_arg.coordinate_set.x,
                compute_arg.coordinate_set.y,
                compute_arg.coordinate_set.t,
            ),
            FromCellArray => compute_arg
                .history
                .get_normalised(
                    compute_arg.coordinate_set.get_coord_point(),
                    compute_arg.reborrow().current_t.saturating_sub(1),
                )
                .into(),

            FromCamera => compute_arg.camera_frames.as_ref().unwrap().get(
                compute_arg.coordinate_set.get_coord_point(),
                compute_arg.current_t,
            ),

            Decompose { r, g, b, a } => ByteColor {
                r: r.compute(compute_arg.reborrow()),
                g: g.compute(compute_arg.reborrow()),
                b: b.compute(compute_arg.reborrow()),
                a: a.compute(compute_arg.reborrow()),
            },
            RemoveAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow());

                if child_b.compute(compute_arg.reborrow()).into_inner() {
                    color.a = Byte::new(0);
                }

                color
            }
            SetAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow());

                color.a = child_b.compute(compute_arg.reborrow());

                color
            }
            FromGenericColor { child } => child.compute(compute_arg).into(),
            // FromBitColor { child } => child.compute(compute_arg.reborrow()).into(),
            LifeLikeDeltaAutomata {
                r_inc,
                r_dec,
                g_inc,
                g_dec,
                b_inc,
                b_dec,
                r_clamp,
                g_clamp,
                b_clamp,
                rule,
            } => {
                let x = (compute_arg.coordinate_set.x.to_unsigned().into_inner()
                    * CONSTS.cell_array_width as f32)
                    .round() as usize;
                let y = (compute_arg.coordinate_set.y.to_unsigned().into_inner()
                    * CONSTS.cell_array_height as f32)
                    .round() as usize;
                let prev_t = compute_arg.current_t.saturating_sub(1);

                let mut neighbour_counts = [0; 8];

                // for (dx, dy) in rule.neighbourhood.offsets() {
                //     let neighbour = BitColor::from(compute_arg.history.get(
                //         (x as isize + dx).rem_euclid(CONSTS.cell_array_width as isize) as usize,
                //         (y as isize + dy).rem_euclid(CONSTS.cell_array_height as isize) as usize,
                //         prev_t,
                //     ));

                //     for (color, neighbour_count) in
                //         rule.color_order.iter().zip(neighbour_counts.iter_mut())
                //     {
                //         if neighbour.has_color(*color) {
                //             *neighbour_count += 1;
                //         }
                //     }
                // }

                for (color, neighbour_count, rule) in izip!(
                    rule.color_order.iter(),
                    neighbour_counts.iter_mut(),
                    rule.color_rules.iter()
                ) {
                    for (dx, dy) in rule.neighbourhood.offsets() {
                        let neighbour = BitColor::from(compute_arg.history.get(
                            (x as isize + dx).rem_euclid(CONSTS.cell_array_width as isize) as usize,
                            (y as isize + dy).rem_euclid(CONSTS.cell_array_height as isize)
                                as usize,
                            prev_t,
                        ));

                        if neighbour.has_color(*color) {
                            *neighbour_count += 1;
                        }
                    }
                }

                let mut new_color = BitColor::from(compute_arg.history.get(x, y, prev_t));

                for (color, neighbour_count, rule) in izip!(
                    rule.color_order.iter(),
                    neighbour_counts.iter(),
                    rule.color_rules.iter()
                ) {
                    let table = &rule.rules[*neighbour_count];

                    if new_color.has_color(*color) {
                        if !table.survival.into_inner() {
                            new_color = BitColor::from_components(new_color.take_color(*color));
                        }
                    } else {
                        if table.birth.into_inner() {
                            new_color = BitColor::from_components(new_color.give_color(*color));
                        }
                    }
                }

                let other = new_color.to_components();
                let old_color: ByteColor = compute_arg.history.get(x, y, prev_t).into();

                let r_delta = if other[0] {
                    r_inc.into_inner() as i32
                } else {
                    r_dec.into_inner() as i32 * -1
                };
                let g_delta = if other[1] {
                    g_inc.into_inner() as i32
                } else {
                    g_dec.into_inner() as i32 * -1
                };
                let b_delta = if other[2] {
                    b_inc.into_inner() as i32
                } else {
                    b_dec.into_inner() as i32 * -1
                };

                ByteColor {
                    r: if r_clamp.into_inner() {
                        old_color.r.clamped_add_i32(r_delta)
                    } else {
                        old_color.r.circular_add_i32(r_delta)
                    },
                    g: if g_clamp.into_inner() {
                        old_color.g.clamped_add_i32(g_delta)
                    } else {
                        old_color.g.circular_add_i32(g_delta)
                    },
                    b: if b_clamp.into_inner() {
                        old_color.b.clamped_add_i32(b_delta)
                    } else {
                        old_color.b.circular_add_i32(b_delta)
                    },
                    a: old_color.a,
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
        }
    }
}

impl<'a> Updatable<'a> for ByteColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum HSVColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: HSVColor },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromGenericColor { child: NodeBox<GenericColorNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComponents {
        h: NodeBox<AngleNodes>,
        s: NodeBox<UNFloatNodes>,
        v: NodeBox<UNFloatNodes>,
        a: NodeBox<UNFloatNodes>,
        offset: Angle,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    OffsetHue {
        child_color: NodeBox<HSVColorNodes>,
        child_offset: NodeBox<AngleNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    HueTShifting {
        child_color: NodeBox<HSVColorNodes>,
        scaling_factor: SNFloat,
    },

    #[mutagen(gen_weight = 0.0)]
    //branch_node_weight)]//TODO FIX, yes I know it says it down there but this is important. Go learn some fractal stuff buttface
    DucksFractal {
        child_offset: NodeBox<SNPointNodes>,
        child_scale: NodeBox<SNPointNodes>,
        child_iterations: NodeBox<ByteNodes>,
        child_magnitude_normaliser: NodeBox<UFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<HSVColorNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<HSVColorNodes>,
        child_b: NodeBox<HSVColorNodes>,
    },
}

impl Node for HSVColorNodes {
    type Output = HSVColor;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use HSVColorNodes::*;

        match self {
            Constant { value } => *value,

            FromGenericColor { child } => child.compute(compute_arg).into(),

            FromImage { image } => FloatColor::from(image.get_pixel_normalised(
                compute_arg.coordinate_set.x,
                compute_arg.coordinate_set.y,
                compute_arg.coordinate_set.t,
            ))
            .into(),

            FromComponents { h, s, v, a, offset } => HSVColor {
                h: h.compute(compute_arg.reborrow()) + *offset,
                s: s.compute(compute_arg.reborrow()),
                v: v.compute(compute_arg.reborrow()),
                a: a.compute(compute_arg.reborrow()),
            },

            OffsetHue {
                child_color,
                child_offset,
            } => {
                let c = child_color.compute(compute_arg.reborrow());

                c.offset_hue(child_offset.compute(compute_arg.reborrow()))
            }
            HueTShifting {
                child_color,
                scaling_factor,
            } => {
                let c = child_color.compute(compute_arg.reborrow());

                c.offset_hue(Angle::new(
                    compute_arg
                        .reborrow()
                        .coordinate_set
                        .get_unfloat_t()
                        .to_signed()
                        .to_angle()
                        .into_inner()
                        * scaling_factor.into_inner(),
                ))
            }

            DucksFractal {
                //TODO make gooderer
                child_offset,
                child_scale,
                ..
            } => {
                let _offset = child_offset.compute(compute_arg.reborrow()).into_inner();
                let _scale = child_scale.compute(compute_arg.reborrow()).into_inner();
                let iterations = //50;
                128 - (128 - compute_arg.coordinate_set.get_byte_t().into_inner() as i32).abs();
                // 1 + child_iterations
                //     .compute(compute_arg.reborrow())
                //     .into_inner()
                //     / 4;

                // x and y are swapped intentionally
                let c = Complex::new(
                    f64::from(
                        //0.001 *
                        (1.0 - compute_arg.coordinate_set.get_unfloat_t().into_inner()) *
                    // scale.y * 
                    compute_arg.coordinate_set.y.into_inner(), // - 0.5
                    ),
                    f64::from(
                        //0.001 *
                        (1.0 - compute_arg.coordinate_set.get_unfloat_t().into_inner()) *
                    // scale.x * 
                    compute_arg.coordinate_set.x.into_inner(), // - 0.5
                    ),
                );

                let _c_offset = Complex::new(
                    // compute_arg.coordinate_set.get_unfloat_t().into_inner() as f64
                    // f64::from(scale.y) *
                    // f64::from(offset.y)
                    0.0,
                    // f64::from(scale.x) *
                    // f64::from(offset.x)
                    // compute_arg.coordinate_set.get_unfloat_t().into_inner() as f64
                    0.0,
                );

                let mut magnitude = 0.0;

                let (_z_final, _escape) = escape_time_system(
                    c, // + c_offset
                    iterations as usize,
                    |z, _i| {
                        (
                            // if z.im > 0.0 { z } else { z.conj() }
                            z.abs() + c
                        )
                            .ln()
                    },
                    |z, _i| {
                        magnitude += z.norm();
                        false
                    },
                );

                // magnitude /= iterations as f64;

                // IterativeResult::new(
                //     SNComplex::new_normalised(
                //         z_final,
                //         child_exit_normaliser.compute(compute_arg.reborrow()),
                //     ),
                //     Byte::new(iterations),
                // )

                HSVColor {
                    h: Angle::new(magnitude as f32),
                    s: UNFloat::new(1.0),
                    v: UNFloat::new(0.5),
                    a: UNFloat::new(1.0),
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
        }
    }
}

impl<'a> Updatable<'a> for HSVColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum CMYKColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: CMYKColor },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromGenericColor { child: NodeBox<GenericColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComponents {
        c: NodeBox<UNFloatNodes>,
        m: NodeBox<UNFloatNodes>,
        y: NodeBox<UNFloatNodes>,
        k: NodeBox<UNFloatNodes>,
        a: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<CMYKColorNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<CMYKColorNodes>,
        child_b: NodeBox<CMYKColorNodes>,
    },
}

impl Node for CMYKColorNodes {
    type Output = CMYKColor;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use CMYKColorNodes::*;

        match self {
            Constant { value } => *value,

            FromGenericColor { child } => child.compute(compute_arg).into(),

            FromComponents { c, m, y, k, a } => CMYKColor {
                c: c.compute(compute_arg.reborrow()),
                m: m.compute(compute_arg.reborrow()),
                y: y.compute(compute_arg.reborrow()),
                k: k.compute(compute_arg.reborrow()),
                a: a.compute(compute_arg.reborrow()),
            },

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

impl<'a> Updatable<'a> for CMYKColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum LABColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: LABColor },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromGenericColor { child: NodeBox<GenericColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComponents {
        l: NodeBox<SNFloatNodes>,
        a: NodeBox<SNFloatNodes>,
        b: NodeBox<SNFloatNodes>,
        alpha: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComplexComponents {
        l: NodeBox<SNFloatNodes>,
        ab: NodeBox<SNComplexNodes>,
        alpha: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IterativeResult {
        child_iterative_function: NodeBox<IterativeFunctionNodes>,
        alpha: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: NodeBox<LABColorNodes>,
        child_state: NodeBox<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: NodeBox<BooleanNodes>,
        child_a: NodeBox<LABColorNodes>,
        child_b: NodeBox<LABColorNodes>,
    },
}

impl Node for LABColorNodes {
    type Output = LABColor;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use LABColorNodes::*;

        match self {
            Constant { value } => *value,

            FromGenericColor { child } => child.compute(compute_arg).into(),

            FromComponents { l, a, b, alpha } => LABColor {
                l: l.compute(compute_arg.reborrow()),
                ab: SNComplex::from_snfloats(
                    a.compute(compute_arg.reborrow()),
                    b.compute(compute_arg.reborrow()),
                ),
                alpha: alpha.compute(compute_arg.reborrow()),
            },

            FromComplexComponents { l, ab, alpha } => LABColor {
                l: l.compute(compute_arg.reborrow()),
                ab: ab.compute(compute_arg.reborrow()),
                alpha: alpha.compute(compute_arg.reborrow()),
            },

            IterativeResult {
                child_iterative_function,
                alpha,
            } => {
                let result = child_iterative_function.compute(compute_arg.reborrow());

                LABColor {
                    l: SNFloat::new((result.iter_final.into_inner() as f32 - 127.0) / 255.0),
                    ab: result.z_final,
                    alpha: alpha.compute(compute_arg.reborrow()),
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
        }
    }
}

impl<'a> Updatable<'a> for LABColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum GenericColorNodes {
    //Necessary for using a generic color node as a child
    #[mutagen(mut_reroll = 0.9)]
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: FloatColor },
    #[mutagen(mut_reroll = 0.5)]
    #[mutagen(gen_weight = pipe_node_weight)]
    Float { child: NodeBox<FloatColorNodes> },
    #[mutagen(mut_reroll = 0.5)]
    #[mutagen(gen_weight = pipe_node_weight)]
    Byte { child: NodeBox<ByteColorNodes> },
    #[mutagen(mut_reroll = 0.5)]
    #[mutagen(gen_weight = pipe_node_weight)]
    Bit { child: NodeBox<BitColorNodes> },

    #[mutagen(mut_reroll = 0.5)]
    #[mutagen(gen_weight = pipe_node_weight)]
    HSV { child: NodeBox<HSVColorNodes> },
    #[mutagen(mut_reroll = 0.5)]
    #[mutagen(gen_weight = pipe_node_weight)]
    CMYK { child: NodeBox<CMYKColorNodes> },
    #[mutagen(mut_reroll = 0.5)]
    #[mutagen(gen_weight = pipe_node_weight)]
    LAB { child: NodeBox<LABColorNodes> },
}

impl Node for GenericColorNodes {
    type Output = FloatColor;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
        use GenericColorNodes::*;

        match self {
            Constant { value } => *value,
            Float { child } => child.compute(compute_arg),
            Byte { child } => child.compute(compute_arg).into(),
            Bit { child } => child.compute(compute_arg).into(),

            HSV { child } => child.compute(compute_arg).into(),
            CMYK { child } => child.compute(compute_arg).into(),
            LAB { child } => child.compute(compute_arg).into(),
        }
    }
}

impl<'a> Updatable<'a> for GenericColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
