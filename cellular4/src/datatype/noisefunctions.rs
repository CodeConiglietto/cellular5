use mutagen::{Generatable, Mutatable};
use noise::{
    BasicMulti, Billow, Checkerboard, Fbm, HybridMulti, NoiseFn, OpenSimplex, RangeFunction,
    RidgedMulti, Seedable, SuperSimplex, Value, Worley,
};
use rand::prelude::*;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

use crate::{
    datatype::{
        continuous::UNFloat,
        discrete::{Boolean, Nibble},
    },
    mutagen_args::*,
};

#[derive(Serialize, Deserialize, Generatable, Mutatable, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum NoiseFunctions {
    BasicMulti(Noise<BasicMulti>),
    Billow(Noise<Billow>),
    Checkerboard(Noise<Checkerboard>),
    Fbm(Noise<Fbm>),
    HybridMulti(Noise<HybridMulti>),
    OpenSimplex(Noise<OpenSimplex>),
    RidgedMulti(Noise<RidgedMulti>),
    SuperSimplex(Noise<SuperSimplex>),
    Value(Noise<Value>),
    Worley(Noise<Worley>),
}

impl NoiseFunctions {
    pub fn compute(&self, x: f64, y: f64, t: f64) -> f64 {
        match self {
            NoiseFunctions::BasicMulti(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::Billow(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::Checkerboard(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::Fbm(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::HybridMulti(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::OpenSimplex(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::RidgedMulti(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::SuperSimplex(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::Value(noise) => noise.noise.get([x, y, t]),
            NoiseFunctions::Worley(noise) => noise.noise.get([x, y, t]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Noise<T: NoiseFunction> {
    noise: T,
    params: T::Params,
}

impl<T> Serialize for Noise<T>
where
    T: NoiseFunction,
    T::Params: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.params.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Noise<T>
where
    T: NoiseFunction,
    T::Params: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let params = T::Params::deserialize(deserializer)?;
        Ok(Self {
            noise: T::new(&params),
            params,
        })
    }
}

impl<'a, T> Generatable<'a> for Noise<T>
where
    T: NoiseFunction,
    T::Params: Generatable<'a>,
{
    type GenArg = <T::Params as Generatable<'a>>::GenArg;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        state: mutagen::State,
        arg: &'a mut Self::GenArg,
    ) -> Self {
        let params = T::Params::generate_rng(rng, state, arg);

        Self {
            noise: T::new(&params),
            params,
        }
    }
}

impl<'a, T> Mutatable<'a> for Noise<T>
where
    T: NoiseFunction,
    T::Params: Mutatable<'a>,
{
    type MutArg = <T::Params as Mutatable<'a>>::MutArg;

    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: &'a mut Self::MutArg,
    ) {
        self.params.mutate_rng(rng, state, arg);
        self.noise = T::new(&self.params);
    }
}

pub trait NoiseFunction {
    type Params;
    fn new(params: &Self::Params) -> Self;
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SeedParams {
    pub seed: u32,
}

impl SeedParams {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self { seed: rng.gen() }
    }
}

impl<'a> Generatable<'a> for SeedParams {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: &'a mut GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for SeedParams {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        _state: mutagen::State,
        _arg: &'a mut MutArg<'a>,
    ) {
        *self = Self::random(rng);
    }
}

impl NoiseFunction for BasicMulti {
    type Params = SeedParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_seed(params.seed)
    }
}

impl NoiseFunction for Billow {
    type Params = SeedParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_seed(params.seed)
    }
}

impl NoiseFunction for Checkerboard {
    type Params = CheckerboardParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_size(usize::from(params.size.into_inner() + 1))
    }
}

#[derive(Serialize, Deserialize, Generatable, Mutatable, Debug, Clone, Copy)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub struct CheckerboardParams {
    pub size: Nibble,
}

impl NoiseFunction for Fbm {
    type Params = SeedParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_seed(params.seed)
    }
}

impl NoiseFunction for HybridMulti {
    type Params = SeedParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_seed(params.seed)
    }
}

impl NoiseFunction for OpenSimplex {
    type Params = SeedParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_seed(params.seed)
    }
}

impl NoiseFunction for RidgedMulti {
    type Params = RidgedMultiParams;

    fn new(params: &Self::Params) -> Self {
        Self::default()
            .set_attenuation(f64::from(params.attenuation.into_inner()) * 8.0)
            .set_seed(params.seed.seed)
    }
}

#[derive(Serialize, Deserialize, Generatable, Mutatable, Debug, Clone)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub struct RidgedMultiParams {
    pub attenuation: UNFloat,
    #[serde(flatten)]
    pub seed: SeedParams,
}

impl NoiseFunction for SuperSimplex {
    type Params = SeedParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_seed(params.seed)
    }
}

impl NoiseFunction for Value {
    type Params = SeedParams;

    fn new(params: &Self::Params) -> Self {
        Self::default().set_seed(params.seed)
    }
}

impl NoiseFunction for Worley {
    type Params = WorleyParams;

    fn new(params: &Self::Params) -> Self {
        Self::default()
            .enable_range(params.enable_range.into_inner())
            .set_range_function(params.range_function.into())
            .set_displacement(f64::from(params.displacement.into_inner()))
            .set_seed(params.seed.seed)
    }
}

#[derive(Generatable, Mutatable, Serialize, Deserialize, Debug, Clone)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub struct WorleyParams {
    pub range_function: RangeFunctionParam,
    pub enable_range: Boolean,
    pub displacement: UNFloat,
    #[serde(flatten)]
    pub seed: SeedParams,
}

#[derive(Generatable, Mutatable, Serialize, Deserialize, Debug, Clone, Copy)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum RangeFunctionParam {
    Euclidean,
    EuclideanSquared,
    Manhattan,
    Chebyshev,
    Quadratic,
}

impl From<RangeFunctionParam> for RangeFunction {
    fn from(f: RangeFunctionParam) -> Self {
        match f {
            RangeFunctionParam::Euclidean => RangeFunction::Euclidean,
            RangeFunctionParam::EuclideanSquared => RangeFunction::EuclideanSquared,
            RangeFunctionParam::Manhattan => RangeFunction::Manhattan,
            RangeFunctionParam::Chebyshev => RangeFunction::Chebyshev,
            RangeFunctionParam::Quadratic => RangeFunction::Quadratic,
        }
    }
}
