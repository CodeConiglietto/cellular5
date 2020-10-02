use std::f32::consts::PI;

use ggez::{
    graphics::{Color as GgColor, DrawParam, Image as GgImage},
    timer, Context, GameResult,
};

use crate::prelude::*;

#[derive(Debug)]
pub struct RenderArgs<'a> {
    pub ctx: &'a mut Context,
    pub history: &'a History,
    pub blank_texture: &'a GgImage,
    pub current_t: usize,
    pub lerp_sub: f32,
    pub lerp_i: usize,
}

impl<'a> RenderArgs<'a> {
    pub fn lerp_len(&self) -> usize {
        CONSTS.cell_array_lerp_length
    }

    pub fn back_lerp_val(&self) -> f32 {
        (self.lerp_i as f32 + (1.0 - self.lerp_sub)) / self.lerp_len() as f32
    }

    pub fn lerp_val(&self) -> f32 {
        (self.lerp_i as f32 + self.lerp_sub) / self.lerp_len() as f32
    }

    pub fn prev_history_index(&self) -> usize {
        (self.current_t + self.lerp_i + self.history_len() - self.lerp_len() - 1)
            % self.history_len()
    }

    pub fn history_index(&self) -> usize {
        (self.prev_history_index() + 1) % self.history_len()
    }

    pub fn prev_history_step(&self) -> &'a HistoryStep {
        &self.history.history_steps[self.prev_history_index()]
    }

    pub fn history_step(&self) -> &'a HistoryStep {
        &self.history.history_steps[self.history_index()]
    }

    pub fn history_len(&self) -> usize {
        self.history.history_steps.len()
    }
}

