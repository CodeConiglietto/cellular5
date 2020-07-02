use std::{
    f32::consts::PI,
    fs,
    iter::Sum,
    path::PathBuf,
};

use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color as GgColor, DrawParam, Image as GgImage, Rect, WHITE},
    input::keyboard,
    timer, Context, ContextBuilder, GameResult,
};
use log::{error, info};
use mutagen::Generatable;
use ndarray::{s, ArrayViewMut1, Axis};
use rand::prelude::*;
use rayon::prelude::*;
use structopt::StructOpt;

use crate::{
    arena_wrappers::*,
    constants::*,
    coordinate_set::*,
    data_set::*,
    datatype::{
        colors::{ByteColor, FloatColor},
        continuous::*,
        image::*,
        points::*,
    },
    history::*,
    mutagen_args::*,
    node::{Node, color_nodes::*},
    node_set::*,
    opts::Opts,
    update_stat::UpdateStat,
    util::{*, DeterministicRng, RNG_SEED},
};

mod arena_wrappers;
mod constants;
mod data_set;
mod datatype;
mod coordinate_set;
mod history;
mod mutagen_args;
mod node;
mod node_set;
mod opts;
mod preloader;
mod update_stat;
mod util;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    setup_logging();

    let opts = Opts::from_args();
    let (mut ctx, mut event_loop) = ContextBuilder::new("cellular4", "CodeBunny")
        .window_mode(
            WindowMode::default()
                .dimensions(CONSTS.initial_window_width, CONSTS.initial_window_height)
                .fullscreen_type(if CONSTS.fullscreen {
                    FullscreenType::Desktop
                } else {
                    FullscreenType::Windowed
                }),
        )
        .window_setup(
            WindowSetup::default()
                .title("Cellular 4")
                .vsync(CONSTS.vsync),
        )
        .build()
        .expect("Could not create ggez context!");

    let mut my_game = MyGame::new(&mut ctx, opts);

    // Eagerly initialize the image preloader rather than waiting for the first time it's used
    IMAGE_PRELOADER.with(|_| ());

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => info!("Exited cleanly."),
        Err(e) => error!("Error occurred: {}", e),
    }
}

