use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::points::*,
    node::{continuous_nodes::*, mutagen_functions::*, point_set_nodes::*, Node},
    updatestate::*,
};

//Note: SNPoints are not normalised in the matematical sense, each coordinate is simply capped at -1..1
#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
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
    IterativeCircularAdd {
        value: SNPoint,
        child: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    GetClosestPointInSet { child: Box<PointSetNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    GetFurthestPointInSet { child: Box<PointSetNodes> },
}

impl<'a> Mutagen<'a> for SNPointNodes {
    type Arg = UpdateState<'a>;
}
impl Node for SNPointNodes {
    type Output = SNPoint;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNPointNodes::*;

        match self {
            Zero => SNPoint::zero(),
            Coordinate => state.coordinate_set.get_coord_point(),
            Constant { value } => *value,
            Invert { child } => {
                let point = child.compute(state).into_inner();
                SNPoint::new(Point2::new(point.x * -1.0, point.y * -1.0))
            }
            FromSNFloats { child_a, child_b } => SNPoint::new(Point2::new(
                child_a.compute(state).into_inner(),
                child_b.compute(state).into_inner(),
            )),
            SawtoothAdd { child_a, child_b } => {
                child_a.compute(state).sawtooth_add(child_b.compute(state))
            }
            TriangleAdd { child_a, child_b } => {
                child_a.compute(state).triangle_add(child_b.compute(state))
            }
            IterativeCircularAdd { value, child } => *value,
            GetClosestPointInSet { child } => child
                .compute(state)
                .get_closest_point(state.coordinate_set.get_coord_point()),
            GetFurthestPointInSet { child } => child
                .compute(state)
                .get_furthest_point(state.coordinate_set.get_coord_point()),
        }
    }
}

impl<'a> Updatable<'a> for SNPointNodes {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        use SNPointNodes::*;

        match self {
            IterativeCircularAdd { value, child } => {}
            _ => {}
        }
    }
}
