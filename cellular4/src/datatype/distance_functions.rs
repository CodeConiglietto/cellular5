use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{constraint_resolvers::*, continuous::*, points::*},
    mutagen_args::*,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, UpdatableRecursively)]
pub enum DistanceFunction {
    Euclidean,
    Manhattan,
    Chebyshev,
    Minimum,
    //Minkowski,
}

//wrapped in triangle waves for now, maybe parametrise SN resolution method
impl DistanceFunction {
    pub fn calculate_point2(self, a: Point2<f32>, b: Point2<f32>) -> f32 {
        let new_point = b - a;
        let x = new_point.x;
        let y = new_point.y;

        use DistanceFunction::*;

        match self {
            Euclidean => distance(&a, &b) * 0.5,
            Manhattan => (x.abs() + y.abs()) * 0.5,
            Chebyshev => (x.abs()).max(y.abs()),
            Minimum => (x.abs()).min(y.abs()),
        }
    }

    pub fn calculate_normalised(
        self,
        a: SNPoint,
        b: SNPoint,
        normaliser: &UFloatNormaliser,
    ) -> UNFloat {
        normaliser.normalise(self.calculate_point2(a.into_inner(), b.into_inner()))
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0..4) {
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

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for DistanceFunction {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for DistanceFunction {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: Self::UpdateArg) {}
}
