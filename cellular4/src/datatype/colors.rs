use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use palette::rgb::Rgb;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{continuous::*, discrete::*},
    updatestate::UpdateState,
};

#[derive(Generatable, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NibbleColor {
    pub r: Nibble,
    pub g: Nibble,
    pub b: Nibble,
    pub a: Nibble,
}

impl<'a> Mutagen<'a> for NibbleColor {
    type Arg = UpdateState<'a>;
}

impl<'a> Updatable<'a> for NibbleColor {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for NibbleColor {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl From<FloatColor> for NibbleColor {
    fn from(other: FloatColor) -> Self {
        Self {
            r: Nibble::new((other.r.into_inner() * 16.0) as u8),
            g: Nibble::new((other.g.into_inner() * 16.0) as u8),
            b: Nibble::new((other.b.into_inner() * 16.0) as u8),
            a: Nibble::new((other.a.into_inner() * 16.0) as u8),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl<'a> Updatable<'a> for ByteColor {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for ByteColor {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> Mutagen<'a> for ByteColor {
    type Arg = UpdateState<'a>;
}

impl<'a> Generatable<'a> for ByteColor {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) -> Self {
        Self {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
            a: rng.gen(),
        }
    }
}

impl<'a> Mutatable<'a> for ByteColor {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl From<image::Rgba<u8>> for ByteColor {
    fn from(c: image::Rgba<u8>) -> Self {
        Self {
            r: c.0[0],
            g: c.0[1],
            b: c.0[2],
            a: c.0[3],
        }
    }
}

impl From<FloatColor> for ByteColor {
    fn from(other: FloatColor) -> Self {
        Self {
            r: (other.r.into_inner() * 255.0) as u8,
            g: (other.g.into_inner() * 255.0) as u8,
            b: (other.b.into_inner() * 255.0) as u8,
            a: (other.a.into_inner() * 255.0) as u8,
        }
    }
}

pub fn float_color_from_pallette_rgb(rgb: Rgb, alpha: f32) -> FloatColor {
    FloatColor {
        r: UNFloat::new(rgb.red as f32),
        g: UNFloat::new(rgb.green as f32),
        b: UNFloat::new(rgb.blue as f32),
        a: UNFloat::new(alpha),
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BitColor {
    Black,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,
}

impl BitColor {
    pub fn get_color(self) -> ByteColor {
        match self {
            BitColor::Black => ByteColor {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            BitColor::Red => ByteColor {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
            BitColor::Green => ByteColor {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            },
            BitColor::Blue => ByteColor {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
            },
            BitColor::Cyan => ByteColor {
                r: 0,
                g: 255,
                b: 255,
                a: 255,
            },
            BitColor::Magenta => ByteColor {
                r: 255,
                g: 0,
                b: 255,
                a: 255,
            },
            BitColor::Yellow => ByteColor {
                r: 255,
                g: 255,
                b: 0,
                a: 255,
            },
            BitColor::White => ByteColor {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        }
    }

    pub fn from_float_color(c: FloatColor) -> BitColor {
        BitColor::from_components([
            c.r.into_inner() >= 0.5,
            c.g.into_inner() >= 0.5,
            c.b.into_inner() >= 0.5,
        ])
    }

    pub fn from_byte_color(c: ByteColor) -> BitColor {
        BitColor::from_components([c.r >= 127, c.g >= 127, c.b >= 127])
    }

    pub fn to_index(self) -> usize {
        match self {
            BitColor::Black => 0,
            BitColor::Red => 1,
            BitColor::Green => 2,
            BitColor::Blue => 3,
            BitColor::Cyan => 4,
            BitColor::Magenta => 5,
            BitColor::Yellow => 6,
            BitColor::White => 7,
        }
    }

    pub fn from_index(index: usize) -> BitColor {
        match index {
            0 => BitColor::Black,
            1 => BitColor::Red,
            2 => BitColor::Green,
            3 => BitColor::Blue,
            4 => BitColor::Cyan,
            5 => BitColor::Magenta,
            6 => BitColor::Yellow,
            7 => BitColor::White,
            _ => {
                dbg!(index);
                panic!()
            }
        }
    }

    pub fn to_components(self) -> [bool; 3] {
        match self {
            BitColor::Black => [false, false, false],
            BitColor::Red => [true, false, false],
            BitColor::Green => [false, true, false],
            BitColor::Blue => [false, false, true],
            BitColor::Cyan => [false, true, true],
            BitColor::Magenta => [true, false, true],
            BitColor::Yellow => [true, true, false],
            BitColor::White => [true, true, true],
        }
    }

    pub fn from_components(components: [bool; 3]) -> BitColor {
        match components {
            [false, false, false] => BitColor::Black,
            [true, false, false] => BitColor::Red,
            [false, true, false] => BitColor::Green,
            [false, false, true] => BitColor::Blue,
            [false, true, true] => BitColor::Cyan,
            [true, false, true] => BitColor::Magenta,
            [true, true, false] => BitColor::Yellow,
            [true, true, true] => BitColor::White,
        }
    }

    pub fn has_color(self, other: BitColor) -> bool {
        let mut has_color = false;
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            has_color = has_color || (current_color[i] && other_color[i]);
        }

        has_color
    }

    pub fn give_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] || other_color[i];
        }

        new_color
    }

    pub fn take_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] && !other_color[i];
        }

        new_color
    }

    pub fn xor_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] =
                (current_color[i] || other_color[i]) && !(current_color[i] && other_color[i]);
        }

