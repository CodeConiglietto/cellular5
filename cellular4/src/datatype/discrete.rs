use std::num::Wrapping;

use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::updatestate::UpdateState;

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
}

impl<'a> Mutagen<'a> for Boolean {
    type Arg = UpdateState<'a>;
}

impl<'a> Generatable<'a> for Boolean {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) -> Self {
        Boolean { value: rng.gen() }
    }
}

impl<'a> Mutatable<'a> for Boolean {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for Boolean {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for Boolean {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
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

    pub const MAX_VALUE: u8 = 16;
}

impl<'a> Mutagen<'a> for Nibble {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for Nibble {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) -> Self {
        Nibble::new(rng.gen_range(0, Self::MAX_VALUE))
    }
}

impl<'a> Mutatable<'a> for Nibble {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for Nibble {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for Nibble {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
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
}

impl<'a> Mutagen<'a> for Byte {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for Byte {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) -> Self {
        Byte { value: rng.gen() }
    }
}

impl<'a> Mutatable<'a> for Byte {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for Byte {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for Byte {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
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
}

impl<'a> Mutagen<'a> for UInt {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for UInt {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) -> Self {
        UInt { value: rng.gen() }
    }
}

impl<'a> Mutatable<'a> for UInt {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for UInt {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for UInt {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
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
}

impl<'a> Mutagen<'a> for SInt {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for SInt {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) -> Self {
        SInt { value: rng.gen() }
    }
}

impl<'a> Mutatable<'a> for SInt {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for SInt {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for SInt {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}
