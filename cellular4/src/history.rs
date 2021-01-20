use ggez::{graphics::Image as GgImage, Context};
use ndarray::{s, Array3, ArrayView1};

use crate::prelude::*;

#[derive(Debug)]
pub struct HistoryStep {
    pub cell_array: Array3<u8>,
    pub computed_texture: GgImage,

    pub update_coordinate: CoordinateSet,
    pub frame_renderer: FrameRenderers,
    pub root_scalar: UNFloat,
    pub fade_color: FloatColor,
    pub alpha_multiplier: UNFloat,
}

impl HistoryStep {
    pub fn new(
        ctx: &mut Context,
        array_width: usize,
        array_height: usize,
        use_nearest_neighbour_scaling: bool,
    ) -> Self {
        let cell_array = init_cell_array(array_width, array_height);

        Self {
            computed_texture: compute_texture(
                ctx,
                cell_array.view(),
                use_nearest_neighbour_scaling,
            ),
            cell_array,
            update_coordinate: CoordinateSet {
                x: SNFloat::ZERO,
                y: SNFloat::ZERO,
                t: 0.0,
            },
            frame_renderer: FrameRenderers::default(),
            root_scalar: UNFloat::ZERO,
            fade_color: FloatColor::ALL_ZERO,
            alpha_multiplier: UNFloat::ZERO,
        }
    }
}

#[derive(Debug)]
pub struct History {
    pub history_steps: Vec<HistoryStep>,
}

impl History {
    pub fn new(ctx: &mut Context, array_width: usize, array_height: usize, size: usize) -> Self {
        Self {
            history_steps: (0..size)
                .map(|_| HistoryStep::new(ctx, array_width, array_height, false))
                .collect(),
        }
    }

    pub fn get_raw(&self, x: usize, y: usize, t: usize) -> ArrayView1<u8> {
        let array = &self.history_steps[t % self.history_steps.len()].cell_array;
        array.slice(s![y % array.dim().0, x % array.dim().1, ..])
    }

    pub fn get_normalised(&self, pos: SNPoint, t: usize) -> FloatColor {
        self.get(
            ((pos.x().into_inner() + 1.0) * 0.5 * CONSTS.cell_array_width as f32) as usize,
            ((pos.y().into_inner() + 1.0) * 0.5 * CONSTS.cell_array_height as f32) as usize,
            t as usize,
        )
        .into()
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
