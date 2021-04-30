use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use ndarray::prelude::*;
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

#[derive(Debug, Clone, Copy, Generatable, Serialize, Deserialize)]
#[mutagen(gen_arg = type GenArg<'a>)]
pub enum PixelNeighbourhood {
    Melt,
    BigMelt,
    VonNeumann,
    AntiVonNeumann,
    Cross,
    Moore,
    Spiral,
    Diamond,
    Circle,
    Flower,
}

impl PixelNeighbourhood {
    pub fn offsets(&self) -> &'static [(isize, isize)] {
        match self {
            PixelNeighbourhood::Melt => &[(-1, -1), (0, -1), (1, -1)],
            PixelNeighbourhood::BigMelt => {
                &[(-1, -1), (0, -1), (1, -1), (-1, -2), (0, -2), (1, -2)]
            }
            PixelNeighbourhood::VonNeumann => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
            PixelNeighbourhood::AntiVonNeumann => &[(-1, -1), (1, -1), (1, -1), (1, 1)],
            PixelNeighbourhood::Cross => &[
                (-1, 0),
                (-2, 0),
                (1, 0),
                (2, 0),
                (0, -1),
                (0, -2),
                (0, 1),
                (0, 2),
            ],
            PixelNeighbourhood::Moore => &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            PixelNeighbourhood::Spiral => &[
                //TODO: Double check when not tired
                (-1, 0),
                (-2, 1),
                (1, 0),
                (2, 1),
                (0, -1),
                (1, -2),
                (0, 1),
                (1, 2),
            ],
            PixelNeighbourhood::Diamond => &[
                //TODO: Double check when not tired
                (-1, -1),
                (-2, 0),
                (-1, 1),
                (2, 0),
                (1, -1),
                (0, -2),
                (1, 1),
                (0, 2),
            ],
            PixelNeighbourhood::Circle => &[
                //TODO: Double check when not tired
                (-2, -1),
                (-2, 0),
                (-2, 1),
                (2, -1),
                (2, 0),
                (2, 1),
                (-1, -2),
                (0, -2),
                (1, -2),
                (-1, 2),
                (0, 2),
                (1, 2),
            ],
            PixelNeighbourhood::Flower => &[
                //TODO: Double check when not tired
                (-2, -1),
                (-1, 0),
                (-2, 1),
                (2, -1),
                (1, 0),
                (2, 1),
                (-1, -2),
                (0, -1),
                (1, -2),
                (-1, 2),
                (0, 1),
                (1, 2),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighbourCountAutomataRule {
    pub neighbourhood: PixelNeighbourhood,
    pub truth_table: Array3<BitColor>,
}

impl<'a> Generatable<'a> for NeighbourCountAutomataRule {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, mut arg: Self::GenArg) -> Self {
        let neighbourhood = PixelNeighbourhood::generate_rng(rng, arg.reborrow());
        let n = neighbourhood.offsets().len() + 1;

        Self {
            neighbourhood,
            truth_table: Array3::from_shape_fn((n, n, n), move |_| {
                BitColor::generate_rng(rng, arg.reborrow())
            }),
        }
    }
}

impl<'a> Mutatable<'a> for NeighbourCountAutomataRule {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        // *self = Self::generate_rng(rng, arg.into());
        let n = self.neighbourhood.offsets().len() + 1;
        let index_r = thread_rng().gen::<usize>() % n;
        let index_g = thread_rng().gen::<usize>() % n;
        let index_b = thread_rng().gen::<usize>() % n;

        self.truth_table[[index_r, index_g, index_b]] = BitColor::generate_rng(rng, arg.into());
    }
}

impl<'a> Updatable<'a> for NeighbourCountAutomataRule {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for NeighbourCountAutomataRule {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeLikeAutomataRule {
    pub neighbourhood: PixelNeighbourhood,
    pub color_order: [BitColor; 8],
    /// Indexed by (neighbour_count, color_idx)
    pub truth_table: Vec<[LifeLikeTable; 8]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Generatable, Mutatable)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub struct LifeLikeTable {
    pub birth: Boolean,
    pub survival: Boolean,
}

impl<'a> Generatable<'a> for LifeLikeAutomataRule {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, mut arg: Self::GenArg) -> Self {
        let neighbourhood = PixelNeighbourhood::generate_rng(rng, arg.reborrow());
        let n = neighbourhood.offsets().len() + 1;
        let mut color_order = BitColor::values();
        color_order.shuffle(rng);

        Self {
            neighbourhood,
            color_order,
            truth_table: (0..n)
                .map(|_| {
                    [
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                        LifeLikeTable::generate_rng(rng, arg.reborrow()),
                    ]
                })
                .collect(),
        }
    }
}

impl<'a> Mutatable<'a> for LifeLikeAutomataRule {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        // *self = Self::generate_rng(rng, arg.into());
        let n = self.neighbourhood.offsets().len() + 1;
        let i = thread_rng().gen::<usize>() % n;
        let j = thread_rng().gen::<usize>() % 8;

        self.truth_table[i][j].mutate_rng(rng, arg);
    }
}

impl<'a> Updatable<'a> for LifeLikeAutomataRule {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for LifeLikeAutomataRule {
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
