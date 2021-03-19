use std::fs;

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::util;

lazy_static! {
    pub static ref CONSTS: Constants = {
        let path = util::local_path("constants.yml");

        serde_yaml::from_str(&fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "Couldn't read constants.yml in {}: {}",
                path.to_string_lossy(),
                e
            )
        }))
        .unwrap_or_else(|e| panic!("Failed to parse constants.yml: {}", e))
    };
}

#[derive(Deserialize)]
pub struct Constants {
    pub tics_per_update: usize,

    pub initial_window_width: f32,
    pub initial_window_height: f32,
    pub vsync: bool,
    pub fullscreen: bool,
    pub console_width: usize,
    pub fancy_terminal: bool,

    pub cell_array_width: usize,
    pub cell_array_height: usize,
    pub cell_array_history_length: usize,
    pub cell_array_lerp_length: usize,

    pub apply_frame_transformations: bool,

    pub auto_mutate: bool,
    pub auto_mutate_above_cpu_usage: f64,

    pub lerp_aggressiveness: f32,

    pub time_scale_divisor: f32,

    pub noise_x_scale_factor: f64,
    pub noise_y_scale_factor: f64,
    pub noise_t_scale_factor: f64,
    pub noise_x_scale_minimum: f64,
    pub noise_y_scale_minimum: f64,
    pub noise_t_scale_minimum: f64,

    pub graph_mutation_divisor: usize,

    pub activity_value_upper_bound: f64,
    pub activity_value_lower_bound: f64,
    pub alpha_value_upper_bound: f64,
    pub alpha_value_lower_bound: f64,
    pub local_similarity_upper_bound: f64,
    pub local_similarity_lower_bound: f64,
    pub global_similarity_upper_bound: f64,
    pub global_similarity_lower_bound: f64,

    pub image_path: String,
    pub image_download_probability: f64,

    //primitive consts
    pub byte_max_value: u64,
    pub byte_possible_values: u64,

    //neighbour consts
    pub max_neighbour_array_count: usize, //Use this for array indexes as it counts zero
    pub max_neighbour_count: i32,         //Use this for total neighbours excluding zero

    //color consts
    pub max_colors: usize,

    pub parallelize: bool,

    pub graph_convergence: f64,
    pub node_regenerate_chance: f64,

    pub min_leaf_depth: usize,
    pub max_leaf_depth: usize,

    pub min_pipe_depth: usize,
    pub max_pipe_depth: usize,

    pub min_branch_depth: usize,
    pub max_branch_depth: usize,

    pub gamepad_node_weight_mod: f64,

    pub mic: Option<MicConfig>,

    pub smithsonian_api_key: Option<String>,
    pub gfycat: Option<GfycatConfig>,

    pub mutagen_profiler: bool,
    pub mutagen_profiler_graphs: bool,
}

#[derive(Clone, Deserialize)]
pub struct MicConfig {
    pub min_frequency: f32,
    pub max_frequency: f32,
    pub gamma: f32,
    pub lerp_factor: f32,
    pub target_fps: f32,
}

#[derive(Clone, Deserialize)]
pub struct GfycatConfig {
    pub client_id: String,
    pub client_secret: String,
    pub trending: bool,
    pub trending_weight: f64,
    pub exclude_nsfw: bool,
    #[serde(default)]
    pub search_terms: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_default_constants() {
        let _: Constants = serde_yaml::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../constants.default.yml"
        )))
        .unwrap();
    }
}
