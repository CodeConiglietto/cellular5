use std::{collections::VecDeque, f32::consts::PI};

use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, Lab, Limited, RgbHue};
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

    //Brute force attempt to get more visually satisfying behaviour
    #[mutagen(gen_weight = pipe_node_weight)]
    AbsChildCoords { child: NodeBox<FloatColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    RGB {
        r: NodeBox<UNFloatNodes>,
        g: NodeBox<UNFloatNodes>,
        b: NodeBox<UNFloatNodes>,
        a: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    HSV {
        h: NodeBox<UNFloatNodes>,
        s: NodeBox<UNFloatNodes>,
        v: NodeBox<UNFloatNodes>,
        a: NodeBox<UNFloatNodes>,
        offset: UNFloat,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    HSVAngle {
        h: NodeBox<AngleNodes>,
        s: NodeBox<UNFloatNodes>,
        v: NodeBox<UNFloatNodes>,
        a: NodeBox<UNFloatNodes>,
        offset: Angle,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    CMYK {
        c: NodeBox<UNFloatNodes>,
        m: NodeBox<UNFloatNodes>,
        y: NodeBox<UNFloatNodes>,
        k: NodeBox<UNFloatNodes>,
        a: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    LAB {
        l: NodeBox<UNFloatNodes>,
        a: NodeBox<SNFloatNodes>,
        b: NodeBox<SNFloatNodes>,
        alpha: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    // #[mutagen(gen_preferred)]
    ComplexLAB {
        l: NodeBox<UNFloatNodes>,
        child_complex: NodeBox<SNComplexNodes>,
        alpha: NodeBox<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    // #[mutagen(gen_preferred)]
    IterativeResultLAB {
        child_iterative_function: NodeBox<IterativeFunctionNodes>,
        alpha: NodeBox<UNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromBlend { child: NodeBox<ColorBlendNodes> },

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
    // #[mutagen(gen_preferred)]
    TriangleAberration {
        child: NodeBox<FloatColorNodes>,
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    // #[mutagen(gen_preferred)]
    TriangleBlendAutomata{
        child_rho: NodeBox<UNFloatNodes>,
        child_theta: NodeBox<AngleNodes>,
        child_normaliser: NodeBox<SFloatNormaliserNodes>,
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
    // #[mutagen(gen_preferred)]
    PointSetLineBuffer {
        buffer: Buffer<FloatColor>,
        child_point_set: NodeBox<PointSetNodes>,
        child_point: NodeBox<SNPointNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    PointSetDotBuffer {
        buffer: Buffer<FloatColor>,
        child_point_set: NodeBox<PointSetNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IterativePolarLineBuffer {
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
        buffer: Buffer<FloatColor>,
        child_a: NodeBox<PointSetNodes>,
        child_b: NodeBox<PointSetNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    NextPointLineBuffer {
        buffer: Buffer<FloatColor>,
        child_set: NodeBox<PointSetNodes>,
        child_index: NodeBox<ByteNodes>,
        child_color: NodeBox<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    // #[mutagen(gen_preferred)]
    DucksFractal {
        child_offset: NodeBox<SNPointNodes>,
        child_scale: NodeBox<SNPointNodes>,
        child_iterations: NodeBox<ByteNodes>,
        child_magnitude_normaliser: NodeBox<UFloatNormaliserNodes>,
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
            TriangleBlendAutomata {
                child_rho,
                child_theta,
                child_normaliser,
            } => {
                let rho = UNFloat::new(0.1);//child_rho.compute(compute_arg.reborrow());
                let theta = Angle::new(0.0);//child_theta.compute(compute_arg.reborrow());
                let normaliser = child_normaliser.compute(compute_arg.reborrow());

                let theta_a = theta;
                let theta_b = theta + Angle::new(2.0 / 3.0 * PI);
                let theta_c = theta - Angle::new(2.0 / 3.0 * PI);

                // TODO rewrite this once we have a polar type node
                let middle = compute_arg.coordinate_set.get_coord_point();

                let t = compute_arg.reborrow().current_t;

                let color_a = compute_arg.reborrow().history.get_normalised(middle.normalised_add(
                                SNPoint::from_snfloats(
                                    SNFloat::new(rho.into_inner() * f32::sin(theta_a.into_inner())),
                                    SNFloat::new(rho.into_inner() * f32::cos(theta_a.into_inner())),
                                ),
                                normaliser,
                            ), t);

                let color_b = compute_arg.reborrow().history.get_normalised(middle.normalised_add(
                    SNPoint::from_snfloats(
                                    SNFloat::new(rho.into_inner() * f32::sin(theta_b.into_inner())),
                                    SNFloat::new(rho.into_inner() * f32::cos(theta_b.into_inner())),
                                ),
                                normaliser,
                            ), t);

                let color_c = compute_arg.reborrow().history.get_normalised(middle.normalised_add(
                    SNPoint::from_snfloats(
                                    SNFloat::new(rho.into_inner() * f32::sin(theta_c.into_inner())),
                                    SNFloat::new(rho.into_inner() * f32::cos(theta_c.into_inner())),
                                ),
                                normaliser,
                            ), t);
                
                let r = UNFloat::new((color_a.r.into_inner() + color_b.r.into_inner() + color_c.r.into_inner()) / 3.0);
                let g = UNFloat::new((color_a.g.into_inner() + color_b.g.into_inner() + color_c.g.into_inner()) / 3.0);
                let b = UNFloat::new((color_a.b.into_inner() + color_b.b.into_inner() + color_c.b.into_inner()) / 3.0);
                let a = UNFloat::new((color_a.a.into_inner() + color_b.a.into_inner() + color_c.a.into_inner()) / 3.0);

                let average = FloatColor { r, g, b, a };

                if average.get_average() > 0.5 {
                    FloatColor::WHITE
                } else {
                    FloatColor::BLACK
                }
            }
            AbsChildCoords { child } => {
                let new_point = compute_arg.coordinate_set.get_coord_point().abs();
                child.compute(compute_arg.reborrow().replace_coords(&new_point))
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
            RemoveAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow());

                if child_b.compute(compute_arg.reborrow()).into_inner() {color.a = UNFloat::ZERO;}

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

            DucksFractal {//TODO make gooderer
                child_offset,
                child_scale,
                child_iterations,
                child_magnitude_normaliser,
            } => {
                let offset = child_offset.compute(compute_arg.reborrow()).into_inner();
                let scale = child_scale.compute(compute_arg.reborrow()).into_inner();
                let iterations = //50;
                128 - (128 - compute_arg.coordinate_set.get_byte_t().into_inner() as i32).abs();
                // 1 + child_iterations
                //     .compute(compute_arg.reborrow())
                //     .into_inner()
                //     / 4;

                // x and y are swapped intentionally
                let c = Complex::new(
                    f64::from(//0.001 *
                        (1.0 - compute_arg.coordinate_set.get_unfloat_t().into_inner()) * 
                    // scale.y * 
                    compute_arg.coordinate_set.y.into_inner()// - 0.5
                ),
                    f64::from(//0.001 *
                        (1.0 - compute_arg.coordinate_set.get_unfloat_t().into_inner()) * 
                    // scale.x * 
                    compute_arg.coordinate_set.x.into_inner()// - 0.5
                ),
                );

                let c_offset =
                        Complex::new(
                            // compute_arg.coordinate_set.get_unfloat_t().into_inner() as f64
                            // f64::from(scale.y) *
                            // f64::from(offset.y)
                            0.0
                            ,
                            // f64::from(scale.x) *
                            // f64::from(offset.x)
                            // compute_arg.coordinate_set.get_unfloat_t().into_inner() as f64
                            0.0
                            ,
                        );

                let mut magnitude = 0.0;

                let (z_final, _escape) = escape_time_system(
                    c// + c_offset
                    ,
                    iterations as usize,
                    |z, _i| (
                        // if z.im > 0.0 { z } else { z.conj() } 
                        z.abs() + c).ln(),
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
                
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_degrees(
                        // magnitude as f32,
                            // child_magnitude_normaliser.compute(compute_arg.reborrow()).normalise(magnitude as f32).into_inner()
                    UFloatNormaliser::Sawtooth.normalise(//compute_arg.coordinate_set.get_unfloat_t().into_inner() + 
                    magnitude as f32).into_inner()
                            
                            * 360.0,
                    ),
                    1.0 as f32,
                    // child_magnitude_normaliser.compute(compute_arg.reborrow()).normalise(magnitude as f32).into_inner(),
                    // UFloatNormaliser::Sawtooth.normalise(magnitude as f32).into_inner()
                    0.5,
                ))
                .into();

                float_color_from_pallette_rgb(rgb, 1.0)
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
    FromFloatColor { child: NodeBox<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromByteColor { child: NodeBox<ByteColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromNibbleIndex { child: NodeBox<NibbleNodes> },

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
    #[mutagen(gen_weight = pipe_node_weight)]
    FromFloatColor { child: NodeBox<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromBitColor { child: NodeBox<FloatColorNodes> },

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
            RemoveAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow());

                if child_b.compute(compute_arg.reborrow()).into_inner() {color.a = Byte::new(0);}

                color
            }
            SetAlpha { child_a, child_b } => {
                let mut color = child_a.compute(compute_arg.reborrow());

                color.a = child_b.compute(compute_arg.reborrow());

                color
            }
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

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
