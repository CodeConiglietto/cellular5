use std::num::Wrapping;

use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{mutagen_args::*};

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

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: &'a mut GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for Boolean {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: &'a mut MutArg<'a>,
    ) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for Boolean {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for Boolean {
    fn update_recursively(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Nibble {
    pub value: u8,
}

impl Nibble {
    pub fn new(value: u8) -> Self {
        Self {
            value: value % Self::MAX_VALUE,
        }
    }

    pub fn into_inner(self) -> u8 {
        self.value
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value == 0 {
            Self::new(other.value)
        } else {
            Self::new(self.value / other.value)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value == 0 {
            Self::new(other.value)
        } else {
            Self::new(self.value % other.value)
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Nibble::new(rng.gen_range(0, Self::MAX_VALUE))
    }

    pub const MAX_VALUE: u8 = 16;
}

impl<'a> Generatable<'a> for Nibble {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: &'a mut GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for Nibble {
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

impl<'a> Updatable<'a> for Nibble {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for Nibble {
    fn update_recursively(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
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

    pub fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
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

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: &'a mut GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for Byte {
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

impl<'a> Updatable<'a> for Byte {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for Byte {
    fn update_recursively(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
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

    pub fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
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

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: &'a mut GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for UInt {
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

impl<'a> Updatable<'a> for UInt {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for UInt {
    fn update_recursively(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
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

    pub fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
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

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: &'a mut GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for SInt {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: &'a mut MutArg<'a>,
    ) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for SInt {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for SInt {
    fn update_recursively(&mut self, _state: mutagen::State, arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}
