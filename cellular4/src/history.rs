use crate::{
    coordinate_set::*,
    datatype::{colors::*, continuous::*, discrete::*, points::*},
    util::*,
};
use ndarray::{s, Array3, ArrayView1};

use ggez::{graphics::Image as GgImage, Context};

#[derive(Debug)]
pub struct HistoryStep {
    pub cell_array: Array3<u8>,
    pub computed_texture: GgImage,

    pub update_coordinate: CoordinateSet,

    pub rotation: Angle,
    pub translation: SNPoint,
    pub offset: SNPoint,
    pub from_scale: SNPoint,
    pub to_scale: SNPoint,

    pub root_scalar: UNFloat,
    pub alpha: UNFloat,
    pub rotation_scalar: UNFloat,
    pub translation_scalar: UNFloat,
    pub offset_scalar: UNFloat,
    pub from_scale_scalar: UNFloat,
    pub to_scale_scalar: UNFloat,
}

#[derive(Debug)]
pub struct History {
    pub history_steps: Vec<HistoryStep>,
}

impl History {
    pub fn new(ctx: &mut Context, array_width: usize, array_height: usize, size: usize) -> Self {
        Self {
            history_steps: (0..size)
                .map(|_| {
                    let cell_array = init_cell_array(array_width, array_height);

                    HistoryStep {
                        computed_texture: compute_texture(ctx, cell_array.view()),
                        cell_array,
                        update_coordinate: CoordinateSet {
                            x: SNFloat::ZERO,
                            y: SNFloat::ZERO,
                            t: 0.0,
                        },
                        rotation: Angle::ZERO,
                        translation: SNPoint::zero(),
                        offset: SNPoint::zero(),
                        from_scale: SNPoint::zero(),
                        to_scale: SNPoint::zero(),

                        root_scalar: UNFloat::default(),
                        alpha: UNFloat::default(),
                        rotation_scalar: UNFloat::default(),
                        translation_scalar: UNFloat::default(),
                        offset_scalar: UNFloat::default(),
                        from_scale_scalar: UNFloat::default(),
                        to_scale_scalar: UNFloat::default(),
                    }
                })
                .collect(),
        }
    }

    pub fn get_raw(&self, x: usize, y: usize, t: usize) -> ArrayView1<u8> {
        let array = &self.history_steps[t % self.history_steps.len()].cell_array;
        array.slice(s![y % array.dim().0, x % array.dim().1, ..])
    }

    pub fn get(&self, x: usize, y: usize, t: usize) -> ByteColor {
        let raw = self.get_raw(x, y, t);
        ByteColor {
            r: Byte::new(raw[0]),
            g: Byte::new(raw[1]),
            b: Byte::new(raw[2]),
            a: Byte::new(raw[3]),
        }
    }
}
