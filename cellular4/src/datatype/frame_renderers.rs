use std::f32::consts::PI;

use ggez::{
    graphics::{Color as GgColor, DrawParam},
    timer, Context, GameResult,
};

use crate::prelude::*;

#[derive(Debug)]
pub enum FrameRenderers {
    Generalized {
        rotation: Angle,
        translation: SNPoint,
        offset_translation: SNPoint,
        offset: SNPoint,
        from_scale: SNPoint,
        to_scale: SNPoint,

        alpha: UNFloat,

        rotation_scalar: UNFloat,
        translation_scalar: UNFloat,
        offset_scalar: UNFloat,
        from_scale_scalar: UNFloat,
        to_scale_scalar: UNFloat,
    },

    /// Used as a default value for history steps before they first time they're computed to
    None,
}

impl FrameRenderers {
    pub fn draw(&self, ctx: &mut Context, history: &History, current_t: usize) -> GameResult<()> {
        match self {
            FrameRenderers::Generalized {
                rotation,
                translation,
                offset_translation,
                offset,
                from_scale,
                to_scale,

                alpha,

                rotation_scalar,
                translation_scalar,
                offset_scalar,
                from_scale_scalar,
                to_scale_scalar,
            } => {
                let lerp_sub = (timer::ticks(ctx) % CONSTS.tics_per_update) as f32
                    / CONSTS.tics_per_update as f32;

                let lerp_len = CONSTS.cell_array_lerp_length;

                for i in 0..lerp_len {
                    //let transparency = if i == 0 {1.0} else {if i == 1 {0.5} else {0.0}};
                    let back_lerp_val = (i as f32 + (1.0 - lerp_sub)) / lerp_len as f32;
                    let alpha = 1.0 - back_lerp_val;

                    let _lerp_val = (i as f32 + lerp_sub) / lerp_len as f32;

                    let history_len = history.history_steps.len();

                    let prev_history_index =
                        (current_t + i + history_len - lerp_len - 1) % history_len;
                    let history_index = (prev_history_index + 1) % history_len;

                    let prev_history_step = &history.history_steps[prev_history_index];
                    let history_step = &history.history_steps[history_index];

                    let mut alpha =
                        (1.0 - ((alpha * 2.0) - 1.0).abs()) / CONSTS.cell_array_lerp_length as f32;

                    let mut dest_x = CONSTS.initial_window_width * 0.5;
                    let mut dest_y = CONSTS.initial_window_height * 0.5;

                    let mut offset_x = 0.5;
                    let mut offset_y = 0.5;

                    let mut scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                    let mut scale_y =
                        CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

                    let mut rotation: f32 = 0.0;

                    let prev_root_scalar = prev_history_step.root_scalar;
                    let root_scalar = history_step.root_scalar;

                    let (
                        prev_rotation_scalar,
                        prev_translation_scalar,
                        prev_offset_scalar,
                        prev_from_scale_scalar,
                        prev_to_scale_scalar,
                        prev_translation,
                        prev_offset_translation,
                        prev_offset,
                    ) = if let FrameRenderers::Generalized {
                        rotation_scalar,
                        translation_scalar,
                        offset_scalar,
                        from_scale_scalar,
                        to_scale_scalar,
                        translation,
                        offset_translation,
                        offset,
                        ..
                    } = &prev_history_step.frame_renderer
                    {
                        (
                            *rotation_scalar,
                            *translation_scalar,
                            *offset_scalar,
                            *from_scale_scalar,
                            *to_scale_scalar,
                            *translation,
                            *offset_translation,
                            *offset,
                        )
                    } else {
                        Default::default()
                    };

                    if CONSTS.apply_frame_transformations {
                        let root_scalar = lerp(
                            root_scalar.into_inner(),
                            root_scalar.into_inner(),
                            back_lerp_val,
                        );

                        let rotation_scalar = lerp(
                            prev_rotation_scalar.into_inner(),
                            rotation_scalar.into_inner(),
                            back_lerp_val,
                        );

                        let translation_scalar = lerp(
                            prev_translation_scalar.into_inner(),
                            translation_scalar.into_inner(),
                            back_lerp_val,
                        );

                        let offset_scalar = lerp(
                            prev_offset_scalar.into_inner(),
                            offset_scalar.into_inner(),
                            back_lerp_val,
                        );

                        let from_scale_scalar = lerp(
                            prev_from_scale_scalar.into_inner(),
                            from_scale_scalar.into_inner(),
                            back_lerp_val,
                        );

                        let to_scale_scalar = lerp(
                            prev_to_scale_scalar.into_inner(),
                            to_scale_scalar.into_inner(),
                            back_lerp_val,
                        );

                        let translation_x = lerp(
                            prev_translation.into_inner().x,
                            translation.into_inner().x,
                            back_lerp_val,
                        ) * 0.5
                            * CONSTS.initial_window_width;

                        let translation_y = lerp(
                            prev_translation.into_inner().y,
                            translation.into_inner().y,
                            back_lerp_val,
                        ) * 0.5
                            * CONSTS.initial_window_height;

                        let offset_translation_x = lerp(
                            prev_offset.into_inner().x,
                            offset.into_inner().x,
                            back_lerp_val,
                        ) * 0.5;

                        let offset_translation_y = lerp(
                            prev_offset.into_inner().y,
                            offset.into_inner().y,
                            back_lerp_val,
                        ) * 0.5;

                        alpha *= lerp(1.0, alpha, root_scalar);
                        dest_x += translation_x * scale_x * translation_scalar * root_scalar;
                        dest_y += translation_y * scale_y * translation_scalar * root_scalar;
                        offset_x += offset_translation_x * scale_x * offset_scalar * root_scalar;
                        offset_y += offset_translation_y * scale_y * offset_scalar * root_scalar;

                        // NOTE PI is only subtracted because angles are 0..2PI currently
                        rotation += (1.0 - alpha) * (rotation - PI) * rotation_scalar * root_scalar;

                        scale_x *= lerp(
                            1.0,
                            lerp(
                                lerp(1.0, from_scale.into_inner().x, from_scale_scalar),
                                lerp(1.0, to_scale.into_inner().x, to_scale_scalar),
                                alpha,
                            ) * 2.0,
                            root_scalar,
                        );

                        scale_y *= lerp(
                            1.0,
                            lerp(
                                lerp(1.0, from_scale.into_inner().y, from_scale_scalar),
                                lerp(1.0, to_scale.into_inner().y, to_scale_scalar),
                                alpha,
                            ) * 2.0,
                            root_scalar,
                        );
                    }

                    ggez::graphics::draw(
                        ctx,
                        &history_step.computed_texture,
                        DrawParam::new()
                            .color(GgColor::new(1.0, 1.0, 1.0, alpha))
                            .dest([dest_x, dest_y])
                            .offset([offset_x, offset_y])
                            .scale([scale_x, scale_y])
                            .rotation(rotation),
                    )?;
                }
            }

            FrameRenderers::None => {}
        }

        Ok(())
    }
}

impl Default for FrameRenderers {
    fn default() -> Self {
        FrameRenderers::None
    }
}
