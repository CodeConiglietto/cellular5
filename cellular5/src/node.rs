pub mod automata_nodes;
pub mod color_blend_nodes;
pub mod color_nodes;
pub mod complex_nodes;
pub mod constraint_resolver_nodes;
pub mod continuous_nodes;
pub mod coord_map_nodes;
pub mod discrete_nodes;
pub mod frame_renderer_nodes;
pub mod iterative_function_nodes;
pub mod matrix_nodes;
pub mod point_nodes;
pub mod point_set_nodes;

use crate::prelude::*;

pub trait Node {
    type Output;

    fn compute(&self, compute_arg: ComArg) -> Self::Output;
}

pub fn max_node_depth() -> usize {
    CONSTS
        .max_branch_depth
        .max(CONSTS.max_pipe_depth.max(CONSTS.max_leaf_depth))
}

pub mod mutagen_functions {
    use super::*;

    pub fn leaf_node_weight<T: MutagenArg>(arg: T) -> f64 {
        debug_assert!(arg.depth() <= max_node_depth());

        if arg.depth() < CONSTS.min_leaf_depth || arg.depth() > CONSTS.max_leaf_depth {
            0.0
        } else {
            map_range(
                arg.depth() as f32,
                (CONSTS.min_leaf_depth as f32, CONSTS.max_leaf_depth as f32),
                (0.0, 1.0),
            ) as f64
        }
    }

    pub fn pipe_node_weight<T: MutagenArg>(arg: T) -> f64 {
        debug_assert!(arg.depth() <= max_node_depth());

        if arg.depth() < CONSTS.min_pipe_depth || arg.depth() > CONSTS.max_pipe_depth {
            0.0
        } else {
            1.0 - map_range(
                arg.depth() as f32,
                (CONSTS.min_pipe_depth as f32, CONSTS.max_pipe_depth as f32),
                (0.0, 1.0),
            ) as f64
        }
    }

    pub fn branch_node_weight<T: MutagenArg>(arg: T) -> f64 {
        debug_assert!(arg.depth() <= max_node_depth());

        if arg.depth() < CONSTS.min_branch_depth || arg.depth() > CONSTS.max_branch_depth {
            0.0
        } else {
            1.0 - map_range(
                arg.depth() as f32,
                (
                    CONSTS.min_branch_depth as f32,
                    CONSTS.max_branch_depth as f32,
                ),
                (0.0, 1.0),
            ) as f64
        }
    }

    pub fn gamepad_node_weight<T: MutagenArg>(arg: T) -> f64 {
        if arg.gamepads().gamepads.is_empty() {
            0.0
        } else {
            1.0
        }
    }

    pub fn mic_node_weight<T: MutagenArg>(arg: T) -> f64 {
        if arg.mic_spectrograms().is_none() {
            0.0
        } else {
            1.0
        }
    }

    pub fn camera_node_weight<T: MutagenArg>(arg: T) -> f64 {
        if arg.camera_frames().is_none() {
            0.0
        } else {
            1.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[derive(Copy, Clone)]
        struct TestArg<'a> {
            depth: usize,
            gamepads: &'a Gamepads,
            mic_spectrograms: &'a Option<FrequencySpectrograms>,
            camera_frames: &'a Option<CameraFrames>,
        }

        impl<'a> MutagenArg for TestArg<'a> {
            fn depth(&self) -> usize {
                self.depth
            }

            fn gamepads(&self) -> &Gamepads {
                &self.gamepads
            }

            fn mic_spectrograms(&self) -> &Option<FrequencySpectrograms> {
                &self.mic_spectrograms
            }

            fn camera_frames(&self) -> &Option<CameraFrames> {
                &self.camera_frames
            }
        }

        #[test]
        fn all_depths_have_a_node() {
            for depth in 0..=max_node_depth() {
                let arg = TestArg {
                    depth,
                    gamepads: &Gamepads::new(),
                    mic_spectrograms: &Some(FrequencySpectrograms::new(256)),
                    camera_frames: &None,
                };

                assert!(
                    leaf_node_weight(arg) > 0.0
                        || pipe_node_weight(arg) > 0.0
                        || branch_node_weight(arg) > 0.0
                );
            }
        }

        #[test]
        fn max_depth_only_gens_leaf() {
            let arg = TestArg {
                depth: max_node_depth(),
                gamepads: &Gamepads::new(),
                mic_spectrograms: &Some(FrequencySpectrograms::new(256)),
                camera_frames: &None,
            };

            assert!(leaf_node_weight(arg) > 0.0);
            assert_eq!(pipe_node_weight(arg), 0.0);
            assert_eq!(branch_node_weight(arg), 0.0);
        }
    }
}