#[derive(Debug)]
pub enum FrameRenderers {
    FadeAndChild {child: Box<FrameRenderers>, fade_color: FloatColor, fade_alpha_multiplier: UNFloat},
    InfiniZoom {invert_direction: Boolean},
    InfiniZoomRotate {invert_direction: Boolean, angle: Angle},
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
    pub fn draw(&self, args: RenderArgs) -> GameResult<()> {
        match self {
            FrameRenderers::FadeAndChild {child, fade_color, fade_alpha_multiplier} => {
                if args.lerp_i == 0
                {
                    let mut modified_color = fade_color.clone();
                    modified_color.a = 
                    UNFloat::new(modified_color.a.into_inner() * fade_alpha_multiplier.into_inner() * 0.5);
                    
                    ggez::graphics::draw(
                        args.ctx,
                        args.blank_texture,
                        DrawParam::new()
                            .color(modified_color.into())
                            .scale([CONSTS.initial_window_width as f32, CONSTS.initial_window_height as f32]),
                    )?;
                }
                child.draw(args);
            },
            FrameRenderers::InfiniZoom { invert_direction } => {
                let alpha = 1.0 - args.back_lerp_val();
                let mut alpha =
                    (1.0 - ((alpha * 2.0) - 1.0).abs()) / CONSTS.cell_array_lerp_length as f32;

                let mut scalar = 1.0;
                // let mut scalar = 
                //     lerp(1.0, 
                //         1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0), 
                //         args.history_step().root_scalar.into_inner());
                if invert_direction.into_inner() 
                {
                    // scalar = 1.0 - scalar
                    scalar = 
                    lerp(1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0),
                        1.0,
                        args.history_step().root_scalar.into_inner());
                } else {
                    scalar = 
                        lerp(1.0, 
                            1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0), 
                            args.history_step().root_scalar.into_inner());
                }
                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = CONSTS.initial_window_height * 0.5;

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;
                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(1.0, 1.0, 1.0, (1.0 / args.history_len() as f32) * alpha))
                        .offset([0.5, 0.5])
                        .dest([dest_x, dest_y])
                        .scale([scalar * scale_x, scalar * scale_y]),
                )?;
            },
            FrameRenderers::InfiniZoomRotate { invert_direction, angle } => {
                let alpha = 1.0 - args.back_lerp_val();
                let mut alpha =
                    (1.0 - ((alpha * 2.0) - 1.0).abs()) / CONSTS.cell_array_lerp_length as f32;
                //TODO fix
                let mut scalar = 1.0;
                // let mut scalar = 1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0);
                // let mut scalar = lerp(1.0, 1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0), args.history_step().root_scalar.into_inner());
                // if invert_direction.into_inner() {scalar = 1.0 - scalar}
                if invert_direction.into_inner() 
                {
                    // scalar = 1.0 - scalar
                    scalar = 
                    lerp(1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0),
                        1.0,
                        args.history_step().root_scalar.into_inner());
                } else {
                    scalar = 
                        lerp(1.0, 
                            1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0), 
                            args.history_step().root_scalar.into_inner());
                }
                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = CONSTS.initial_window_height * 0.5;

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;
                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(1.0, 1.0, 1.0, (1.0 / args.history_len() as f32) * alpha))
                        .offset([0.5, 0.5])
                        .dest([dest_x, dest_y])
                        .scale([scalar * scale_x * 1.5, scalar * scale_y * 1.5])
                        .rotation(angle.into_inner() * scalar * args.history_step().root_scalar.into_inner()),
                )?;
            },
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
                let alpha = 1.0 - args.back_lerp_val();

                let mut alpha =
                    (1.0 - ((alpha * 2.0) - 1.0).abs()) / CONSTS.cell_array_lerp_length as f32;

                let mut dest_x = CONSTS.initial_window_width * 0.5;
                let mut dest_y = CONSTS.initial_window_height * 0.5;

                let mut offset_x = 0.5;
                let mut offset_y = 0.5;

                let mut scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let mut scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

                let mut rotation: f32 = 0.0;

                let prev_root_scalar = args.prev_history_step().root_scalar;
                let root_scalar = args.history_step().root_scalar;

                let (
                    prev_rotation_scalar,
                    prev_translation_scalar,
                    prev_offset_scalar,
                    prev_from_scale_scalar,
                    prev_to_scale_scalar,
                    prev_translation,
                    prev_offset_translation,
                    prev_offset,
                ) = if let &FrameRenderers::Generalized {
                    rotation_scalar,
                    translation_scalar,
                    offset_scalar,
                    from_scale_scalar,
                    to_scale_scalar,
                    translation,
                    offset_translation,
                    offset,
                    ..
                } = &args.prev_history_step().frame_renderer
                {
                    (
                        rotation_scalar,
                        translation_scalar,
                        offset_scalar,
                        from_scale_scalar,
                        to_scale_scalar,
                        translation,
                        offset_translation,
                        offset,
                    )
                } else {
                    Default::default()
                };

                if CONSTS.apply_frame_transformations {
                    let root_scalar = lerp(
                        root_scalar.into_inner(),
                        root_scalar.into_inner(),
                        args.back_lerp_val(),
                    );

                    let rotation_scalar = lerp(
                        prev_rotation_scalar.into_inner(),
                        rotation_scalar.into_inner(),
                        args.back_lerp_val(),
                    );

                    let translation_scalar = lerp(
                        prev_translation_scalar.into_inner(),
                        translation_scalar.into_inner(),
                        args.back_lerp_val(),
                    );

                    let offset_scalar = lerp(
                        prev_offset_scalar.into_inner(),
                        offset_scalar.into_inner(),
                        args.back_lerp_val(),
                    );

                    let from_scale_scalar = lerp(
                        prev_from_scale_scalar.into_inner(),
                        from_scale_scalar.into_inner(),
                        args.back_lerp_val(),
                    );

                    let to_scale_scalar = lerp(
                        prev_to_scale_scalar.into_inner(),
                        to_scale_scalar.into_inner(),
                        args.back_lerp_val(),
                    );

                    let translation_x = lerp(
                        prev_translation.into_inner().x,
                        translation.into_inner().x,
                        args.back_lerp_val(),
                    ) * 0.5
                        * CONSTS.initial_window_width;

                    let translation_y = lerp(
                        prev_translation.into_inner().y,
                        translation.into_inner().y,
                        args.back_lerp_val(),
                    ) * 0.5
                        * CONSTS.initial_window_height;

                    let offset_translation_x = lerp(
                        prev_offset.into_inner().x,
                        offset.into_inner().x,
                        args.back_lerp_val(),
                    ) * 0.5;

                    let offset_translation_y = lerp(
                        prev_offset.into_inner().y,
                        offset.into_inner().y,
                        args.back_lerp_val(),
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
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(1.0, 1.0, 1.0, alpha))
                        .dest([dest_x, dest_y])
                        .offset([offset_x, offset_y])
                        .scale([scale_x, scale_y])
                        .rotation(rotation),
                )?;
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