fn setup_logging() {
    let image_error_dispatch = fern::Dispatch::new()
        .level(log::LevelFilter::Off)
        .level_for(datatype::image::MODULE_PATH, log::LevelFilter::Error)
        .chain(fern::log_file("image_errors.log").unwrap());

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for(module_path!(), log::LevelFilter::Trace)
        .chain(image_error_dispatch)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

// #[derive(Serialize, Deserialize, Generatable, Mutatable, UpdatableRecursively, Debug)]
// struct RenderNodes {
//     compute_offset_node: Box<CoordMapNodes>,

//     root_rotation_node: Box<SNFloatNodes>,
//     root_translation_node: Box<SNPointNodes>,
//     root_offset_node: Box<SNPointNodes>,
//     root_from_scale_node: Box<SNPointNodes>,
//     root_to_scale_node: Box<SNPointNodes>,

//     root_scalar_node: Box<UNFloatNodes>,
//     root_alpha_node: Box<UNFloatNodes>,

//     rotation_scalar_node: Box<UNFloatNodes>,
//     translation_scalar_node: Box<UNFloatNodes>,
//     offset_scalar_node: Box<UNFloatNodes>,
//     from_scale_scalar_node: Box<UNFloatNodes>,
//     to_scale_scalar_node: Box<UNFloatNodes>,
// }

// impl<'a> Mutagen<'a> for RenderNodes {
//     type Arg = UpdateState<'a>;
// }

// impl<'a> Updatable<'a> for RenderNodes {
//     fn update(&mut self, _state: mutagen::State, _arg: &'a mut UpdArg<'a>) {}
// }

// #[derive(Serialize, Deserialize, Generatable, Mutatable, UpdatableRecursively, Debug)]
// struct NodeTree {
//     //The root node for the tree that computes the next screen state
//     root_node: Box<FloatColorNodes>,
//     //Nodes for computing parameters for the next draw param
//     render_nodes: RenderNodes,
// }

// impl NodeTree {
//     fn try_save(&self, slot: &str) -> Fallible<()> {
//         info!("Saving tree to slot {}", slot);
//         let path = save_slot_path(slot);

//         fs::create_dir_all(path.parent().unwrap())?;
//         fs::write(&path, serde_yaml::to_vec(&self)?)?;

//         Ok(())
//     }

//     fn save(&self, slot: &str) {
//         self.try_save(slot)
//             .unwrap_or_else(|e| error!("Failed to save tree to slot '{}': {}", slot, e));
//     }

//     fn try_load(&mut self, slot: &str) -> Fallible<()> {
//         info!("Loading tree from slot {}", slot);
//         let loaded = serde_yaml::from_slice(&fs::read(&save_slot_path(slot))?)?;
//         *self = loaded;

//         Ok(())
//     }

//     fn load(&mut self, slot: &str) {
//         self.try_load(slot)
//             .unwrap_or_else(|e| error!("Failed to load tree from slot '{}': {}", slot, e));
//     }

//     fn try_graph(&self) -> Fallible<()> {
//         let tmp_dir = env::temp_dir().join("cellular3");
//         fs::create_dir_all(&tmp_dir)?;

//         let dot_path = tmp_dir.join("tree_graph.dot");
//         fs::write(&dot_path, &dot_serde::to_vec(&self.root_node)?)?;
//         let png_path = dot_path.with_extension("png");

//         ensure!(
//             Command::new("dot")
//                 .arg("-T")
//                 .arg("png")
//                 .arg("-o")
//                 .arg(&png_path)
//                 .arg(&dot_path)
//                 .status()?
//                 .success(),
//             "Could not run dot"
//         );

//         opener::open(png_path)?;

//         Ok(())
//     }

//     fn graph(&self) {
//         self.try_graph()
//             .unwrap_or_else(|e| error!("Failed to graph tree: {}", e));
//     }
// }

fn save_slot_path(slot: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("saves")
        .join(&format!("{}.yml", slot))
}

// impl<'a> Mutagen<'a> for NodeTree {
//     type Arg = UpdateState<'a>;
// }

// impl<'a> Updatable<'a> for NodeTree {
//     fn update(&mut self, _state: mutagen::State, _arg: &'a mut UpdArg<'a>) {}
// }

struct MyGame {
    //Screen bounds
    bounds: Rect,

    history: History,
    next_history_step: HistoryStep,

    //The rolling total used to calculate the average per update instead of per slice
    rolling_update_stat_total: UpdateStat,
    //The average update stat over time, calculated by averaging rolling total and itself once an update
    average_update_stat: UpdateStat,

    nodes: Vec<NodeSet>,
    data: DataSet,

    root_node: NodeBox<FloatColorNodes>,

    record_tree: bool,
    tree_dirty: bool,
    current_t: usize,
    last_render_t: usize,
    rng: DeterministicRng,
    opts: Opts,
}

impl MyGame {
    pub fn new(ctx: &mut Context, opts: Opts) -> MyGame {
        // Load/create resources such as images here.
        let (pixels_x, pixels_y) = ggez::graphics::size(ctx);

        if let Some(seed) = opts.seed {
            info!("Manually setting RNG seed");
            *RNG_SEED.lock().unwrap() = seed;
        }

        fs::write("last_seed.txt", &RNG_SEED.lock().unwrap().to_string()).unwrap();

        let mut rng = DeterministicRng::new();

        let history = History::new(
            ctx,
            CONSTS.cell_array_width,
            CONSTS.cell_array_height,
            CONSTS.cell_array_history_length,
        );

        let update_state = UpdateState {
            coordinate_set: CoordinateSet {
                x: SNFloat::ZERO,
                y: SNFloat::ZERO,
                t: 0.0,
            },
            history: &history,
        };

        let nodes = Vec::new();
        let data = DataSet::new();

        MyGame {
            bounds: Rect::new(0.0, 0.0, pixels_x, pixels_y),

            next_history_step: HistoryStep {
                cell_array: init_cell_array(CONSTS.cell_array_width, CONSTS.cell_array_height),
                computed_texture: GgImage::solid(ctx, 1, WHITE).unwrap(),
                update_coordinate: CoordinateSet {
                    x: SNFloat::ZERO,
                    y: SNFloat::ZERO,
                    t: 0.0,
                },
                rotation: 0.0,
                translation: SNPoint::zero(),
                offset: SNPoint::zero(),
                from_scale: SNPoint::zero(),
                to_scale: SNPoint::zero(),

                root_scalar: UNFloat::default(),
                alpha: UNFloat::default(),
                rotation_scalar: UNFloat::default(),
                translation_scalar: UNFloat::default(),
                offset_scalar: UNFloat::default(),
                from_scale_scalar: UNFloat::default(),
                to_scale_scalar: UNFloat::default(),
            },
            rolling_update_stat_total: UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            },
            average_update_stat: UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            },

            nodes: nodes,
            data: data,

            root_node: NodeBox::generate_rng(
                &mut rng,
                mutagen::State::default(),
                &mut GenArg {
                    nodes: &mut nodes,
                    data: &mut data,
                },
            ),

            record_tree: false,
            tree_dirty: false,
            current_t: 0,
            last_render_t: 0,
            rng,
            opts,
            history,
        }
    }
}

