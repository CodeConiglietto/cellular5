use mutagen::{Generatable, Mutatable};
use nalgebra::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{continuous::*, points::*},
    mutagen_args::*,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DistanceFunction {
    Euclidean,
    Manhattan,
    Chebyshev,
    Minimum,
    //Minkowski,
}

//wrapped in triangle waves for now, maybe parametrise SN resolution method
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

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0, 4) {
            0 => DistanceFunction::Euclidean,
            1 => DistanceFunction::Manhattan,
            2 => DistanceFunction::Chebyshev,
            3 => DistanceFunction::Minimum,
            _ => unreachable!(),
        }
    }
}

impl<'a> Generatable<'a> for DistanceFunction {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for DistanceFunction {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State, arg: MutArg<'a>) {
        *self = Self::random(rng);
    }
}
