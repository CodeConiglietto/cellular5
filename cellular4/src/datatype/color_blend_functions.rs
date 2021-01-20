use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{colors::*, continuous::*, discrete::*},
    mutagen_args::*,
};

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, Generatable, Mutatable, UpdatableRecursively,
)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum ColorBlendFunctions {
    Dissolve,
    Overlay,
    ScreenDodge,
}

impl ColorBlendFunctions {
    pub fn blend(self, a: FloatColor, b: FloatColor) -> FloatColor {
        match self {
            Self::Dissolve => {
                if Boolean::random(&mut rand::thread_rng()).into_inner() {
                    a
                } else {
                    b
                }
            }
            Self::Overlay => {
                let ar = a.r.into_inner();
                let ag = a.g.into_inner();
                let ab = a.b.into_inner();

                let br = b.r.into_inner();
                let bg = b.g.into_inner();
                let bb = b.b.into_inner();

                FloatColor {
                    r: UNFloat::new(if ar < 0.5 {
                        (2.0 * ar * br).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - ar) * (1.0 - br)))
                    }),
                    g: UNFloat::new(if ag < 0.5 {
                        (2.0 * ag * bg).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - ag) * (1.0 - bg)))
                    }),
                    b: UNFloat::new(if ab < 0.5 {
                        (2.0 * ab * bb).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - ab) * (1.0 - bb)))
                    }),
                    a: UNFloat::new((a.a.into_inner() + b.a.into_inner()) * 0.5),
                }
            }
            Self::ScreenDodge => {
                let ar = a.r.into_inner();
                let ag = a.g.into_inner();
                let ab = a.b.into_inner();

                let br = b.r.into_inner();
                let bg = b.g.into_inner();
                let bb = b.b.into_inner();

                FloatColor {
                    r: UNFloat::new(1.0 - ((1.0 - ar) * (1.0 - br))),
                    g: UNFloat::new(1.0 - ((1.0 - ag) * (1.0 - bg))),
                    b: UNFloat::new(1.0 - ((1.0 - ab) * (1.0 - bb))),
                    a: UNFloat::new((a.a.into_inner() + b.a.into_inner()) * 0.5),
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for ColorBlendFunctions {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: Self::UpdateArg) {}
}
