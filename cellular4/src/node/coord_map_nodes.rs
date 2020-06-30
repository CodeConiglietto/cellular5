use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::{geometry::Point2, geometry::Rotation2};
use serde::{Deserialize, Serialize};

use crate::{
    coordinate_set::*,
    datatype::{continuous::*, points::*},
    mutagen_args::*,
    node::{
        continuous_nodes::*, discrete_nodes::*, matrix_nodes::*, mutagen_functions::*,
        point_nodes::*, point_set_nodes::*, Node,
    },
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum CoordMapNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Replace { child: Box<SNPointNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Shift {
        x: Box<SNFloatNodes>,
        y: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    Scale {
        x: Box<SNFloatNodes>,
        y: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Rotation { angle: Box<AngleNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    ToPolar,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromPolar,
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<CoordMapNodes>,
        child_b: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ApplyMatrix { child: Box<SNFloatMatrix3Nodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    Tessellate {
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
        child_scale: Box<SNPointNodes>,
        point_a: SNPoint,
        point_b: SNPoint,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    TessellatePerPoint {
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    TesellatePolarTwoClosestPointSet { child: Box<PointSetNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    TesellateClosestPointSet { child: Box<PointSetNodes> },
}

impl Node for CoordMapNodes {
    type Output = CoordinateSet;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
        use CoordMapNodes::*;

        match self {
            Replace { child } => {
                compute_arg
                    .replace_coords(&child.compute(compute_arg))
                    .coordinate_set
            }
            Shift { x, y } => compute_arg.coordinate_set.get_coord_shifted(
                x.compute(compute_arg),
                y.compute(compute_arg),
                SNFloat::new(0.0),
            ),
            Scale { x, y } => compute_arg.coordinate_set.get_coord_scaled(
                x.compute(compute_arg),
                y.compute(compute_arg),
                SNFloat::new(1.0),
            ),
            Rotation { angle } => {
                let new_pos = Rotation2::new(angle.compute(compute_arg).into_inner())
                    .transform_point(&Point2::new(
                        compute_arg.coordinate_set.x.into_inner(),
                        compute_arg.coordinate_set.y.into_inner(),
                    ));

                CoordinateSet {
                    x: SNFloat::new(0.0).sawtooth_add_f32(new_pos.x),
                    y: SNFloat::new(0.0).sawtooth_add_f32(new_pos.y),
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
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(compute_arg).into_inner() {
                    child_a.compute(compute_arg)
                } else {
                    child_b.compute(compute_arg)
                }
            }
            ApplyMatrix { child } => {
                let point = Point2::new(
                    compute_arg.coordinate_set.x.into_inner(),
                    compute_arg.coordinate_set.y.into_inner(),
                )
                .to_homogeneous();

                let result =
                    Point2::from_homogeneous(child.compute(compute_arg).into_inner() * point)
                        .unwrap();

                CoordinateSet {
                    x: SNFloat::new_triangle(result.coords.x),
                    y: SNFloat::new_triangle(result.coords.y),
                    t: compute_arg.coordinate_set.t,
                }
            }
            Tessellate {
                child_a,
                child_b,
                child_scale,
                point_a,
                point_b,
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
                let a = child_a.compute(compute_arg).into_inner();
                let b = child_b.compute(compute_arg).into_inner();

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
                let closest = child.compute(compute_arg).get_closest_point(p);

                let offset =
                    SNPoint::new(Point2::from(p.into_inner() - closest.into_inner()) * 0.5);

                CoordinateSet {
                    x: offset.x(),
                    y: offset.y(),
                    t: compute_arg.coordinate_set.t,
                }
            }

            TesellatePolarTwoClosestPointSet { child } => {
                let p = compute_arg.coordinate_set.get_coord_point();
                let mut point_set = child.compute(compute_arg);
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

                let offset =
                    SNPoint::from_snfloats(polar_1.x(), UNFloat::new(y_result).to_signed())
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

    fn update(&mut self, _state: mutagen::State, arg: UpdArg<'a>) {
        match self {
            CoordMapNodes::Tessellate {
                child_a,
                child_b,
                child_scale,
                ref mut point_a,
                ref mut point_b,
            } => {
                let mut state_a = arg.clone();
                state_a.coordinate_set.x = point_a.x();
                state_a.coordinate_set.y = point_a.y();

                let mut state_b = arg.clone();
                state_b.coordinate_set.x = point_b.x();
                state_b.coordinate_set.y = point_b.y();

                let translation_scale = child_scale.compute(arg).scale_unfloat(UNFloat::new(0.025));

                *point_a =
                    point_a.sawtooth_add(child_a.compute(&state_a).scale_point(translation_scale));
                *point_b =
                    point_b.sawtooth_add(child_b.compute(&state_b).scale_point(translation_scale));
            }
            _ => (),
        }
    }
}
