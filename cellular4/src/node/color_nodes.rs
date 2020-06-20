use std::collections::VecDeque;

use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{buffers::*, colors::*, continuous::*, image::*, points::*},
    node::{
        color_blend_nodes::*, continuous_nodes::*, coord_map_nodes::*, discrete_nodes::*,
        mutagen_functions::*, point_nodes::*, point_set_nodes::*, Node,
    },
    updatestate::UpdateState,
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
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

impl<'a> Mutagen<'a> for FloatColorNodes {
    type Arg = UpdateState<'a>;
}
impl Node for FloatColorNodes {
    type Output = FloatColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use FloatColorNodes::*;

        match self {
            Constant { value } => *value,
            FromImage { image } => image
                .get_pixel_normalised(
                    state.coordinate_set.x,
                    state.coordinate_set.y,
                    state.coordinate_set.t,
                )
                .into(),
            FromCellArray => state
                .history
                .get(
                    ((state.coordinate_set.x.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_width as f32) as usize,
                    ((state.coordinate_set.y.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_height as f32) as usize,
                    state.coordinate_set.t as usize,
                )
                .into(),
            Grayscale { child } => {
                let value = child.compute(state);
                FloatColor {
                    r: value,
                    g: value,
                    b: value,
                    a: value,
                }
            }
            RGB { r, g, b, a } => FloatColor {
                r: r.compute(state),
                g: g.compute(state),
                b: b.compute(state),
                a: a.compute(state),
            },
            HSV { h, s, v, a } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_degrees(h.compute(state).into_inner() as f32 * 360.0),
                    s.compute(state).into_inner() as f32,
                    v.compute(state).into_inner() as f32,
                ))
                .into();

                float_color_from_pallette_rgb(rgb, a.compute(state).into_inner())
            }
            FromBlend { child } => child.compute(state),
            FromBitColor { child } => FloatColor::from(child.compute(state)),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            FromByteColor { child } => FloatColor::from(child.compute(state)),
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
            SetAlpha { child_a, child_b } => {
                let mut color = child_a.compute(state);

                color.a = child_b.compute(state);

                color
            }
            PointDrawingBuffer { buffer, .. } => buffer[state.coordinate_set.xy()],
            PointSetLineBuffer { buffer, .. } => buffer[state.coordinate_set.xy()],
            PointSetDotBuffer { buffer, .. } => buffer[state.coordinate_set.xy()],
            ClosestPointLineBuffer { buffer, .. } => buffer[state.coordinate_set.xy()],
        }
    }
}

impl<'a> Updatable<'a> for FloatColorNodes {
    fn update(&mut self, _state: mutagen::State, mut arg: UpdateState<'a>) {
        use FloatColorNodes::*;

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
#[mutagen(mut_reroll = 0.1)]
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

impl<'a> Mutagen<'a> for BitColorNodes {
    type Arg = UpdateState<'a>;
}
impl Node for BitColorNodes {
    type Output = BitColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use BitColorNodes::*;
        match self {
            Constant { value } => *value,
            GiveColor { child_a, child_b } => {
                BitColor::from_components(child_a.compute(state).give_color(child_b.compute(state)))
            }
            TakeColor { child_a, child_b } => {
                BitColor::from_components(child_a.compute(state).take_color(child_b.compute(state)))
            }
            XorColor { child_a, child_b } => {
                BitColor::from_components(child_a.compute(state).xor_color(child_b.compute(state)))
            }
            EqColor { child_a, child_b } => {
                BitColor::from_components(child_a.compute(state).eq_color(child_b.compute(state)))
            }
            FromComponents { r, g, b } => BitColor::from_components([
                r.compute(state).into_inner(),
                g.compute(state).into_inner(),
                b.compute(state).into_inner(),
            ]),
            FromImage { image } => image
                .get_pixel_normalised(
                    state.coordinate_set.x,
                    state.coordinate_set.y,
                    state.coordinate_set.t,
                )
                .into(),
            FromCellArray => state
                .history
                .get(
                    ((state.coordinate_set.x.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_width as f32) as usize,
                    ((state.coordinate_set.y.into_inner() + 1.0)
                        * 0.5
                        * CONSTS.cell_array_height as f32) as usize,
                    state.coordinate_set.t as usize,
                )
                .into(),
            FromUNFloat { child } => BitColor::from_index(
                (child.compute(state).into_inner() * 0.99 * (CONSTS.max_colors) as f32) as usize,
            ),
            FromFloatColor { child } => BitColor::from_float_color(child.compute(state)),
            FromByteColor { child } => BitColor::from_byte_color(child.compute(state)),
            FromNibbleIndex { child } => {
                BitColor::from_index(child.compute(state).into_inner() as usize % 8)
            }
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

impl<'a> Updatable<'a> for BitColorNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {}
}

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
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

impl<'a> Mutagen<'a> for ByteColorNodes {
    type Arg = UpdateState<'a>;
}
impl Node for ByteColorNodes {
    type Output = ByteColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use ByteColorNodes::*;

        match self {
            Constant { value } => *value,
            FromImage { image } => image.get_pixel_normalised(
                state.coordinate_set.x,
                state.coordinate_set.y,
                state.coordinate_set.t,
            ),
            FromCellArray => state.history.get(
                ((state.coordinate_set.x.into_inner() + 1.0) * 0.5 * CONSTS.cell_array_width as f32)
                    as usize,
                ((state.coordinate_set.y.into_inner() + 1.0)
                    * 0.5
                    * CONSTS.cell_array_height as f32) as usize,
                state.coordinate_set.t as usize,
            ),
            Decompose { r, g, b, a } => ByteColor {
                r: r.compute(state).into_inner(),
                g: g.compute(state).into_inner(),
                b: b.compute(state).into_inner(),
                a: a.compute(state).into_inner(),
            },
            FromFloatColor { child } => child.compute(state).into(),
            FromBitColor { child } => child.compute(state).into(),
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

impl<'a> Updatable<'a> for ByteColorNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {}
}