        new_color
    }

    pub fn eq_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] == other_color[i];
        }

        new_color
    }
}

impl<'a> Mutagen<'a> for BitColor {
    type Arg = UpdateState<'a>;
}
impl<'a> Generatable<'a> for BitColor {
    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) -> Self {
        Self::from_components([rng.gen(), rng.gen(), rng.gen()])
    }
}

impl<'a> Mutatable<'a> for BitColor {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        _state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        let current_color = self.to_components();
        let mut new_color = [rng.gen(), rng.gen(), rng.gen()];

        for i in 0..3 {
            if rng.gen::<bool>() {
                new_color[i] = current_color[i];
            }
        }

        *self = Self::from_components(new_color);
    }
}

impl<'a> Updatable<'a> for BitColor {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for BitColor {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl From<ByteColor> for BitColor {
    fn from(other: ByteColor) -> Self {
        Self::from_components([other.r > 127, other.g > 127, other.b > 127])
    }
}

#[derive(Generatable, Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct FloatColor {
    pub r: UNFloat,
    pub g: UNFloat,
    pub b: UNFloat,
    pub a: UNFloat,
}

impl FloatColor {
    pub fn get_average(&self) -> f32 {
        (self.r.into_inner() + self.b.into_inner() + self.g.into_inner()) / 3.0
    }

    //Translated to rust from an answer here here: https://stackoverflow.com/questions/23090019/fastest-formula-to-get-hue-from-rgb
    pub fn get_hue_unfloat(&self) -> UNFloat {
        let r = self.r.into_inner();
        let g = self.g.into_inner();
        let b = self.b.into_inner();

        let min = r.min(g.min(b));
        let max = r.min(g.min(b));

        if min == max {
            UNFloat::new(0.0)
        } else {
            let mut hue = if max == r {
                (g - b) / (max - min)
            } else if max == g {
                2.0 + (b - r) / (max - min)
            } else {
                4.0 + (r - g) / (max - min)
            };

            hue = hue * 60.0;

            if hue < 0.0 {
                hue += 360.0;
            }

            UNFloat::new(hue / 360.0)
        }
    }
}

impl From<ByteColor> for FloatColor {
    fn from(c: ByteColor) -> FloatColor {
        FloatColor {
            r: UNFloat::new(c.r as f32 / 255.0),
            g: UNFloat::new(c.g as f32 / 255.0),
            b: UNFloat::new(c.b as f32 / 255.0),
            a: UNFloat::new(c.a as f32 / 255.0),
        }
    }
}

impl From<BitColor> for FloatColor {
    fn from(c: BitColor) -> FloatColor {
        let color_components = c.to_components();

        FloatColor {
            r: UNFloat::new_unchecked(if color_components[0] { 1.0 } else { 0.0 }),
            g: UNFloat::new_unchecked(if color_components[1] { 1.0 } else { 0.0 }),
            b: UNFloat::new_unchecked(if color_components[2] { 1.0 } else { 0.0 }),
            a: UNFloat::ONE,
        }
    }
}

impl<'a> Mutagen<'a> for FloatColor {
    type Arg = UpdateState<'a>;
}

impl<'a> Mutatable<'a> for FloatColor {
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        state: mutagen::State,
        arg: UpdateState<'a>,
    ) {
        *self = Self::generate_rng(rng, state, arg);
    }
}

impl<'a> Updatable<'a> for FloatColor {
    fn update(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for FloatColor {
    fn update_recursively(&mut self, _state: mutagen::State, arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}
