use std::collections::VecDeque;

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{buffers::*, colors::*, image::*, points::*},
    mutagen_args::*,
    node::{
        color_blend_nodes::*, continuous_nodes::*, coord_map_nodes::*, discrete_nodes::*,
        mutagen_functions::*, point_nodes::*, point_set_nodes::*, Node,
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
}

impl Node for FloatColorNodes {
    type Output = FloatColor;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
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
            HSV { h, s, v, a } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_degrees(h.compute(compute_arg.reborrow()).into_inner() as f32 * 360.0),
                    s.compute(compute_arg.reborrow()).into_inner() as f32,
                    v.compute(compute_arg.reborrow()).into_inner() as f32,
                ))
                .into();

                float_color_from_pallette_rgb(rgb, a.compute(compute_arg.reborrow()).into_inner())
            }
            FromBlend { child } => child.compute(compute_arg.reborrow()),
            FromBitColor { child } => FloatColor::from(child.compute(compute_arg.reborrow())),
            ModifyState { child, child_state } => child.compute(ComArg {
                coordinate_set: child_state.compute(compute_arg.reborrow()),
                ..compute_arg.reborrow()
            }),
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
            PointDrawingBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
            PointSetLineBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
            PointSetDotBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
            ClosestPointLineBuffer { buffer, .. } => buffer[compute_arg.coordinate_set.xy()],
        }
    }
}

impl<'a> Updatable<'a> for FloatColorNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: UpdArg<'a>) {
        use FloatColorNodes::*;
        let arg = ComArg::from(arg.reborrow());

        match self {
            PointDrawingBuffer {
                buffer,
                child,
                child_color,
                points_len_child,
                points,
            } => {
                let last_point = points.back().copied().unwrap_or_else(SNPoint::zero);

                arg.coordinate_set.x = last_point.x();
                arg.coordinate_set.y = last_point.y();

                let new_point = last_point.sawtooth_add(child.compute(arg));

                let points_len = points_len_child.compute(arg);

                let color = child_color.compute(arg);

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
                let source = child_point.compute(arg);

                for dest in child_point_set.compute(arg).points.iter() {
                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();
                    let color = child_color.compute(arg);

                    buffer.draw_line(source, *dest, color.clone())
                }
            }

            PointSetDotBuffer {
                buffer,
                child_point_set,
                child_color,
            } => {
                for dest in child_point_set.compute(arg).points.iter() {
                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();
                    let color = child_color.compute(arg);

                    buffer.draw_dot(*dest, color.clone());
                }
            }

            ClosestPointLineBuffer {
                buffer,
                child_a,
                child_b,
                child_color,
            } => {
                let set_a = child_a.compute(arg);
                let set_b = child_b.compute(arg);

                for source in set_a.points.iter() {
                    let dest = set_b.get_closest_point(*source);

                    arg.coordinate_set.x = dest.x();
                    arg.coordinate_set.y = dest.y();
                    let color = child_color.compute(arg);

                    buffer.draw_line(*source, dest, color.clone())
                }
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

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
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
                (child.compute(compute_arg.reborrow()).into_inner() * 0.99 * (CONSTS.max_colors) as f32)
                    as usize,
            ),
            FromFloatColor { child } => BitColor::from_float_color(child.compute(compute_arg.reborrow())),
            FromByteColor { child } => BitColor::from_byte_color(child.compute(compute_arg.reborrow())),
            FromNibbleIndex { child } => {
                BitColor::from_index(child.compute(compute_arg.reborrow()).into_inner() as usize % 8)
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

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
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
