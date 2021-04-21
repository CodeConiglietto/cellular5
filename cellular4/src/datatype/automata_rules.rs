use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementaryAutomataRule {
    pub pattern: [Boolean; 8],
}

impl ElementaryAutomataRule {
    pub fn get_index_from_booleans(l: Boolean, c: Boolean, r: Boolean) -> u8 {
        let mut result = 0;

        if r.into_inner() {
            result |= 1;
        }

        if c.into_inner() {
            result |= 2;
        }

        if l.into_inner() {
            result |= 4;
        }

        return result;
    }

    pub fn get_value_from_booleans(&self, l: Boolean, c: Boolean, r: Boolean) -> Boolean {
        self.pattern[usize::from(Self::get_index_from_booleans(l, c, r))]
    }

    pub fn from_wolfram_code(code: u8) -> Self {
        Self {
            pattern: [
                Boolean::new((code & (1 << 0)) > 0),
                Boolean::new((code & (1 << 1)) > 0),
                Boolean::new((code & (1 << 2)) > 0),
                Boolean::new((code & (1 << 3)) > 0),
                Boolean::new((code & (1 << 4)) > 0),
                Boolean::new((code & (1 << 5)) > 0),
                Boolean::new((code & (1 << 6)) > 0),
                Boolean::new((code & (1 << 7)) > 0),
            ],
        }
    }
}

impl<'a> Generatable<'a> for ElementaryAutomataRule {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, mut arg: Self::GenArg) -> Self {
        Self {
            //noice
            pattern: [
                Boolean::generate_rng(rng, arg.reborrow()),
                Boolean::generate_rng(rng, arg.reborrow()),
                Boolean::generate_rng(rng, arg.reborrow()),
                Boolean::generate_rng(rng, arg.reborrow()),
                Boolean::generate_rng(rng, arg.reborrow()),
                Boolean::generate_rng(rng, arg.reborrow()),
                Boolean::generate_rng(rng, arg.reborrow()),
                Boolean::generate_rng(rng, arg.reborrow()),
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
            self.pattern[index] = Boolean::new(!self.pattern[index].into_inner());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_110() {
        let rule = ElementaryAutomataRule::from_wolfram_code(110);

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(true),
                Boolean::new(true),
                Boolean::new(true),
            )
            .into_inner(),
            false,
        );

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(true),
                Boolean::new(true),
                Boolean::new(false),
            )
            .into_inner(),
            true,
        );

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(true),
                Boolean::new(false),
                Boolean::new(true),
            )
            .into_inner(),
            true,
        );

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(true),
                Boolean::new(false),
                Boolean::new(false),
            )
            .into_inner(),
            false,
        );

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(false),
                Boolean::new(true),
                Boolean::new(true),
            )
            .into_inner(),
            true,
        );

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(false),
                Boolean::new(true),
                Boolean::new(false),
            )
            .into_inner(),
            true,
        );

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(false),
                Boolean::new(false),
                Boolean::new(true),
            )
            .into_inner(),
            true,
        );

        assert_eq!(
            rule.get_value_from_booleans(
                Boolean::new(false),
                Boolean::new(false),
                Boolean::new(false),
            )
            .into_inner(),
            false,
        );
    }
}
