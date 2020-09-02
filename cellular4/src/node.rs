pub mod automata_nodes;
pub mod color_blend_nodes;
pub mod color_nodes;
pub mod complex_nodes;
pub mod constraint_resolver_nodes;
pub mod continuous_nodes;
pub mod coord_map_nodes;
pub mod discrete_nodes;
pub mod distance_function_nodes;
pub mod iterative_function_nodes;
pub mod matrix_nodes;
pub mod noise_nodes;
pub mod point_nodes;
pub mod point_set_nodes;

use crate::mutagen_args::ComArg;

pub trait Node {
    type Output;

    fn compute(&self, compute_arg: ComArg) -> Self::Output;
}

mod mutagen_functions {
    use crate::{constants::*, util::*};

    pub fn leaf_node_weight(state: &mutagen::State) -> f64 {
        if state.depth < CONSTS.min_leaf_depth || state.depth > CONSTS.max_leaf_depth {
            0.0
        } else {
            map_range(
                state.depth as f32,
                (CONSTS.min_leaf_depth as f32, CONSTS.max_leaf_depth as f32),
                (0.0, 1.0),
            ) as f64
        }
    }

    pub fn pipe_node_weight(state: &mutagen::State) -> f64 {
        if state.depth < CONSTS.min_pipe_depth || state.depth > CONSTS.max_pipe_depth {
            0.0
        } else {
            1.0 - map_range(
                state.depth as f32,
                (CONSTS.min_pipe_depth as f32, CONSTS.max_pipe_depth as f32),
                (0.0, 1.0),
            ) as f64
        }
    }

    pub fn branch_node_weight(state: &mutagen::State) -> f64 {
        if state.depth < CONSTS.min_branch_depth || state.depth > CONSTS.max_branch_depth {
            0.0
        } else {
            1.0 - map_range(
                state.depth as f32,
                (
                    CONSTS.min_branch_depth as f32,
                    CONSTS.max_branch_depth as f32,
                ),
                (0.0, 1.0),
            ) as f64
        }
    }
}
