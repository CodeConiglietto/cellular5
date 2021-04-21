use std::num::Wrapping;

use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::mutagen_args::*;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Default)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn into_inner(self) -> bool {
        self.value
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self { value: rng.gen() }
    }
}

impl<'a> Generatable<'a> for Boolean {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for Boolean {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        match rng.gen_range(0..2) {
            0 => *self = Self::random(rng),
            1 => *self = Self::new(!self.into_inner()),
            _ => unreachable!(),
        }
    }
}

impl<'a> Updatable<'a> for Boolean {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for Boolean {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Nibble {
    pub value: u8,
}

impl Nibble {
    pub fn new(value: u8) -> Self {
        assert!(value < Self::MODULUS);
        Self::new_unchecked(value)
    }

    pub fn new_circular(value: u8) -> Self {
        Self::new_unchecked(value % Self::MODULUS)
    }

    pub fn new_unchecked(value: u8) -> Self {
        Self { value }
    }

    pub fn into_inner(self) -> u8 {
        self.value
    }

    pub fn circular_add(self, other: Self) -> Self {
        Self::new_circular(self.value + other.value)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value == 0 {
            other
        } else {
            Self::new_unchecked(self.value / other.value)
        }
    }

    pub fn circular_multiply(self, other: Self) -> Self {
        Self::new_circular(self.value * other.value)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value == 0 {
            other
        } else {
            Self::new_circular(self.value % other.value)
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Nibble::new_unchecked(rng.gen_range(0..Self::MODULUS))
    }

    pub const MODULUS: u8 = 16;
}

impl<'a> Generatable<'a> for Nibble {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for Nibble {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        match rng.gen_range(0..3) {
            0 => *self = Self::new(self.into_inner().saturating_add(1) % 16),
            1 => *self = Self::new(self.into_inner().saturating_sub(1) % 16), //TODO: This won't wrap equally in both directiosn. Fix pls
            2 => *self = Self::random(rng),
            _ => unreachable!(),
        }
    }
}

impl<'a> Updatable<'a> for Nibble {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for Nibble {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Byte {
    pub value: Wrapping<u8>,
}

impl Byte {
    pub fn new(value: u8) -> Self {
        Self {
            value: Wrapping(value),
        }
    }

    pub fn into_inner(self) -> u8 {
        self.value.0
    }

    pub fn circular_add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn invert_wrapped(self) -> Self {
        Self::new((Wrapping(255u8) - self.value).0)
    }

    pub fn circular_multiply(self, other: Self) -> Self {
        Self::new((self.value * other.value).0)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value % other.value).0)
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self { value: rng.gen() }
    }
}

impl<'a> Generatable<'a> for Byte {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for Byte {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        match rng.gen_range(0..4) {
            0 => *self = Self::new(self.into_inner().wrapping_add(1)),
            1 => *self = Self::new(self.into_inner().wrapping_sub(1)),
            2 => *self = Self::new(self.into_inner().saturating_add(1)),
            3 => *self = Self::new(self.into_inner().saturating_sub(1)),
            4 => *self = Self::random(rng),
            _ => unreachable!(),
        }
    }
}

impl<'a> Updatable<'a> for Byte {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for Byte {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct UInt {
    pub value: Wrapping<u32>,
}

impl UInt {
    pub fn new(value: u32) -> Self {
        Self {
            value: Wrapping(value),
        }
    }

    pub fn into_inner(self) -> u32 {
        self.value.0
    }

    pub fn circular_add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn circular_multiply(self, other: Self) -> Self {
        Self::new((self.value * other.value).0)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value % other.value).0)
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self { value: rng.gen() }
    }
}

impl<'a> Generatable<'a> for UInt {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for UInt {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for UInt {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for UInt {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct SInt {
    pub value: Wrapping<i32>,
}

impl SInt {
    pub fn new(value: i32) -> Self {
        Self {
            value: Wrapping(value),
        }
    }

    pub fn into_inner(self) -> i32 {
        self.value.0
    }

    pub fn circular_add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn circular_multiply(self, other: Self) -> Self {
        Self::new((self.value * other.value).0)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value % other.value).0)
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self { value: rng.gen() }
    }
}

impl<'a> Generatable<'a> for SInt {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for SInt {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for SInt {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for SInt {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}
