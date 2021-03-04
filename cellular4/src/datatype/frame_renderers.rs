use std::f32::consts::PI;

use ggez::{
    graphics::{Color as GgColor, DrawParam, Image as GgImage},
    Context, GameResult,
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
    pub fresh_frame: bool, //Hack, expose this as function instead
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
    BasicFade,
    InterleavedRotate,
    DiscreteTransform,
    FadeAndChild {
        child: Box<FrameRenderers>,
        fade_color: FloatColor,
        fade_alpha_multiplier: UNFloat,
    },
    Dripping {
        invert: Boolean,
    },
    SpaceOdyssey {
        axis: Boolean,
        scale_secondary_axis: Boolean,
    },
    InfiniZoom {
        invert_direction: Boolean,
    },
    InfiniZoomRotate {
        invert_direction: Boolean,
        angle: Angle,
    },
    DiscreteRotation {
        rotation_value: Angle,
        render_single_frame: Boolean,
        invert_t_offset: Boolean,
    },
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
            FrameRenderers::BasicFade => {
                let original_alpha = 1.0 - args.back_lerp_val();
                let alpha = (1.0 - ((original_alpha * 2.0) - 1.0).abs())
                    / CONSTS.cell_array_lerp_length as f32;

                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = CONSTS.initial_window_height * 0.5;

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;
                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(
                            1.0,
                            1.0,
                            1.0,
                            (1.0 / args.history_len() as f32) * alpha,
                        ))
                        .offset([0.5, 0.5])
                        .dest([dest_x, dest_y])
                        .scale([scale_x, scale_y]),
                )?;
            }
            FrameRenderers::DiscreteTransform => {
                //TODO FIX ME
                if args.fresh_frame {
                    let dest_x = CONSTS.initial_window_width * 0.5;
                    let dest_y = CONSTS.initial_window_height * 0.5;

                    let scalar = 1.0 - ((args.lerp_i) as f32 / args.lerp_len() as f32);

                    let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                    let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;
                    ggez::graphics::draw(
                        args.ctx,
                        &args.history_step().computed_texture,
                        DrawParam::new()
                            .color(GgColor::new(1.0, 1.0, 1.0, 1.0 / args.history_len() as f32))
                            .offset([0.5, 0.5])
                            .dest([dest_x, dest_y])
                            .scale([scale_x * scalar, scale_y * scalar]),
                    )?;
                }
            }
            FrameRenderers::InterleavedRotate => {
                let original_alpha = 1.0 - args.back_lerp_val();
                let alpha = (1.0 - ((original_alpha * 2.0) - 1.0).abs())
                    / CONSTS.cell_array_lerp_length as f32;

                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = CONSTS.initial_window_height * 0.5;

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;
                //TODO fix
                let invert = (args
                    .history_step()
                    .update_coordinate
                    .get_byte_t()
                    .into_inner()
                    % 2)
                    == 0;
                let inversion_scalar = if invert { -1.0 } else { 1.0 };
                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(
                            1.0,
                            1.0,
                            1.0,
                            (1.0 / args.history_len() as f32) * alpha,
                        ))
                        .offset([0.5, 0.5])
                        .dest([dest_x, dest_y])
                        .scale([scale_x * inversion_scalar, scale_y])
                        .rotation(
                            PI * original_alpha
                                * args.history_step().root_scalar.into_inner()
                                * inversion_scalar,
                        ),
                )?;
            }
            FrameRenderers::FadeAndChild {
                child,
                fade_color,
                fade_alpha_multiplier,
            } => {
                if args.lerp_i == 0 {
                    let (prev_fade_color, prev_fade_alpha_multiplier) =
                        if let FrameRenderers::FadeAndChild {
                            fade_color,
                            fade_alpha_multiplier,
                            ..
                        } = args.prev_history_step().frame_renderer
                        {
                            (fade_color, fade_alpha_multiplier)
                        } else {
                            Default::default()
                        };

                    let mut modified_color = fade_color
                        .clone()
                        .lerp(prev_fade_color, UNFloat::new(args.lerp_sub));
                    modified_color.a = UNFloat::new(
                        lerp(
                            modified_color.a.into_inner() * fade_alpha_multiplier.into_inner(),
                            prev_fade_color.a.into_inner()
                                * prev_fade_alpha_multiplier.into_inner(),
                            args.lerp_sub,
                        ) / args.lerp_len() as f32,
                    );

                    ggez::graphics::draw(
                        args.ctx,
                        args.blank_texture,
                        DrawParam::new().color(modified_color.into()).scale([
                            CONSTS.initial_window_width as f32,
                            CONSTS.initial_window_height as f32,
                        ]),
                    )?;
                }
                child.draw(args).unwrap();
            }
            FrameRenderers::Dripping { invert } => {
                let original_alpha = 1.0 - args.back_lerp_val();

                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = if invert.into_inner() {
                    CONSTS.initial_window_height
                } else {
                    0.0
                };

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

                let offset_y = if invert.into_inner() { 1.0 } else { 0.0 };

                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(
                            1.0,
                            1.0,
                            1.0,
                            (1.0 / args.history_len() as f32) * (1.0 - original_alpha),
                        ))
                        .offset([0.5, offset_y])
                        .dest([dest_x, dest_y])
                        .scale([scale_x, scale_y * original_alpha]),
                )?;
            }
            FrameRenderers::SpaceOdyssey {
                axis,
                scale_secondary_axis,
            } => {
                let original_alpha = 1.0 - args.back_lerp_val();

                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = CONSTS.initial_window_height * 0.5;

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

                let x_scalar;
                let y_scalar;

                if axis.into_inner() {
                    x_scalar = original_alpha;
                    y_scalar = 1.0
                        + if scale_secondary_axis.into_inner() {
                            original_alpha
                        } else {
                            0.0
                        };
                } else {
                    x_scalar = 1.0
                        + if scale_secondary_axis.into_inner() {
                            original_alpha
                        } else {
                            0.0
                        };
                    y_scalar = original_alpha;
                };

                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(
                            1.0,
                            1.0,
                            1.0,
                            (1.0 / args.history_len() as f32) * (1.0 - original_alpha),
                        ))
                        .offset([0.5, 0.5])
                        .dest([dest_x, dest_y])
                        .scale([scale_x * x_scalar, scale_y * y_scalar]),
                )?;
            }
            FrameRenderers::InfiniZoom { invert_direction } => {
                let original_alpha = 1.0 - args.back_lerp_val();
                let alpha = (1.0 - ((original_alpha * 2.0) - 1.0).abs())
                    / CONSTS.cell_array_lerp_length as f32;

                let scalar = if invert_direction.into_inner() {
                    lerp(
                        1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0),
                        1.0,
                        args.history_step().root_scalar.into_inner(),
                    )
                } else {
                    lerp(
                        1.0,
                        1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0),
                        args.history_step().root_scalar.into_inner(),
                    )
                };
                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = CONSTS.initial_window_height * 0.5;

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;
                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(
                            1.0,
                            1.0,
                            1.0,
                            (1.0 / args.history_len() as f32) * alpha,
                        ))
                        .offset([0.5, 0.5])
                        .dest([dest_x, dest_y])
                        .scale([scalar * scale_x, scalar * scale_y]),
                )?;
            }
            FrameRenderers::InfiniZoomRotate {
                invert_direction,
                angle,
            } => {
                let original_alpha = 1.0 - args.back_lerp_val();
                let alpha = (1.0 - ((original_alpha * 2.0) - 1.0).abs())
                    / CONSTS.cell_array_lerp_length as f32;
                //TODO fix
                let scalar = if invert_direction.into_inner() {
                    lerp(
                        1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0),
                        1.0,
                        args.history_step().root_scalar.into_inner(),
                    )
                } else {
                    lerp(
                        1.0,
                        1.0 / (args.lerp_i as f32 + (1.0 - args.lerp_sub)).max(1.0),
                        args.history_step().root_scalar.into_inner(),
                    )
                };
                let dest_x = CONSTS.initial_window_width * 0.5;
                let dest_y = CONSTS.initial_window_height * 0.5;

                let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;
                ggez::graphics::draw(
                    args.ctx,
                    &args.history_step().computed_texture,
                    DrawParam::new()
                        .color(GgColor::new(
                            1.0,
                            1.0,
                            1.0,
                            (1.0 / args.history_len() as f32) * alpha,
                        ))
                        .offset([0.5, 0.5])
                        .dest([dest_x, dest_y])
                        .scale([scalar * scale_x * 1.5, scalar * scale_y * 1.5])
                        .rotation(
                            angle.into_inner()
                                * scalar
                                * args.history_step().root_scalar.into_inner(),
                        ),
                )?;
            }
            FrameRenderers::DiscreteRotation {
                rotation_value,
                render_single_frame,
                invert_t_offset,
            } => {
                if args.fresh_frame && (!render_single_frame.into_inner() || args.lerp_i == 0) {
                    //TODO fix
                    let dest_x = CONSTS.initial_window_width * 0.5;
                    let dest_y = CONSTS.initial_window_height * 0.5;

                    let scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                    let scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

                    let t_offset = if invert_t_offset.into_inner() {
                        args.lerp_len() - args.lerp_i
                    } else {
                        args.lerp_i
                    };

                    let angle =
                        (rotation_value.into_inner()) * (args.current_t + (t_offset)) as f32;

                    ggez::graphics::draw(
                        args.ctx,
                        &args.history_step().computed_texture,
                        DrawParam::new()
                            .color(GgColor::new(1.0, 1.0, 1.0, 1.0 / args.history_len() as f32))
                            .offset([0.5, 0.5])
                            .dest([dest_x, dest_y])
                            .scale([scale_x, scale_y])
                            .rotation(angle),
                    )?;
                }
            }
            FrameRenderers::Generalized {
                // rotation,
                translation,
                // offset_translation,
                offset,
                from_scale,
                to_scale,

                // alpha,
                rotation_scalar,
                translation_scalar,
                offset_scalar,
                from_scale_scalar,
                to_scale_scalar,
                ..
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

                let rotation: f32 = 0.0;

                let _prev_root_scalar = args.prev_history_step().root_scalar;
                let root_scalar = args.history_step().root_scalar;

                let (
                    prev_rotation_scalar,
                    prev_translation_scalar,
                    prev_offset_scalar,
                    prev_from_scale_scalar,
                    prev_to_scale_scalar,
                    prev_translation,
                    _prev_offset_translation,
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
                } = args.prev_history_step().frame_renderer
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

                    let _rotation_scalar = lerp(
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
                    // DOUBLE NOTE This is probably not necessary any more after fixing the angle ranges
                    // rotation += (1.0 - alpha) * (rotation - PI) * rotation_scalar * root_scalar;

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
