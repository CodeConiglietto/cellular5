use std::fs;

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONSTS: Constants = serde_yaml::from_str(
        &fs::read_to_string("constants.yml").unwrap_or_else(|_e| panic!(
            "Couldn't find constants.yml in {}",
            std::env::current_dir().unwrap().to_string_lossy()
        ))
    )
    .unwrap_or_else(|e| panic!("Failed to parse constants.yml: {}", e));
}

#[derive(Deserialize)]
pub struct Constants {
    pub tics_per_update: usize,

    pub initial_window_width: f32,
    pub initial_window_height: f32,
    pub vsync: bool,
    pub fullscreen: bool,

    pub cell_array_width: usize,
    pub cell_array_height: usize,
    pub cell_array_history_length: usize,
    pub cell_array_lerp_length: usize,

    pub auto_mutate: bool,

    pub lerp_aggressiveness: f32,

    pub time_scale_divisor: f32,

    pub noise_x_scale_factor: f64,
    pub noise_y_scale_factor: f64,
    pub noise_t_scale_factor: f64,
    pub noise_x_scale_minimum: f64,
    pub noise_y_scale_minimum: f64,
    pub noise_t_scale_minimum: f64,

    pub activity_value_upper_bound: f64,
    pub activity_value_lower_bound: f64,
    pub alpha_value_upper_bound: f64,
    pub alpha_value_lower_bound: f64,
    pub local_similarity_upper_bound: f64,
    pub local_similarity_lower_bound: f64,
    pub global_similarity_upper_bound: f64,
    pub global_similarity_lower_bound: f64,

    pub image_path: String,

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
}
