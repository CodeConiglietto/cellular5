use std::{
    f32::consts::PI,
    fmt::{self, Display, Formatter},
};

use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{datatype::discrete::*, updatestate::UpdateState, util::*};

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
}

impl<'a> Mutagen<'a> for UNFloat {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for UNFloat {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: UpdateState<'a>,
    ) -> Self {
        Self::new_unchecked(rng.gen_range(0.0, 1.0))
    }
}

impl<'a> Mutatable<'a> for UNFloat {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for UNFloat {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for UNFloat {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
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

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_angle(self) -> Angle {
        Angle::new_from_range(self.value, -1.0, 1.0)
    }

    pub fn to_unsigned(self) -> UNFloat {
        UNFloat::new_from_range(self.value, -1.0, 1.0)
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

    pub const ZERO: Self = Self { value: 0.0 };
    pub const ONE: Self = Self { value: 1.0 };
    pub const NEG_ONE: Self = Self { value: -1.0 };
}

impl Display for SNFloat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.4}", self.into_inner())
    }
}

impl<'a> Mutagen<'a> for SNFloat {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for SNFloat {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: UpdateState<'a>,
    ) -> Self {
        Self::new_unchecked(rng.gen_range(-1.0, 1.0))
    }
}

impl<'a> Mutatable<'a> for SNFloat {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for SNFloat {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for SNFloat {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
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
        let normalised = value - 2.0 * PI * (value / (2.0 * PI)).floor();

        debug_assert!(
            normalised >= 0.0 && normalised < 2.0 * PI,
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
}

impl<'a> Mutagen<'a> for Angle {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for Angle {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: UpdateState<'a>,
    ) -> Self {
        Angle::new_unchecked(rng.gen_range(0.0, 2.0 * PI))
    }
}

impl<'a> Mutatable<'a> for Angle {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for Angle {
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for Angle {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
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
