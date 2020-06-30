use crate::{coordinate_set::*, datatype::{continuous::*, colors::*, discrete::*, points::*}, util::*};
use ndarray::{s, ArrayView1, Array3};

use ggez::{
    graphics::{Image as GgImage},
    Context,
};

#[derive(Debug)]
pub struct HistoryStep {
    cell_array: Array3<u8>,
    computed_texture: GgImage,

    update_coordinate: CoordinateSet,

    rotation: f32,
    translation: SNPoint,
    offset: SNPoint,
    from_scale: SNPoint,
    to_scale: SNPoint,

    root_scalar: UNFloat,
    alpha: UNFloat,
    rotation_scalar: UNFloat,
    translation_scalar: UNFloat,
    offset_scalar: UNFloat,
    from_scale_scalar: UNFloat,
    to_scale_scalar: UNFloat,
}

#[derive(Debug)]
pub struct History {
    history_steps: Vec<HistoryStep>,
}

impl History {
    fn new(ctx: &mut Context, array_width: usize, array_height: usize, size: usize) -> Self {
        Self {
            history_steps: (0..size)
                .map(|_| {
                    let cell_array = init_cell_array(array_width, array_height);
                    let computed_texture = compute_texture(ctx, cell_array.view());

                    HistoryStep {
                        cell_array: cell_array,
                        computed_texture: computed_texture,
                        update_coordinate: CoordinateSet {
                            x: SNFloat::ZERO,
                            y: SNFloat::ZERO,
                            t: 0.0,
                        },
                        rotation: 0.0,
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

    fn get_raw(&self, x: usize, y: usize, t: usize) -> ArrayView1<u8> {
        let array = &self.history_steps[t % self.history_steps.len()].cell_array;
        array.slice(s![y % array.dim().0, x % array.dim().1, ..])
    }

    fn get(&self, x: usize, y: usize, t: usize) -> ByteColor {
        let raw = self.get_raw(x, y, t);
        ByteColor {
            r: Byte::new(raw[0]),
            g: Byte::new(raw[1]),
            b: Byte::new(raw[2]),
            a: Byte::new(raw[3]),
        }
    }
}