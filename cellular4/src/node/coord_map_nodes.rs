use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use nalgebra::{geometry::Point2, geometry::Rotation2};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum CoordMapNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Replace { child: Box<SNPointNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Shift {
        x: Box<SNFloatNodes>,
        y: Box<SNFloatNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    Scale {
        x: Box<SNFloatNodes>,
        y: Box<SNFloatNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Rotation {
        child_angle: Box<AngleNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },

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
    ApplyMatrix {
        child_matrix: Box<SNFloatMatrix3Nodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Tessellate {
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
        child_scale: Box<SNPointNodes>,
        child_normaliser: Box<SFloatNormaliserNodes>,
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

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use CoordMapNodes::*;

        match self {
            Replace { child } => {
                let coords = child.compute(compute_arg.reborrow());
                compute_arg
                    .reborrow()
                    .replace_coords(&coords)
                    .coordinate_set
            }
            Shift {
                x,
                y,
                child_normaliser,
            } => compute_arg.coordinate_set.get_coord_shifted(
                x.compute(compute_arg.reborrow()),
                y.compute(compute_arg.reborrow()),
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

            TesellatePolarTwoClosestPointSet { child } => {
                let p = compute_arg.coordinate_set.get_coord_point();
                let mut point_set = child.compute(compute_arg.reborrow());
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

    fn update(&mut self, _state: mutagen::State, mut arg: UpdArg<'a>) {
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
