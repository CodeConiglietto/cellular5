use crate::colors::*;
use crate::constants::*;
use crate::{mutagen_args::*,get_random_color};
use ndarray::Array2;
use rand::prelude::*;

pub trait Reseed {
    fn reseed(&self, cell_array: &mut Array2<BitColor>) {
        let cell_array_width = cell_array.dim().0;
        let cell_array_height = cell_array.dim().1;

        for x in 0..cell_array_width {
            for y in 0..cell_array_height {
                cell_array[[x, y]] = self.reseed_cell(x, y);
            }
        }
    }

    fn mutate(&mut self);
    fn reseed_cell(&self, x: usize, y: usize) -> BitColor;
}

pub enum Reseeder {
    Modulus {
        x_mod: usize,
        y_mod: usize,
        x_offset: usize,
        y_offset: usize,
        color_table: Array2<BitColor>,
    },
}

impl Reseed for Reseeder {
    fn reseed_cell(&self, x: usize, y: usize) -> BitColor {
        match self {
            Reseeder::Modulus {
                x_mod,
                y_mod,
                x_offset,
                y_offset,
                color_table,
            } => {
                let x_index = if (x + x_offset) % x_mod == 0 { 1 } else { 0 };
                let y_index = if (y + y_offset) % y_mod == 0 { 1 } else { 0 };

                color_table[[x_index, y_index]]
            }
        }
    }

    fn mutate(&mut self) {
        match self {
            Reseeder::Modulus {
                x_mod,
                y_mod,
                x_offset,
                y_offset,
                color_table,
            } => {
                let min_cell_array_dim = CELL_ARRAY_WIDTH.min(CELL_ARRAY_HEIGHT);

                if random::<bool>() {
                    *x_mod = (random::<usize>() % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    *x_mod = ((*x_mod + 1) % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    *x_offset = (random::<usize>() % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    *x_offset = ((*x_offset + 1) % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    *y_mod = (random::<usize>() % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    *y_mod = ((*y_mod + 1) % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    *y_offset = (random::<usize>() % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    *y_offset = ((*y_offset + 1) % min_cell_array_dim) + 1;
                }

                if random::<bool>() {
                    color_table[[random::<usize>() % 2, random::<usize>() % 2]] =
                        get_random_color();
                }
            }
        }
    }
}