fn lerp(a: f32, b: f32, value: f32) -> f32 {
    a + (b - a) * value
}

impl EventHandler for MyGame {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
            return;
        }

        // if !repeat {
        //     let save_slot = match keycode {
        //         KeyCode::Key1 => Some("1"),
        //         KeyCode::Key2 => Some("2"),
        //         KeyCode::Key3 => Some("3"),
        //         KeyCode::Key4 => Some("4"),
        //         KeyCode::Key5 => Some("5"),
        //         KeyCode::Key6 => Some("6"),
        //         KeyCode::Key7 => Some("7"),
        //         KeyCode::Key8 => Some("8"),
        //         KeyCode::Key9 => Some("9"),
        //         KeyCode::Key0 => Some("0"),

        //         _ => None,
        //     };

        //     if let Some(save_slot) = save_slot {
        //         if keymods.contains(KeyMods::CTRL) {
        //             self.node_tree.save(save_slot);
        //         } else {
        //             self.node_tree.load(save_slot);
        //         }
        //     }

        //     if keycode == KeyCode::D {
        //         self.node_tree.graph();
        //     }

        //     if keycode == KeyCode::Tab {
        //         self.record_tree = !self.record_tree;

        //         if self.record_tree {
        //             self.node_tree.save("latest");
        //         }

        //         let title = if self.record_tree {
        //             "Cellular 3 (Recording)"
        //         } else {
        //             "Cellular 3"
        //         };

        //         graphics::set_window_title(ctx, title);
        //     }
        // }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.tree_dirty = true;
        }

        let current_t = self.current_t;

        let slice_height = CONSTS.cell_array_height / CONSTS.tics_per_update;
        let slice_y = (timer::ticks(ctx) % CONSTS.tics_per_update) * slice_height;
        let slice_y_range = slice_y..slice_y + slice_height;

        let mut new_update_slice =
            self.next_history_step
                .cell_array
                .slice_mut(s![slice_y_range, .., ..]);
        let new_update_iter = new_update_slice.lanes_mut(Axis(2));

        let history = &self.history;

        //let rule_sets = self.rule_sets;

        let root_node = &self.root_node;

        let compute_arg = ComArg{
            nodes: &self.nodes,
            data: &self.data,
        };

        let update_step = |y, x, mut new: ArrayViewMut1<u8>| {
            let total_cells = CONSTS.cell_array_width * CONSTS.cell_array_height;

            let compute_result = root_node.compute(&UpdateState {
                coordinate_set: CoordinateSet {
                    x: UNFloat::new(x as f32 / CONSTS.cell_array_width as f32).to_signed(),
                    y: UNFloat::new(
                        (y + slice_y as usize) as f32 / CONSTS.cell_array_height as f32,
                    )
                    .to_signed(),
                    t: current_t as f32,
                },
                history,
            },
            compute_arg);

            let new_color = ByteColor::from(compute_result);

            new[0] = new_color.r.into_inner();
            new[1] = new_color.g.into_inner();
            new[2] = new_color.b.into_inner();
            new[3] = new_color.a.into_inner();

            let current_color = history.get(x, y, current_t);
            let older_color = history.get(x, y, usize::max(current_t, 1) - 1);

            let local_offset = (thread_rng().gen_range(-1, 2), thread_rng().gen_range(-1, 2));
            let local_color = history.get(
                (x as i32 + local_offset.0)
                    .max(0)
                    .min(CONSTS.cell_array_width as i32 - 1) as usize,
                (y as i32 + local_offset.1).min(CONSTS.cell_array_height as i32 - 1) as usize,
                current_t,
            );
            let global_color = history.get(
                random::<usize>() % CONSTS.cell_array_width,
                random::<usize>() % CONSTS.cell_array_height,
                current_t,
            );

            let older_color: FloatColor = older_color.into();
            let current_color: FloatColor = current_color.into();
            let local_color: FloatColor = local_color.into();
            let global_color: FloatColor = global_color.into();

            UpdateStat {
                activity_value: (older_color.get_average() - current_color.get_average()).abs()
                    / total_cells as f32,
                alpha_value: current_color.a.into_inner() / total_cells as f32,
                local_similarity_value: (1.0
                    - (local_color.get_average() - current_color.get_average()).abs())
                    / total_cells as f32,
                global_similarity_value: (1.0
                    - (global_color.get_average() - current_color.get_average()).abs())
                    / total_cells as f32,
            }
        };

        let zip = ndarray::Zip::indexed(new_update_iter);

        let slice_update_stat: UpdateStat = if CONSTS.parallelize {
            zip.into_par_iter()
                .map(|((y, x), new)| update_step(y, x, new))
                .sum()
        } else {
            let mut stat = UpdateStat::default();
            zip.apply(|(y, x), new| stat += update_step(y, x, new));
            stat
        };

        self.rolling_update_stat_total += slice_update_stat;

        if timer::ticks(ctx) % CONSTS.tics_per_update == 0 {
            self.average_update_stat =
                (self.average_update_stat + self.rolling_update_stat_total) / 2.0;

            dbg!(timer::fps(ctx));

            self.rolling_update_stat_total = UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            };

            let update_state = UpdateState {
                coordinate_set: CoordinateSet {
                    x: SNFloat::ZERO,
                    y: SNFloat::ZERO,
                    t: current_t as f32,
                },
                history,
            };

            if self.tree_dirty
                || (CONSTS.auto_mutate
                    && (dbg!(thread_rng().gen::<u32>() % 500) == 0
                        || dbg!(f64::from(self.average_update_stat.activity_value))
                            < CONSTS.activity_value_lower_bound
                        || dbg!(f64::from(self.average_update_stat.alpha_value))
                            < CONSTS.alpha_value_lower_bound
                        || dbg!(f64::from(self.average_update_stat.local_similarity_value))
                            > CONSTS.local_similarity_upper_bound
                        || dbg!(f64::from(self.average_update_stat.global_similarity_value))
                            >= CONSTS.global_similarity_upper_bound))
            {
                info!("====TIC: {} MUTATING TREE====", self.current_t);
                // self.node_tree.root_node.mutate_rng(
                //     &mut self.rng,
                //     mutagen::State::default(),
                //     update_state,
                // );
                // self.node_tree.render_nodes.mutate_rng(
                //     &mut self.rng,
                //     mutagen::State::default(),
                //     update_state,
                // );
                // // info!("{:#?}", &self.root_node);
                // if self.record_tree {
                //     self.node_tree.save("latest");
                // }
                self.tree_dirty = false;
            }

            let hist_len = self.history.history_steps.len();
            let history_index = (self.current_t - 1) % hist_len;
            let history_step = &self.history.history_steps[history_index];

            // let last_update_state = UpdateState {
            //     coordinate_set: history_step.update_coordinate,
            //     history: &self.history,
            // };

            // self.next_history_step.update_coordinate = self
            //     .node_tree
            //     .render_nodes
            //     .compute_offset_node
            //     .compute(last_update_state);

            // let step_update_state = UpdateState {
            //     coordinate_set: self.next_history_step.update_coordinate,
            //     history: &self.history,
            // };

            // self.next_history_step.rotation = self
            //     .node_tree
            //     .render_nodes
            //     .root_rotation_node
            //     .compute(step_update_state)
            //     .into_inner();
            // self.next_history_step.translation = self
            //     .node_tree
            //     .render_nodes
            //     .root_translation_node
            //     .compute(step_update_state);
            // self.next_history_step.offset = self
            //     .node_tree
            //     .render_nodes
            //     .root_offset_node
            //     .compute(step_update_state);
            // self.next_history_step.from_scale = self
            //     .node_tree
            //     .render_nodes
            //     .root_from_scale_node
            //     .compute(step_update_state);
            // self.next_history_step.to_scale = self
            //     .node_tree
            //     .render_nodes
            //     .root_to_scale_node
            //     .compute(step_update_state);

            // self.next_history_step.root_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .root_scalar_node
            //     .compute(step_update_state);

            // self.next_history_step.alpha = self
            //     .node_tree
            //     .render_nodes
            //     .root_alpha_node
            //     .compute(step_update_state);

            // self.next_history_step.rotation_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .rotation_scalar_node
            //     .compute(step_update_state);

            // self.next_history_step.translation_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .translation_scalar_node
            //     .compute(step_update_state);

            // self.next_history_step.offset_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .offset_scalar_node
            //     .compute(step_update_state);

            // self.next_history_step.to_scale_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .to_scale_scalar_node
            //     .compute(step_update_state);

            // self.next_history_step.from_scale_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .from_scale_scalar_node
            //     .compute(step_update_state);

            // self.next_history_step.root_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .root_scalar_node
            //     .compute(step_update_state);

            self.next_history_step.computed_texture =
                compute_texture(ctx, self.next_history_step.cell_array.view());

            // self.node_tree
            //     .update_recursively(mutagen::State::default(), update_state);

            // Rotate the buffers by swapping
            let h_len = self.history.history_steps.len();
            std::mem::swap(
                &mut self.history.history_steps[current_t % h_len],
                &mut self.next_history_step,
            );

            self.current_t += 1;
        }

        timer::yield_now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.last_render_t != timer::ticks(ctx) {
            let base_params = DrawParam::new().dest([0.0, 0.0]).scale([
                self.bounds.w as f32 / CONSTS.cell_array_width as f32,
                self.bounds.h as f32 / CONSTS.cell_array_height as f32,
            ]);

            let lerp_value =
                (timer::ticks(ctx) % CONSTS.tics_per_update) as f32 / CONSTS.tics_per_update as f32;

            let lerp_len = CONSTS.cell_array_lerp_length;

            // let mut alphas = Vec::new();
            for i in 0..lerp_len {
                //let transparency = if i == 0 {1.0} else {if i == 1 {0.5} else {0.0}};
                let alpha = 1.0 - ((i as f32 - lerp_value) / (lerp_len - 1) as f32).max(0.0);

                //todo: put this fetching in its own function
                let hist_len = self.history.history_steps.len();
                let history_index = (self.current_t + i + hist_len - lerp_len) % hist_len;
                let history_step = &self.history.history_steps[history_index];

                let root_scalar =
                    history_step.root_scalar.into_inner() * history_step.root_scalar.into_inner();
                let rotation_scalar = history_step.rotation_scalar.into_inner()
                    * history_step.rotation_scalar.into_inner();
                let translation_scalar = history_step.translation_scalar.into_inner()
                    * history_step.translation_scalar.into_inner();
                let offset_scalar = history_step.offset_scalar.into_inner()
                    * history_step.offset_scalar.into_inner();
                let from_scale_scalar = history_step.from_scale_scalar.into_inner()
                    * history_step.from_scale_scalar.into_inner();
                let to_scale_scalar = history_step.to_scale_scalar.into_inner()
                    * history_step.to_scale_scalar.into_inner();

                let dest_offset_x = CONSTS.initial_window_width
                    * history_step.translation.into_inner().x
                    * 0.5
                    * (1.0 - alpha);
                let dest_offset_y = CONSTS.initial_window_height
                    * history_step.translation.into_inner().y
                    * 0.5
                    * (1.0 - alpha);

                let offset_offset_x = history_step.offset.into_inner().x * 0.5 * (1.0 - alpha);
                let offset_offset_y = history_step.offset.into_inner().y * 0.5 * (1.0 - alpha);

                let x_scale_ratio = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let y_scale_ratio = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

                let scale_x = lerp(
                    lerp(
                        1.0,
                        history_step.from_scale.into_inner().x * 2.0,
                        from_scale_scalar * root_scalar,
                    ),
                    lerp(
                        1.0,
                        history_step.to_scale.into_inner().x * 2.0,
                        to_scale_scalar * root_scalar,
                    ),
                    alpha,
                );
                let scale_y = lerp(
                    lerp(
                        1.0,
                        history_step.from_scale.into_inner().y * 2.0,
                        from_scale_scalar * root_scalar,
                    ),
                    lerp(
                        1.0,
                        history_step.to_scale.into_inner().y * 2.0,
                        to_scale_scalar * root_scalar,
                    ),
                    alpha,
                );

                let rotation = (1.0 - alpha) * history_step.rotation * PI;

                ggez::graphics::draw(
                    ctx,
                    &history_step.computed_texture,
                    base_params
                        .color(GgColor::new(
                            1.0,
                            1.0,
                            1.0,
                            ((1.0 - ((alpha * 2.0) - 1.0).abs())
                                / CONSTS.cell_array_lerp_length as f32)
                                * lerp(1.0, history_step.alpha.into_inner(), root_scalar),
                        ))
                        .dest([
                            ((CONSTS.initial_window_width * 0.5)
                                + dest_offset_x * scale_x * translation_scalar * root_scalar),
                            ((CONSTS.initial_window_height * 0.5)
                                + dest_offset_y * scale_y * translation_scalar * root_scalar),
                        ])
                        .offset([
                            0.5 + offset_offset_x * scale_x * offset_scalar * root_scalar,
                            0.5 + offset_offset_y * scale_y * offset_scalar * root_scalar,
                        ])
                        .scale([
                            lerp(1.0, 1.0 + scale_x, root_scalar) * x_scale_ratio,
                            lerp(1.0, 1.0 + scale_y, root_scalar) * y_scale_ratio,
                        ])
                        .rotation(rotation * rotation_scalar * root_scalar),
                )?;

                // alphas.push(alpha);
            }

            // info!("{}", alphas.iter().map(|a| format!("{:.2}", a)).join(","));

            self.last_render_t = timer::ticks(ctx);
        }
        graphics::present(ctx)?;

        Ok(())
    }
}
