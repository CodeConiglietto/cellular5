use mutagen::{Generatable, Mutagen, Mutatable};
use nalgebra::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{continuous::*, points::*},
    updatestate::UpdateState,
};

#[derive(Clone, Copy, Debug, Generatable, Mutatable, Serialize, Deserialize)]
#[mutagen(mut_reroll = 1.0)]
pub enum DistanceFunction {
    Euclidean,
    Manhattan,
    Chebyshev,
    Minimum,
    //Minkowski,
}

//wrapped in triangle waves for now, maybe parameterise SN resolution method
impl DistanceFunction {
    pub fn calculate(self, a: SNPoint, b: SNPoint) -> UNFloat {
        let new_point = a.into_inner() - b.into_inner();
        let x = new_point.x;
        let y = new_point.y;

        use DistanceFunction::*;

        match self {
            Euclidean => UNFloat::new_triangle(distance(&a.into_inner(), &b.into_inner()) * 0.5),
            Manhattan => UNFloat::new_triangle((x.abs() + y.abs()) * 0.5),
            Chebyshev => UNFloat::new_triangle((x.abs()).max(y.abs())),
            Minimum => UNFloat::new_triangle((x.abs()).min(y.abs())),
        }
    }
}

impl<'a> Mutagen<'a> for DistanceFunction {
    type Arg = UpdateState<'a>;
}
