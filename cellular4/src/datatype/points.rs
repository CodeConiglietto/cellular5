use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use lazy_static::lazy_static;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use rand::prelude::*;
use regex::Regex;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};

use crate::{
    datatype::{constraint_resolvers::*, continuous::*, complex::*},
    mutagen_args::*,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SNPoint {
    value: Point2<f32>,
}

impl SNPoint {
    pub fn new_unchecked(value: Point2<f32>) -> Self {
        Self { value }
    }

    pub fn new(value: Point2<f32>) -> Self {
        assert!(
            value.x >= -1.0 && value.y <= 1.0 && value.x >= -1.0 && value.y <= 1.0,
            "Invalid SNPoint value: {}",
            value
        );

        Self::new_unchecked(value)
    }

    pub fn new_normalised(value: Point2<f32>, normaliser: SFloatNormaliser) -> Self {
        Self::from_snfloats(normaliser.normalise(value.x), normaliser.normalise(value.y))
    }

    // TODO Figure out what this does, if possible replace distance function with distance function enum
    pub fn subtract_normalised(&self, other: SNPoint) -> Self {
        let result = self.into_inner() - other.into_inner();
        Self::new(
            Point2::new(result.x, result.y)
                / distance(&self.into_inner(), &other.into_inner()).max(0.1),
        )
    }

    pub fn from_snfloats(x: SNFloat, y: SNFloat) -> Self {
        Self::new_unchecked(Point2::new(x.into_inner(), y.into_inner()))
    }

    pub fn zero() -> Self {
        Self::new(Point2::origin())
    }

    pub fn into_inner(self) -> Point2<f32> {
        self.value
    }

    pub fn x(self) -> SNFloat {
        SNFloat::new_unchecked(self.value.x)
    }

    pub fn y(self) -> SNFloat {
        SNFloat::new_unchecked(self.value.y)
    }

    pub fn to_angle(self) -> Angle {
        Angle::new(f32::atan2(self.value.x, self.value.y))
    }

    pub fn average(self, other: Self) -> Self {
        Self::new(Point2::from(self.into_inner().coords + other.into_inner().coords) * 0.5)
    }

    pub fn normalised_add(self, other: SNPoint, normaliser: SFloatNormaliser) -> SNPoint {
        SNPoint::from_snfloats(
            self.x().normalised_add(other.x(), normaliser),
            self.y().normalised_add(other.y(), normaliser),
        )
    }

    pub fn scale(self, other: SNFloat) -> SNPoint {
        SNPoint::from_snfloats(self.x().multiply(other), self.y().multiply(other))
    }

    pub fn scale_unfloat(self, other: UNFloat) -> SNPoint {
        SNPoint::from_snfloats(
            self.x().multiply_unfloat(other),
            self.y().multiply_unfloat(other),
        )
    }

    pub fn scale_point(self, other: SNPoint) -> SNPoint {
        SNPoint::from_snfloats(self.x().multiply(other.x()), self.y().multiply(other.y()))
    }

    pub fn to_polar(self) -> Self {
        //Represents the angle from 0.0..2PI
        //atan2(y, x) is correct, but it's more visually appealing to have the axis of symmetry along the vertical axis
        //Sorry if this makes me a bad person :<
        let theta = Angle::new(f32::atan2(-self.value.x, self.value.y));
        //Represents the radius between 0.0..1.0
        let rho = UNFloat::new(f32::sqrt(self.value.x.powf(2.0) + self.value.y.powf(2.0)).min(1.0));

        Self::from_snfloats(theta.to_signed(), rho.to_signed())
    }

    // TODO Refactor this when polar point datatype is added
    #[allow(clippy::wrong_self_convention)]
    pub fn from_polar(self) -> Self {
        let theta = self.x().to_angle().into_inner();
        let rho = self.y().to_unsigned().into_inner();

        Self::from_snfloats(
            SNFloat::new(rho * f32::sin(theta)),
            SNFloat::new(rho * f32::cos(theta)),
        )
    }

    // TODO Refactor this when polar point datatype is added
    pub fn from_complex(value: SNComplex) -> Self {
        Self::from_snfloats(
            value.re(),
            value.im(),
        )
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new(Point2::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        ))
    }
}

impl Serialize for SNPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SNPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SNPointVisitor)
    }
}

struct SNPointVisitor;

impl<'de> Visitor<'de> for SNPointVisitor {
    type Value = SNPoint;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a point like '(0.0, 0.0)'")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"\(\s*(-?[\d\.]+)\s*,\s*(-?[\d\.]+)\s*\)"#).unwrap();
        }

        let caps = RE
            .captures(v)
            .ok_or_else(|| E::custom(format!("Invalid point: {}", v)))?;

        let x = f32::from_str(&caps[1]).map_err(|e| E::custom(e.to_string()))?;
        let y = f32::from_str(&caps[2]).map_err(|e| E::custom(e.to_string()))?;

        if x < -1.0 || x > 1.0 || y < -1.0 || y > 1.0 {
            return Err(E::custom(format!("SNPoint out of range: {}", v)));
        }

        Ok(SNPoint::new(Point2::new(x, y)))
    }
}

impl Display for SNPoint {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl Default for SNPoint {
    fn default() -> Self {
        Self::new(Point2::new(f32::default(), f32::default()))
    }
}

impl<'a> Generatable<'a> for SNPoint {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for SNPoint {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        _state: mutagen::State,
        _arg: MutArg<'a>,
    ) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for SNPoint {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for SNPoint {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snpoint_deserialize() {
        let a = SNPoint::new(Point2::new(-0.5, 1.0));
        let b: SNPoint = serde_yaml::from_str(&serde_yaml::to_string(&a).unwrap()).unwrap();
        assert_eq!(a, b);
    }
}
