use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum FrameRendererNodes {
    // #[mutagen(gen_weight = leaf_node_weight)]
    #[mutagen(gen_weight = 20.0)]
    BasicFade,
    #[mutagen(gen_weight = leaf_node_weight)]
    InterleavedRotate,
    #[mutagen(gen_weight = leaf_node_weight)]
    DiscreteTransform,
    #[mutagen(gen_weight = leaf_node_weight)]
    DiscreteRotation {
        rotation_value: Angle,
        render_single_frame: Boolean,
        invert_t_offset: Boolean,
    },
    #[mutagen(gen_weight = leaf_node_weight)]
    Dripping { invert: Boolean },
    #[mutagen(gen_weight = leaf_node_weight)]
    SpaceOdyssey {
        axis: Boolean,
        scale_secondary_axis: Boolean,
    },
    #[mutagen(gen_weight = leaf_node_weight)]
    InfiniZoom { invert_direction: Boolean },
    #[mutagen(gen_weight = pipe_node_weight)]
    InfiniZoomRotate {
        invert_direction: Boolean,
        child_angle: NodeBox<AngleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Generalized {
        rotation_node: NodeBox<AngleNodes>,
        translation_node: NodeBox<SNPointNodes>,
        offset_node: NodeBox<SNPointNodes>,
        offset_translation_node: NodeBox<SNPointNodes>,
        from_scale_node: NodeBox<SNPointNodes>,
        to_scale_node: NodeBox<SNPointNodes>,

        alpha_node: NodeBox<UNFloatNodes>,

        rotation_scalar_node: NodeBox<UNFloatNodes>,
        translation_scalar_node: NodeBox<UNFloatNodes>,
        offset_scalar_node: NodeBox<UNFloatNodes>,
        from_scale_scalar_node: NodeBox<UNFloatNodes>,
        to_scale_scalar_node: NodeBox<UNFloatNodes>,
    },
    // #[mutagen(gen_weight = branch_node_weight)]
    //TODO: redo this so it doesn't kill people with epilepsy
    #[mutagen(gen_weight = 0.0)]
    FadeAndChild {
        child_renderer: NodeBox<FrameRendererNodes>,
        child_color: NodeBox<FloatColorNodes>,
        child_alpha_multiplier: NodeBox<UNFloatNodes>,
    },

    // TODO Remove when we have a proper leaf nodes
    #[mutagen(gen_weight = leaf_node_weight)]
    None,
}

impl Node for FrameRendererNodes {
    type Output = FrameRenderers;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        match self {
            FrameRendererNodes::BasicFade => FrameRenderers::BasicFade,
            FrameRendererNodes::InterleavedRotate => FrameRenderers::InterleavedRotate,
            FrameRendererNodes::DiscreteTransform => FrameRenderers::DiscreteTransform,
            FrameRendererNodes::DiscreteRotation {
                rotation_value,
                render_single_frame,
                invert_t_offset,
            } => FrameRenderers::DiscreteRotation {
                rotation_value: *rotation_value,
                render_single_frame: *render_single_frame,
                invert_t_offset: *invert_t_offset,
            },
            FrameRendererNodes::Dripping { invert } => FrameRenderers::Dripping { invert: *invert },
            FrameRendererNodes::SpaceOdyssey {
                axis,
                scale_secondary_axis,
            } => FrameRenderers::SpaceOdyssey {
                axis: *axis,
                scale_secondary_axis: *scale_secondary_axis,
            },
            FrameRendererNodes::InfiniZoom { invert_direction } => FrameRenderers::InfiniZoom {
                invert_direction: *invert_direction,
            },
            FrameRendererNodes::InfiniZoomRotate {
                invert_direction,
                child_angle,
            } => FrameRenderers::InfiniZoomRotate {
                invert_direction: *invert_direction,
                angle: child_angle.compute(compute_arg),
            },
            FrameRendererNodes::Generalized {
                rotation_node,
                translation_node,
                offset_translation_node,
                offset_node,
                from_scale_node,
                to_scale_node,

                alpha_node,

                rotation_scalar_node,
                translation_scalar_node,
                offset_scalar_node,
                from_scale_scalar_node,
                to_scale_scalar_node,
            } => FrameRenderers::Generalized {
                rotation: rotation_node.compute(compute_arg.reborrow()),
                translation: translation_node.compute(compute_arg.reborrow()),
                offset: offset_node.compute(compute_arg.reborrow()),
                offset_translation: offset_translation_node.compute(compute_arg.reborrow()),
                from_scale: from_scale_node.compute(compute_arg.reborrow()),
                to_scale: to_scale_node.compute(compute_arg.reborrow()),

                alpha: alpha_node.compute(compute_arg.reborrow()),

                rotation_scalar: rotation_scalar_node.compute(compute_arg.reborrow()),
                translation_scalar: translation_scalar_node.compute(compute_arg.reborrow()),
                offset_scalar: offset_scalar_node.compute(compute_arg.reborrow()),
                from_scale_scalar: from_scale_scalar_node.compute(compute_arg.reborrow()),
                to_scale_scalar: to_scale_scalar_node.compute(compute_arg.reborrow()),
            },
            FrameRendererNodes::FadeAndChild {
                child_renderer,
                child_color,
                child_alpha_multiplier,
            } => FrameRenderers::FadeAndChild {
                child: Box::new(child_renderer.compute(compute_arg.reborrow())),
                fade_color: child_color.compute(compute_arg.reborrow()),
                fade_alpha_multiplier: child_alpha_multiplier.compute(compute_arg.reborrow()),
            },

            FrameRendererNodes::None => FrameRenderers::None,
        }
    }
}

impl<'a> Updatable<'a> for FrameRendererNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}
