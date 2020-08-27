use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use lazy_static::lazy_static;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use num::traits::identities::Zero;
use rand::prelude::*;
use regex::Regex;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};

use crate::{
    datatype::{constraint_resolvers::*, continuous::*, points::*},
    mutagen_args::*,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SNComplex {
    value: Complex<f32>,
}

impl SNComplex {
    pub fn new_unchecked(value: Complex<f32>) -> Self {
        Self { value }
    }

    pub fn new(value: Complex<f32>) -> Self {
        assert!(
            value.re >= -1.0 && value.re <= 1.0 && value.im >= -1.0 && value.im <= 1.0,
            "Invalid Complex value: {}",
            value
        );

        Self::new_unchecked(value)
    }

    pub fn new_normalised(value: Complex<f32>, normaliser: SFloatNormaliser) -> Self {
        Self::from_snfloats(normaliser.normalise(value.re), normaliser.normalise(value.im))
    }

    pub fn from_snfloats(x: SNFloat, y: SNFloat) -> Self {
        Self::new_unchecked(Complex::new(x.into_inner(), y.into_inner()))
    }

    pub fn from_snpoint(value: SNPoint) -> Self {
        Self::new_unchecked(Complex::new(value.x().into_inner(), value.y().into_inner()))
    }

    pub fn zero() -> Self {
        Self::new(Complex::zero())
    }

    pub fn into_inner(self) -> Complex<f32> {
        self.value
    }

    pub fn re(self) -> SNFloat {
        SNFloat::new_unchecked(self.value.re)
    }

    pub fn im(self) -> SNFloat {
        SNFloat::new_unchecked(self.value.im)
    }

    pub fn normalised_add(self, other: SNComplex, normaliser: SFloatNormaliser) -> SNComplex {
        SNComplex::new_normalised(
            self.value + other.into_inner(),
            normaliser,
        )
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new(Complex::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        ))
    }
}

impl Serialize for SNComplex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SNComplex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SNComplexVisitor)
    }
}

struct SNComplexVisitor;

impl<'de> Visitor<'de> for SNComplexVisitor {
    type Value = SNComplex;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a complex like '(0.0, 0.0)'")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"\(\s*(-?[\d\.]+)\s*,\s*(-?[\d\.]+)\s*\)"#).unwrap();
        }

        let caps = RE
            .captures(v)
            .ok_or_else(|| E::custom(format!("Invalid complex: {}", v)))?;

        let x = f32::from_str(&caps[1]).map_err(|e| E::custom(e.to_string()))?;
        let y = f32::from_str(&caps[2]).map_err(|e| E::custom(e.to_string()))?;

        if x < -1.0 || x > 1.0 || y < -1.0 || y > 1.0 {
            return Err(E::custom(format!("SNComplex out of range: {}", v)));
        }

        Ok(SNComplex::new(Complex::new(x, y)))
    }
}

impl Display for SNComplex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.re(), self.im())
    }
}

impl Default for SNComplex {
    fn default() -> Self {
        Self::new(Complex::new(f32::default(), f32::default()))
    }
}

impl<'a> Generatable<'a> for SNComplex {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for SNComplex {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State, arg: MutArg<'a>) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for SNComplex {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for SNComplex {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snpoint_deserialize() {
        let a = SNComplex::new(Complex::new(-0.5, 1.0));
        let b: SNComplex = serde_yaml::from_str(&serde_yaml::to_string(&a).unwrap()).unwrap();
        assert_eq!(a, b);
    }
}
