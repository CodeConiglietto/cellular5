use std::collections::VecDeque;

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, Lab, Limited, RgbHue};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{buffers::*, colors::*, continuous::*, image::*, points::*},
    mutagen_args::*,
    node::{
        color_blend_nodes::*, constraint_resolver_nodes::*, continuous_nodes::*,
        coord_map_nodes::*, discrete_nodes::*, mutagen_functions::*, point_nodes::*,
        point_set_nodes::*, Node,
    },
};

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
                let value = child.compute(compute_arg.reborrow().into());
                FloatColor {
                    r: value,
                    g: value,
                    b: value,
                    a: value,
                }
            }
            RGB { r, g, b, a } => FloatColor {
                r: r.compute(compute_arg.reborrow().into()),
                g: g.compute(compute_arg.reborrow().into()),
                b: b.compute(compute_arg.reborrow().into()),
                a: a.compute(compute_arg.reborrow().into()),
            },
            HSV { h, s, v, a, offset } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_degrees(
                        (h.compute(compute_arg.reborrow().into()).into_inner()
                            + offset.into_inner())
                            * 360.0,
                    ),
                    s.compute(compute_arg.reborrow().into()).into_inner() as f32,
                    v.compute(compute_arg.reborrow().into()).into_inner() as f32,
                ))
                .into();

                float_color_from_pallette_rgb(
                    rgb,
                    a.compute(compute_arg.reborrow().into()).into_inner(),
                )
            }
            HSVAngle { h, s, v, a, offset } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_radians(
                        h.compute(compute_arg.reborrow().into()).into_inner() + offset.into_inner(),
                    ),
                    s.compute(compute_arg.reborrow().into()).into_inner(),
                    v.compute(compute_arg.reborrow().into()).into_inner(),
                ))
                .into();

                float_color_from_pallette_rgb(
                    rgb,
                    a.compute(compute_arg.reborrow().into()).into_inner(),
                )
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
                    alpha.compute(compute_arg.reborrow().into()).into_inner(),
                )
            }
            FromBlend { child } => child.compute(compute_arg.reborrow().into()),
            FromBitColor { child } => {
                FloatColor::from(child.compute(compute_arg.reborrow().into()))
            }
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow().into()),
                ..compute_arg.reborrow().into()
            }),
            FromByteColor { child } => {
                FloatColor::from(child.compute(compute_arg.reborrow().into()))
            }
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate
                    .compute(compute_arg.reborrow().into())
                    .into_inner()
                {
                    child_a.compute(compute_arg.reborrow().into())
                } else {
                    child_b.compute(compute_arg.reborrow().into())
                }
            }
            SetAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow().into());

                color.a = child_b.compute(compute_arg.reborrow().into());

                color
            }
            PointDrawingBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
            PointSetLineBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
            PointSetDotBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
            ClosestPointLineBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
            NextPointLineBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
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

                buffer[new_point] = color.clone();

                for prev_point in points.iter() {
                    buffer.draw_line(*prev_point, new_point, color.clone());
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

                for dest in child_point_set.compute(arg.reborrow().into()).points.iter() {
                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();
                    let color = child_color.compute(arg.reborrow().into());

                    buffer.draw_line(source, *dest, color.clone())
                }
            }

            PointSetDotBuffer {
                buffer,
                child_point_set,
                child_color,
            } => {
                for dest in child_point_set.compute(arg.reborrow().into()).points.iter() {
                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();

                    let color = child_color.compute(arg.reborrow().into());

                    buffer.draw_dot(*dest, color.clone());
                }
            }

            ClosestPointLineBuffer {
                buffer,
                child_a,
                child_b,
                child_color,
            } => {
                let set_a = child_a.compute(arg.reborrow().into());
                let set_b = child_b.compute(arg.reborrow().into());

                for source in set_a.points.iter() {
                    let dest = set_b.get_closest_point(*source);

                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();
                    let color = child_color.compute(arg.reborrow().into());

                    buffer.draw_line(*source, dest, color.clone())
                }
            }

            NextPointLineBuffer {
                buffer,
                child_set,
                child_index,
                child_color,
            } => {
                let set = child_set.compute(arg.reborrow().into());

                let index = child_index.compute(arg.reborrow().into()).into_inner() as usize;

                let source = set.get_at(index % set.len());
                let dest = set.get_at((index + 1) % set.len());

                arg.coordinate_set.x = dest.x();
                arg.coordinate_set.y = dest.y();
                let color = child_color.compute(arg.reborrow().into());

                buffer.draw_line(source, dest, color.clone());
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
                    .compute(compute_arg.reborrow().into())
                    .give_color(child_b.compute(compute_arg.reborrow().into())),
            ),
            TakeColor { child_a, child_b } => BitColor::from_components(
                child_a
                    .compute(compute_arg.reborrow().into())
                    .take_color(child_b.compute(compute_arg.reborrow().into())),
            ),
            XorColor { child_a, child_b } => BitColor::from_components(
                child_a
                    .compute(compute_arg.reborrow().into())
                    .xor_color(child_b.compute(compute_arg.reborrow().into())),
            ),
            EqColor { child_a, child_b } => BitColor::from_components(
                child_a
                    .compute(compute_arg.reborrow().into())
                    .eq_color(child_b.compute(compute_arg.reborrow().into())),
            ),
            FromComponents { r, g, b } => BitColor::from_components([
                r.compute(compute_arg.reborrow().into()).into_inner(),
                g.compute(compute_arg.reborrow().into()).into_inner(),
                b.compute(compute_arg.reborrow().into()).into_inner(),
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
                (child.compute(compute_arg.reborrow().into()).into_inner()
                    * 0.99
                    * (CONSTS.max_colors) as f32) as usize,
            ),
            FromFloatColor { child } => {
                BitColor::from_float_color(child.compute(compute_arg.reborrow().into()))
            }
            FromByteColor { child } => {
                BitColor::from_byte_color(child.compute(compute_arg.reborrow().into()))
            }
            FromNibbleIndex { child } => BitColor::from_index(
                child.compute(compute_arg.reborrow().into()).into_inner() as usize % 8,
            ),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow().into()),
                ..compute_arg.reborrow().into()
            }),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate
                    .compute(compute_arg.reborrow().into())
                    .into_inner()
                {
                    child_a.compute(compute_arg.reborrow().into())
                } else {
                    child_b.compute(compute_arg.reborrow().into())
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
                r: r.compute(compute_arg.reborrow().into()),
                g: g.compute(compute_arg.reborrow().into()),
                b: b.compute(compute_arg.reborrow().into()),
                a: a.compute(compute_arg.reborrow().into()),
            },
            FromFloatColor { child } => child.compute(compute_arg.reborrow().into()).into(),
            FromBitColor { child } => child.compute(compute_arg.reborrow().into()).into(),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow().into()),
                ..compute_arg.reborrow().into()
            }),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate
                    .compute(compute_arg.reborrow().into())
                    .into_inner()
                {
                    child_a.compute(compute_arg.reborrow().into())
                } else {
                    child_b.compute(compute_arg.reborrow().into())
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for ByteColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}
