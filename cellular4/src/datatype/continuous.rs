use std::{
    cmp::Ordering,
    f32::consts::PI,
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct UNFloat {
    value: f32,
}

impl UNFloat {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        assert!(
            value >= 0.0 && value <= 1.0,
            "Invalid UNFloat value: {}",
            value
        );
        Self::new_unchecked(value)
    }

    pub fn new_clamped(value: f32) -> Self {
        Self::new_unchecked(value.max(0.0).min(1.0))
    }

    pub fn new_random_clamped(value: f32) -> Self {
        if value < 0.0 || value > 1.0
        {
            Self::random(&mut rand::thread_rng())
        } else{
            Self::new_unchecked(value)
        }
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (0.0, 1.0)))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn new_sawtooth(value: f32) -> Self {
        Self::new(value.fract() - value.signum().min(0.0))
    }

    pub fn new_triangle(value: f32) -> Self {
        let scaled_value = (value - 1.0) / 2.0;
        Self::new((scaled_value.fract() - scaled_value.signum().min(0.0) - 0.5).abs() * 2.0)
    }

    pub fn sawtooth_add(self, other: Self) -> Self {
        self.sawtooth_add_f32(other.into_inner())
    }

    pub fn sawtooth_add_f32(self, other: f32) -> Self {
        Self::new_sawtooth(self.into_inner() + other)
    }

    pub fn triangle_add(self, other: Self) -> Self {
        self.triangle_add_f32(other.into_inner())
    }

    pub fn triangle_add_f32(self, other: f32) -> Self {
        Self::new_triangle(self.into_inner() + other)
    }

    pub fn to_angle(self) -> Angle {
        Angle::new_from_range(self.value, 0.0, 1.0)
    }

    pub fn to_signed(self) -> SNFloat {
        SNFloat::new_from_range(self.value, 0.0, 1.0)
    }

    pub fn subdivide_sawtooth(self, divisor: Nibble) -> UNFloat {
        let total = self.into_inner() * divisor.into_inner() as f32;
        UNFloat::new_sawtooth(total)
    }

    pub fn subdivide_triangle(self, divisor: Nibble) -> UNFloat {
        let total = self.into_inner() * divisor.into_inner() as f32;
        UNFloat::new_triangle(total)
    }

    pub fn multiply(self, other: UNFloat) -> UNFloat {
        UNFloat::new(self.into_inner() * other.into_inner())
    }

    pub const ZERO: Self = Self { value: 0.0 };
    pub const ONE: Self = Self { value: 1.0 };

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new_unchecked(rng.gen_range(0.0, 1.0))
    }
}

impl<'a> Generatable<'a> for UNFloat {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for UNFloat {
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

impl<'a> Updatable<'a> for UNFloat {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for UNFloat {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct SNFloat {
    value: f32,
}

impl SNFloat {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        assert!(
            value >= -1.0 && value <= 1.0,
            "Invalid SNFloat value: {}",
            value
        );

        Self::new_unchecked(value)
    }

    pub fn new_clamped(value: f32) -> Self {
        Self::new_unchecked(value.max(-1.0).min(1.0))
    }

    pub fn new_random_clamped(value: f32) -> Self {
        if value < -1.0 || value > 1.0
        {
            Self::random(&mut rand::thread_rng())
        } else{
            Self::new_unchecked(value)
        }
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (-1.0, 1.0)))
    }

    pub fn new_sawtooth(value: f32) -> Self {
        let scaled_value = (value + 1.0) / 2.0;
        Self::new((scaled_value.fract() - scaled_value.signum().min(0.0)) * 2.0 - 1.0)
    }

    pub fn new_triangle(value: f32) -> Self {
        let scaled_value = (value - 1.0) / 4.0;
        Self::new((scaled_value.fract() - scaled_value.signum().min(0.0) - 0.5).abs() * 4.0 - 1.0)
    }

    pub fn new_fractional(value: f32) -> Self {
        Self::new(value.fract())
    }

    pub fn new_tanh(value: f32) -> Self {
        Self::new(value.tanh())
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_angle(self) -> Angle {
        Angle::new_from_range(self.value, -1.0, 1.0)
    }

    pub fn to_unsigned(self) -> UNFloat {
        UNFloat::new_from_range(self.value, -1.0, 1.0)
    }

    pub fn normalised_add(self, other: Self, normaliser: SFloatNormaliser) -> Self {
        normaliser.normalise(self.into_inner() + other.into_inner())
    }

    // pub fn sawtooth_add(self, other: Self) -> Self {
    //     self.sawtooth_add_f32(other.into_inner())
    // }

    // pub fn sawtooth_add_f32(self, other: f32) -> Self {
    //     Self::new_sawtooth(self.into_inner() + other)
    // }

    // pub fn triangle_add(self, other: Self) -> Self {
    //     self.triangle_add_f32(other.into_inner())
    // }

    // pub fn triangle_add_f32(self, other: f32) -> Self {
    //     Self::new_triangle(self.into_inner() + other)
    // }

    pub fn subdivide(self, divisor: Nibble) -> SNFloat {
        let total = self.into_inner() * divisor.into_inner() as f32;
        let sign = total.signum();
        SNFloat::new((total.abs() - total.abs().floor()) * sign)
    }

    pub fn multiply(self, other: SNFloat) -> Self {
        Self::new(self.into_inner() * other.into_inner())
    }

    pub fn multiply_unfloat(self, other: UNFloat) -> Self {
        Self::new(self.into_inner() * other.into_inner())
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new_unchecked(rng.gen_range(-1.0, 1.0))
    }

    pub const ZERO: Self = Self { value: 0.0 };
    pub const ONE: Self = Self { value: 1.0 };
    pub const NEG_ONE: Self = Self { value: -1.0 };
}

impl Display for SNFloat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.4}", self.into_inner())
    }
}

impl<'a> Generatable<'a> for SNFloat {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for SNFloat {
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

impl<'a> Updatable<'a> for SNFloat {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for SNFloat {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct Angle {
    value: f32,
}

impl Angle {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        let normalised = match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => (value / (2.0 * PI)).fract() * (2.0 * PI),
            Ordering::Less => (value / (2.0 * PI)).fract() * (2.0 * PI) + (2.0 * PI),
            Ordering::Equal => value,
        };

        assert!(
            normalised >= 0.0 && normalised <= 2.0 * PI,
            "Failed to normalize angle: {} -> {}",
            value,
            normalised,
        );

        Self::new_unchecked(normalised)
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (0.0, 2.0 * PI)))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_signed(self) -> SNFloat {
        SNFloat::new_from_range(self.value, 0.0, 2.0 * PI)
    }

    pub fn to_unsigned(self) -> UNFloat {
        UNFloat::new_from_range(self.value, 0.0, 2.0 * PI)
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new_unchecked(rng.gen_range(0.0, 2.0 * PI))
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Self::new(self.into_inner() + rhs.into_inner())
    }
}

impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        *self = *self + rhs;
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Self::Output {
        Self::new(self.into_inner() - rhs.into_inner())
    }
}

impl SubAssign<Angle> for Angle {
    fn sub_assign(&mut self, rhs: Angle) {
        *self = *self - rhs;
    }
}

impl<'a> Generatable<'a> for Angle {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for Angle {
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

impl<'a> Updatable<'a> for Angle {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for Angle {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angles() {
        for i in 0..100_000 {
            Angle::new(i as f32);
        }
    }
}
