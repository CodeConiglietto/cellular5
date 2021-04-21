use crate::mutagen_args::*; //check if in prelude
use crate::prelude::*;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use ndarray::prelude::*;
use rand::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementaryAutomataRule {
    pub pattern: [bool; 8],
}

impl ElementaryAutomataRule {
    pub fn get_index_from_booleans(a: bool, b: bool, c: bool) -> u8 {
        let mut result = 0;

        if a {
            result += 1;
        }
        if b {
            result += 2;
        }
        if c {
            result += 4;
        }

        return result;
    }
}

impl<'a> Generatable<'a> for ElementaryAutomataRule {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R, _arg: Self::GenArg) -> Self {
        Self {
            //noice
            pattern: [
                thread_rng().gen::<bool>(),
                thread_rng().gen::<bool>(),
                thread_rng().gen::<bool>(),
                thread_rng().gen::<bool>(),
                thread_rng().gen::<bool>(),
                thread_rng().gen::<bool>(),
                thread_rng().gen::<bool>(),
                thread_rng().gen::<bool>(),
            ],
        }
    }
}

impl<'a> Mutatable<'a> for ElementaryAutomataRule {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        if thread_rng().gen::<bool>() {
            *self = Self::generate_rng(rng, arg.into());
        } else {
            let index = thread_rng().gen::<usize>() % 8;
            self.pattern[index] = !&self.pattern[index];
        }
    }
}

impl<'a> Updatable<'a> for ElementaryAutomataRule {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for ElementaryAutomataRule {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}