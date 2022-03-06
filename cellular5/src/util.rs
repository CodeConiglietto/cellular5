use ggez::{
    graphics::{Drawable, Image as GgImage},
    Context,
};

use crate::prelude::*;

use ndarray::prelude::*;
use rand::prelude::*;

pub fn compute_texture(
    ctx: &mut Context,
    cell_array: ArrayView3<u8>,
    use_nearest_neighbour: bool,
) -> GgImage {
    let (height, width, _) = cell_array.dim();
    let mut image = GgImage::from_rgba8(
        ctx,
        width as u16,
        height as u16,
        cell_array.as_slice().unwrap(),
    )
    .unwrap();

    //TODO: figure out if there's some way we can abuse blend modes for novel behaviour
    //Perhaps we make this a node type that interleaves different blend types so it doesn't white/black out the screen
    if false {
        match thread_rng().gen::<u8>() % 8 {
            0 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Add));
            }
            1 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Alpha));
            }
            2 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Darken));
            }
            3 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Invert));
            }
            4 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Lighten));
            }
            5 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Multiply));
            }
            6 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Replace));
            }
            7 => {
                image.set_blend_mode(Some(ggez::graphics::BlendMode::Subtract));
            }
            _ => panic!(),
        }
    }

    if use_nearest_neighbour {
        image.set_filter(ggez::graphics::FilterMode::Nearest);
    }

    image
}

pub fn compute_blank_texture(ctx: &mut Context) -> GgImage {
    let mut image = GgImage::from_rgba8(ctx, 1, 1, &[255, 255, 255, 255]).unwrap();

    image.set_filter(ggez::graphics::FilterMode::Linear);
    //image.set_filter(ggez::graphics::FilterMode::Nearest);

    image
}