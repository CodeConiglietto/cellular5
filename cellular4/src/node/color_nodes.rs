use std::{collections::VecDeque, f32::consts::PI};

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, Lab, Limited, RgbHue};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum FloatColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: FloatColor },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = pipe_node_weight)]
    Grayscale { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    RGB {
        r: Box<UNFloatNodes>,
        g: Box<UNFloatNodes>,
        b: Box<UNFloatNodes>,
        a: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    HSV {
        h: Box<UNFloatNodes>,
        s: Box<UNFloatNodes>,
        v: Box<UNFloatNodes>,
        a: Box<UNFloatNodes>,
        offset: UNFloat,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    HSVAngle {
        h: Box<AngleNodes>,
        s: Box<UNFloatNodes>,
        v: Box<UNFloatNodes>,
        a: Box<UNFloatNodes>,
        offset: Angle,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    CMYK {
        c: Box<UNFloatNodes>,
        m: Box<UNFloatNodes>,
        y: Box<UNFloatNodes>,
        k: Box<UNFloatNodes>,
        a: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    LAB {
        l: Box<UNFloatNodes>,
        a: Box<SNFloatNodes>,
        b: Box<SNFloatNodes>,
        alpha: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    // #[mutagen(gen_preferred)]
    ComplexLAB {
        l: Box<UNFloatNodes>,
        child_complex: Box<SNComplexNodes>,
        alpha: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    // #[mutagen(gen_preferred)]
    IterativeResultLAB {
        child_iterative_function: Box<IterativeFunctionNodes>,
        alpha: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromBlend { child: Box<ColorBlendNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromBitColor { child: Box<BitColorNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromByteColor { child: Box<ByteColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<FloatColorNodes>,
        child_state: Box<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    LineAberration {
        child: Box<FloatColorNodes>,
        child_rho: Box<UNFloatNodes>,
        child_theta: Box<AngleNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    // #[mutagen(gen_preferred)]
    TriangleAberration {
        child: Box<FloatColorNodes>,
        child_rho: Box<UNFloatNodes>,
        child_theta: Box<AngleNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<FloatColorNodes>,
        child_b: Box<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    SetAlpha {
        child_a: Box<FloatColorNodes>,
        child_b: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PointDrawingBuffer {
        buffer: Buffer<FloatColor>,
        child: Box<SNPointNodes>,
        child_color: Box<FloatColorNodes>,
        points_len_child: Box<NibbleNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
        #[mutagen(skip)]
        #[serde(skip)]
        points: VecDeque<SNPoint>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PointSetLineBuffer {
        buffer: Buffer<FloatColor>,
        child_point_set: Box<PointSetNodes>,
        child_point: Box<SNPointNodes>,
        child_color: Box<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PointSetDotBuffer {
        buffer: Buffer<FloatColor>,
        child_point_set: Box<PointSetNodes>,
        child_color: Box<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IterativePolarLineBuffer {
        buffer: Buffer<FloatColor>,
        // TODO Replace child_theta and child_rho with a polar coordinate node when they're implemented
        child_theta: Box<AngleNodes>,
        child_rho: Box<UNFloatNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
        child_color: Box<FloatColorNodes>,
        point: SNPoint,
        theta: Angle,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ClosestPointLineBuffer {
        buffer: Buffer<FloatColor>,
        child_a: Box<PointSetNodes>,
        child_b: Box<PointSetNodes>,
        child_color: Box<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    NextPointLineBuffer {
        buffer: Buffer<FloatColor>,
        child_set: Box<PointSetNodes>,
        child_index: Box<ByteNodes>,
        child_color: Box<FloatColorNodes>,
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
                .get(
                    ((compute_arg.coordinate_set.x.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_width as f32) as usize,
                    ((compute_arg.coordinate_set.y.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_height as f32) as usize,
                    compute_arg.coordinate_set.t as usize,
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
            RGB { r, g, b, a } => FloatColor {
                r: r.compute(compute_arg.reborrow()),
                g: g.compute(compute_arg.reborrow()),
                b: b.compute(compute_arg.reborrow()),
                a: a.compute(compute_arg.reborrow()),
            },
            HSV { h, s, v, a, offset } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_degrees(
                        (h.compute(compute_arg.reborrow()).into_inner() + offset.into_inner())
                            * 360.0,
                    ),
                    s.compute(compute_arg.reborrow()).into_inner() as f32,
                    v.compute(compute_arg.reborrow()).into_inner() as f32,
                ))
                .into();

                float_color_from_pallette_rgb(rgb, a.compute(compute_arg.reborrow()).into_inner())
            }
            HSVAngle { h, s, v, a, offset } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_radians(
                        h.compute(compute_arg.reborrow()).into_inner() + offset.into_inner(),
                    ),
                    s.compute(compute_arg.reborrow()).into_inner(),
                    v.compute(compute_arg.reborrow()).into_inner(),
                ))
                .into();

                float_color_from_pallette_rgb(rgb, a.compute(compute_arg.reborrow()).into_inner())
            }
            CMYK { c, m, y, k, a } => {
                let k_value = k.compute(compute_arg.reborrow()).into_inner();

                FloatColor {
                    r: UNFloat::new(
                        (1.0 - c.compute(compute_arg.reborrow()).into_inner()) * (1.0 - k_value),
                    ),
                    g: UNFloat::new(
                        (1.0 - m.compute(compute_arg.reborrow()).into_inner()) * (1.0 - k_value),
                    ),
                    b: UNFloat::new(
                        (1.0 - y.compute(compute_arg.reborrow()).into_inner()) * (1.0 - k_value),
                    ),
                    a: a.compute(compute_arg.reborrow()),
                }
            }
            LAB { l, a, b, alpha } => {
                let lab = Lab::new(
                    l.compute(compute_arg.reborrow()).into_inner() * 100.0,
                    a.compute(compute_arg.reborrow()).into_inner() * 127.0,
                    b.compute(compute_arg.reborrow()).into_inner() * 127.0,
                );

                let rgb: Rgb = lab.into();

                float_color_from_pallette_rgb(
                    rgb.clamp(),
                    alpha.compute(compute_arg.reborrow()).into_inner(),
                )
            }
            ComplexLAB {
                l,
                child_complex,
                alpha,
            } => {
                let complex = child_complex.compute(compute_arg.reborrow());

                let lab = Lab::new(
                    l.compute(compute_arg.reborrow()).into_inner() * 100.0,
                    complex.re().into_inner() * 127.0,
                    complex.im().into_inner() * 127.0,
                );

                let rgb: Rgb = lab.into();

                float_color_from_pallette_rgb(
                    rgb.clamp(),
                    alpha.compute(compute_arg.reborrow()).into_inner(),
                )
            }
            IterativeResultLAB {
                child_iterative_function,
                alpha,
            } => {
                let result = child_iterative_function.compute(compute_arg.reborrow());

                let lab = Lab::new(
                    result.iter_final.into_inner() as f32 * 100.0 / 255.0,
                    result.z_final.re().into_inner() * 127.0,
                    result.z_final.im().into_inner() * 127.0,
                );

                let rgb: Rgb = lab.into();

                float_color_from_pallette_rgb(
                    rgb.clamp(),
                    alpha.compute(compute_arg.reborrow()).into_inner(),
                )
            }
            FromBlend { child } => child.compute(compute_arg.reborrow()),
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

    fn update(&mut self, _state: mutagen::State, mut arg: UpdArg<'a>) {
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
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TakeColor {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    XorColor {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    EqColor {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComponents {
        r: Box<BooleanNodes>,
        g: Box<BooleanNodes>,
        b: Box<BooleanNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromFloatColor { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromByteColor { child: Box<ByteColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromNibbleIndex { child: Box<NibbleNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<BitColorNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
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
                .get(
                    ((compute_arg.coordinate_set.x.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_width as f32) as usize,
                    ((compute_arg.coordinate_set.y.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_height as f32) as usize,
                    compute_arg.coordinate_set.t as usize,
                )
                .into(),
            FromUNFloat { child } => BitColor::from_index(
                (child.compute(compute_arg.reborrow()).into_inner()
                    * 0.99
                    * (CONSTS.max_colors) as f32) as usize,
            ),
            FromFloatColor { child } => {
                BitColor::from_float_color(child.compute(compute_arg.reborrow()))
            }
            FromByteColor { child } => {
                BitColor::from_byte_color(child.compute(compute_arg.reborrow()))
            }
            FromNibbleIndex { child } => BitColor::from_index(
                child.compute(compute_arg.reborrow()).into_inner() as usize % 8,
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

impl<'a> Updatable<'a> for BitColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
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
    #[mutagen(gen_weight = pipe_node_weight)]
    FromFloatColor { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromBitColor { child: Box<FloatColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Decompose {
        r: Box<ByteNodes>,
        g: Box<ByteNodes>,
        b: Box<ByteNodes>,
        a: Box<ByteNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<ByteColorNodes>,
        child_state: Box<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<ByteColorNodes>,
        child_b: Box<ByteColorNodes>,
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
            FromCellArray => compute_arg.history.get(
                ((compute_arg.coordinate_set.x.into_inner() + 1.0)
                    * 0.5
                    * CONSTS.cell_array_width as f32) as usize,
                ((compute_arg.coordinate_set.y.into_inner() + 1.0)
                    * 0.5
                    * CONSTS.cell_array_height as f32) as usize,
                compute_arg.coordinate_set.t as usize,
            ),
            Decompose { r, g, b, a } => ByteColor {
                r: r.compute(compute_arg.reborrow()),
                g: g.compute(compute_arg.reborrow()),
                b: b.compute(compute_arg.reborrow()),
                a: a.compute(compute_arg.reborrow()),
            },
            FromFloatColor { child } => child.compute(compute_arg.reborrow()).into(),
            FromBitColor { child } => child.compute(compute_arg.reborrow()).into(),
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

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}
