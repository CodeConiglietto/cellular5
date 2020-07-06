use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use nalgebra::*;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::points::*,
    mutagen_args::*,
    node::{continuous_nodes::*, mutagen_functions::*, point_set_nodes::*, Node},
};

//Note: SNPoints are not normalised in the mathematical sense, each coordinate is simply capped at -1..1
#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum SNPointNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Zero,
    #[mutagen(gen_weight = leaf_node_weight)]
    Coordinate,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SNPoint },
    #[mutagen(gen_weight = pipe_node_weight)]
    Invert { child: Box<SNPointNodes> },
    #[mutagen(gen_weight = branch_node_weight)]
    FromSNFloats {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    SawtoothAdd {
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    TriangleAdd {
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    IterativeSawtoothAdd {
        value: SNPoint,
        child: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    GetClosestPointInSet { child: Box<PointSetNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    GetFurthestPointInSet { child: Box<PointSetNodes> },
}

impl Node for SNPointNodes {
    type Output = SNPoint;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNPointNodes::*;

        match self {
            Zero => SNPoint::zero(),
            Coordinate => compute_arg.coordinate_set.get_coord_point(),
            Constant { value } => *value,
            Invert { child } => {
                let point = child.compute(compute_arg.reborrow()).into_inner();
                SNPoint::new(Point2::new(point.x * -1.0, point.y * -1.0))
            }
            FromSNFloats { child_a, child_b } => SNPoint::new(Point2::new(
                child_a.compute(compute_arg.reborrow()).into_inner(),
                child_b.compute(compute_arg.reborrow()).into_inner(),
            )),
            SawtoothAdd { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .sawtooth_add(child_b.compute(compute_arg.reborrow())),
            TriangleAdd { child_a, child_b } => child_a
                .compute(compute_arg.reborrow())
                .triangle_add(child_b.compute(compute_arg.reborrow())),
            IterativeSawtoothAdd { value, .. } => *value,
            GetClosestPointInSet { child } => child
                .compute(compute_arg.reborrow())
                .get_closest_point(compute_arg.coordinate_set.get_coord_point()),
            GetFurthestPointInSet { child } => child
                .compute(compute_arg.reborrow())
                .get_furthest_point(compute_arg.coordinate_set.get_coord_point()),
        }
    }
}

impl<'a> Updatable<'a> for SNPointNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: UpdArg<'a>) {
        use SNPointNodes::*;

        match self {
            IterativeSawtoothAdd { value, child } => {
                *value = value.sawtooth_add(child.compute(arg.into()));
            }
            _ => {}
        }
    }
}
