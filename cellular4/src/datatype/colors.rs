use std::f32::consts::PI;

use approx::abs_diff_eq;
use ggez::graphics::Color as GgColor;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, Mutatable, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub struct NibbleColor {
    pub r: Nibble,
    pub g: Nibble,
    pub b: Nibble,
    pub a: Nibble,
}

impl<'a> Updatable<'a> for NibbleColor {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for NibbleColor {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
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

#[derive(Generatable, Mutatable, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub struct ByteColor {
    pub r: Byte,
    pub g: Byte,
    pub b: Byte,
    pub a: Byte,
}

impl<'a> Updatable<'a> for ByteColor {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for ByteColor {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}

impl From<image::Rgba<u8>> for ByteColor {
    fn from(c: image::Rgba<u8>) -> Self {
        Self {
            r: Byte::new(c.0[0]),
            g: Byte::new(c.0[1]),
            b: Byte::new(c.0[2]),
            a: Byte::new(c.0[3]),
        }
    }
}

impl From<FloatColor> for ByteColor {
    fn from(other: FloatColor) -> Self {
        Self {
            r: Byte::new((other.r.into_inner() * 255.0) as u8),
            g: Byte::new((other.g.into_inner() * 255.0) as u8),
            b: Byte::new((other.b.into_inner() * 255.0) as u8),
            a: Byte::new((other.a.into_inner() * 255.0) as u8),
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

/// Expects all inputs and outputs to be between 0.0 and 1.0
pub fn rgb_tuple_to_hsv_tuple(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let (h, s, v) = Hsv::<Srgb, _>::from(Rgb::<Srgb, _>::new(r, g, b)).into_components();
    (h.to_positive_radians() / (2.0 * PI), s, v)
}

/// Expects all inputs and outputs to be between 0.0 and 1.0
pub fn hsv_tuple_to_rgb_tuple(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    Rgb::<Srgb, _>::from(Hsv::<Srgb, _>::new(
        RgbHue::from_radians(h * 2.0 * PI),
        s,
        v,
    ))
    .into_components()
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
                r: Byte::new(0),
                g: Byte::new(0),
                b: Byte::new(0),
                a: Byte::new(255),
            },
            BitColor::Red => ByteColor {
                r: Byte::new(255),
                g: Byte::new(0),
                b: Byte::new(0),
                a: Byte::new(255),
            },
            BitColor::Green => ByteColor {
                r: Byte::new(0),
                g: Byte::new(255),
                b: Byte::new(0),
                a: Byte::new(255),
            },
            BitColor::Blue => ByteColor {
                r: Byte::new(0),
                g: Byte::new(0),
                b: Byte::new(255),
                a: Byte::new(255),
            },
            BitColor::Cyan => ByteColor {
                r: Byte::new(0),
                g: Byte::new(255),
                b: Byte::new(255),
                a: Byte::new(255),
            },
            BitColor::Magenta => ByteColor {
                r: Byte::new(255),
                g: Byte::new(0),
                b: Byte::new(255),
                a: Byte::new(255),
            },
            BitColor::Yellow => ByteColor {
                r: Byte::new(255),
                g: Byte::new(255),
                b: Byte::new(0),
                a: Byte::new(255),
            },
            BitColor::White => ByteColor {
                r: Byte::new(255),
                g: Byte::new(255),
                b: Byte::new(255),
                a: Byte::new(255),
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
        BitColor::from_components([
            c.r.into_inner() >= 127,
            c.g.into_inner() >= 127,
            c.b.into_inner() >= 127,
        ])
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
                ldbg!(index);
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

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::from_components([rng.gen(), rng.gen(), rng.gen()])
    }
}

impl<'a> Generatable<'a> for BitColor {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for BitColor {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        let mut components = self.to_components();

        for component in components.iter_mut() {
            if rng.gen::<bool>() {
                *component = rng.gen();
            }
        }

        *self = Self::from_components(components);
    }
}

impl<'a> Updatable<'a> for BitColor {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for BitColor {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}

impl From<ByteColor> for BitColor {
    fn from(other: ByteColor) -> Self {
        Self::from_components([
            other.r.into_inner() > 127,
            other.g.into_inner() > 127,
            other.b.into_inner() > 127,
        ])
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq)]
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

        if abs_diff_eq!(min, max) {
            UNFloat::new(0.0)
        } else {
            let mut hue = if abs_diff_eq!(max, r) {
                (g - b) / (max - min)
            } else if abs_diff_eq!(max, g) {
                2.0 + (b - r) / (max - min)
            } else {
                4.0 + (r - g) / (max - min)
            };

            hue *= 60.0;

            if hue < 0.0 {
                hue += 360.0;
            }

            UNFloat::new(hue / 360.0)
        }
    }

    pub fn get_saturation_unfloat(&self) -> UNFloat {
        UNFloat::new(rgb_tuple_to_hsv_tuple(self.r.into_inner(), self.g.into_inner(), self.b.into_inner()).1)
    }

    pub fn get_value_unfloat(&self) -> UNFloat {
        UNFloat::new(rgb_tuple_to_hsv_tuple(self.r.into_inner(), self.g.into_inner(), self.b.into_inner()).2)
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            r: UNFloat::random(rng),
            g: UNFloat::random(rng),
            b: UNFloat::random(rng),
            a: UNFloat::random(rng),
        }
    }

    pub fn lerp(self, other: Self, scalar: UNFloat) -> Self {
        Self {
            r: self.r.lerp(other.r, scalar),
            g: self.g.lerp(other.g, scalar),
            b: self.b.lerp(other.b, scalar),
            a: self.a.lerp(other.a, scalar),
        }
    }

    pub const ALL_ZERO: Self = Self {
        r: UNFloat::ZERO,
        g: UNFloat::ZERO,
        b: UNFloat::ZERO,
        a: UNFloat::ZERO,
    };
    pub const WHITE: Self = Self {
        r: UNFloat::ONE,
        g: UNFloat::ONE,
        b: UNFloat::ONE,
        a: UNFloat::ONE,
    };
    pub const BLACK: Self = Self {
        r: UNFloat::ZERO,
        g: UNFloat::ZERO,
        b: UNFloat::ZERO,
        a: UNFloat::ONE,
    };
}

impl Into<GgColor> for FloatColor {
    fn into(self) -> GgColor {
        GgColor {
            r: self.r.into_inner(),
            g: self.g.into_inner(),
            b: self.b.into_inner(),
            a: self.a.into_inner(),
        }
    }
}

impl From<ByteColor> for FloatColor {
    fn from(c: ByteColor) -> FloatColor {
        FloatColor {
            r: UNFloat::new(c.r.into_inner() as f32 / 255.0),
            g: UNFloat::new(c.g.into_inner() as f32 / 255.0),
            b: UNFloat::new(c.b.into_inner() as f32 / 255.0),
            a: UNFloat::new(c.a.into_inner() as f32 / 255.0),
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

impl<'a> Generatable<'a> for FloatColor {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for FloatColor {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for FloatColor {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for FloatColor {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}
