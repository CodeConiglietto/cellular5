use std::{f32::consts::PI, fs};

use cpu_monitor::CpuInstant;
use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color as GgColor, DrawParam, Image as GgImage, WHITE},
    input::keyboard,
    timer, Context, ContextBuilder, GameResult,
};
use log::{error, info};
use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use ndarray::{s, ArrayViewMut1, Axis};
use rand::prelude::*;
use rayon::prelude::*;
use structopt::StructOpt;

use crate::{
    arena_wrappers::*, data_set::*, history::*, node_set::*, opts::Opts, prelude::*,
    update_stat::UpdateStat,
};

pub mod arena_wrappers;
pub mod constants;
pub mod coordinate_set;
pub mod data_set;
pub mod datatype;
pub mod history;
pub mod mutagen_args;
pub mod node;
pub mod node_set;
pub mod opts;
pub mod preloader;
pub mod prelude;
pub mod update_stat;
pub mod util;

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

#[derive(Debug, Generatable, Mutatable, UpdatableRecursively)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
struct RenderNodes {
    compute_offset_node: NodeBox<CoordMapNodes>,

    root_rotation_node: NodeBox<AngleNodes>,
    root_translation_node: NodeBox<SNPointNodes>,
    root_offset_node: NodeBox<SNPointNodes>,
    root_from_scale_node: NodeBox<SNPointNodes>,
    root_to_scale_node: NodeBox<SNPointNodes>,

    root_scalar_node: NodeBox<UNFloatNodes>,
    root_alpha_node: NodeBox<UNFloatNodes>,

    rotation_scalar_node: NodeBox<UNFloatNodes>,
    translation_scalar_node: NodeBox<UNFloatNodes>,
    offset_scalar_node: NodeBox<UNFloatNodes>,
    from_scale_scalar_node: NodeBox<UNFloatNodes>,
    to_scale_scalar_node: NodeBox<UNFloatNodes>,
}

impl<'a> Updatable<'a> for RenderNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

#[derive(Debug, Generatable, Mutatable, UpdatableRecursively)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
struct NodeTree {
    //The root node for the tree that computes the next screen state
    root_node: NodeBox<FloatColorNodes>,
    //Nodes for computing parameters for the next draw param
    render_nodes: RenderNodes,
}

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

//fn save_slot_path(slot: &str) -> PathBuf {
//    std::env::current_dir()
//        .unwrap()
//        .join("saves")
//        .join(&format!("{}.yml", slot))
//}

impl<'a> Updatable<'a> for NodeTree {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {}
}

struct MyGame {
    history: History,
    next_history_step: HistoryStep,

    //The rolling total used to calculate the average per update instead of per slice
    rolling_update_stat_total: UpdateStat,
    //The average update stat over time, calculated by averaging rolling total and itself once an update
    average_update_stat: UpdateStat,

    nodes: Vec<NodeSet>,
    data: DataSet,

    node_tree: NodeTree,

    //record_tree: bool,
    tree_dirty: bool,
    current_t: usize,
    last_render_t: usize,
    cpu_t: CpuInstant,
    rng: DeterministicRng,
}

impl MyGame {
    pub fn new(ctx: &mut Context, opts: Opts) -> MyGame {
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

        let mut nodes: Vec<_> = (0..CONSTS
            .max_branch_depth
            .max(CONSTS.max_pipe_depth.max(CONSTS.max_leaf_depth))
            + 1)
            .map(|_| NodeSet::new())
            .collect();
        let mut data = DataSet::new();

        MyGame {
            next_history_step: HistoryStep {
                cell_array: init_cell_array(CONSTS.cell_array_width, CONSTS.cell_array_height),
                computed_texture: GgImage::solid(ctx, 1, WHITE).unwrap(),
                update_coordinate: CoordinateSet {
                    x: SNFloat::ZERO,
                    y: SNFloat::ZERO,
                    t: 0.0,
                },
                rotation: Angle::ZERO,
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
                cpu_usage: 0.0,
            },
            average_update_stat: UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
                cpu_usage: 0.0,
            },

            node_tree: Generatable::generate_rng(
                &mut rng,
                mutagen::State::default(),
                GenArg {
                    nodes: &mut nodes,
                    data: &mut data,
                    depth: 0,
                    current_t: 0,
                },
            ),

            nodes,
            data,

            //record_tree: false,
            tree_dirty: false,
            current_t: 0,
            last_render_t: 0,
            cpu_t: CpuInstant::now().unwrap(),
            rng,
            history,
        }
    }
}

impl EventHandler for MyGame {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
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

        let root_node = &self.node_tree.root_node;
        let nodes = &self.nodes;
        let data = &self.data;
        let total_cells = CONSTS.cell_array_width * CONSTS.cell_array_height;

        let update_step = |y, x, mut new: ArrayViewMut1<u8>| {
            let new_color = ByteColor::from(
                root_node.compute(ComArg {
                    nodes,
                    data,
                    coordinate_set: CoordinateSet {
                        x: UNFloat::new(x as f32 / CONSTS.cell_array_width as f32).to_signed(),
                        y: UNFloat::new(
                            (y + slice_y as usize) as f32 / CONSTS.cell_array_height as f32,
                        )
                        .to_signed(),
                        t: current_t as f32,
                    },
                    history,
                    depth: 0,
                }),
            );

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
                activity_value: f64::from(older_color.get_average() - current_color.get_average())
                    .abs(), // / total_cells as f64
                alpha_value: f64::from(current_color.a.into_inner()), // / total_cells as f64
                local_similarity_value: f64::from(
                    1.0 - (local_color.get_average() - current_color.get_average()).abs(),
                ), // / total_cells as f64
                global_similarity_value: f64::from(
                    1.0 - (global_color.get_average() - current_color.get_average()).abs(),
                ), // / total_cells as f64
                cpu_usage: 0.0,//we don't accumulate this here because we set it below
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
        } / total_cells as f64;

        self.rolling_update_stat_total += slice_update_stat;

        if timer::ticks(ctx) % CONSTS.tics_per_update == 0 {
            let next_cpu_t = CpuInstant::now().unwrap();
            let cpu_usage = (next_cpu_t - self.cpu_t).non_idle();

            self.average_update_stat =
                ((self.average_update_stat + self.rolling_update_stat_total) / 2.0).clamp_values();

            dbg!(timer::fps(ctx));

            self.rolling_update_stat_total = UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
                cpu_usage: cpu_usage
            };

            let _update_state = UpdateState {
                coordinate_set: CoordinateSet {
                    x: SNFloat::ZERO,
                    y: SNFloat::ZERO,
                    t: current_t as f32,
                },
                history,
            };

            let mutation_likelihood = &self.average_update_stat.mutation_likelihood();

            dbg!(&self.average_update_stat);
            dbg!(mutation_likelihood);

            if self.tree_dirty
                || (CONSTS.auto_mutate
                    && (dbg!(cpu_usage >= CONSTS.auto_mutate_above_cpu_usage)
                        || dbg!(self.average_update_stat.should_mutate())
                        || dbg!(thread_rng().gen::<usize>() % CONSTS.graph_mutation_divisor) == 0))
            {
                info!("====TIC: {} MUTATING TREE====", self.current_t);
                self.node_tree.root_node.mutate_rng(
                    &mut self.rng,
                    mutagen::State::default(),
                    MutArg {
                        nodes: &mut self.nodes,
                        data: &mut self.data,
                        depth: 0,
                        current_t,
                    },
                );
                self.node_tree.render_nodes.mutate_rng(
                    &mut self.rng,
                    mutagen::State::default(),
                    MutArg {
                        nodes: &mut self.nodes,
                        data: &mut self.data,
                        depth: 0,
                        current_t,
                    },
                );
                // // info!("{:#?}", &self.root_node);
                // if self.record_tree {
                //     self.node_tree.save("latest");
                // }
                self.tree_dirty = false;
            }

            let history_len = self.history.history_steps.len();
            let history_index = self.current_t.saturating_sub(1) % history_len;
            let history_step = &self.history.history_steps[history_index];

            // let last_update_state = UpdateState {
            //     coordinate_set: history_step.update_coordinate,
            //     history: &self.history,
            // };

            let last_update_arg = UpdArg {
                coordinate_set: history_step.update_coordinate,
                history: &self.history,
                nodes: &mut self.nodes,
                data: &mut self.data,
                depth: 0,
                current_t,
            };

            self.next_history_step.update_coordinate = self
                .node_tree
                .render_nodes
                .compute_offset_node
                .compute(last_update_arg.into());

            let mut step_upd_arg = UpdArg {
                coordinate_set: self.next_history_step.update_coordinate,
                history: &self.history,
                nodes: &mut self.nodes,
                data: &mut self.data,
                depth: 0,
                current_t,
            };

            let mut step_com_arg: ComArg = step_upd_arg.reborrow().into();

            self.next_history_step.rotation = self
                .node_tree
                .render_nodes
                .root_rotation_node
                .compute(step_com_arg.reborrow()); //.average(history_step.rotation);
            self.next_history_step.translation = self
                .node_tree
                .render_nodes
                .root_translation_node
                .compute(step_com_arg.reborrow()); //.average(history_step.translation);
            self.next_history_step.offset = self
                .node_tree
                .render_nodes
                .root_offset_node
                .compute(step_com_arg.reborrow()); //.average(history_step.offset);
            self.next_history_step.from_scale = self
                .node_tree
                .render_nodes
                .root_from_scale_node
                .compute(step_com_arg.reborrow()); //.average(history_step.from_scale);
            self.next_history_step.to_scale = self
                .node_tree
                .render_nodes
                .root_to_scale_node
                .compute(step_com_arg.reborrow()); //.average(history_step.to_scale);

            self.next_history_step.root_scalar = dbg!(UNFloat::new(
                // (self
                // .node_tree
                // .render_nodes
                // .root_scalar_node
                // .compute(step_com_arg.reborrow()).average(history_step.root_scalar).into_inner() * 
                mutation_likelihood.powf(4.0) as f32
            // )
            ));

            self.next_history_step.alpha = self
                .node_tree
                .render_nodes
                .root_alpha_node
                .compute(step_com_arg.reborrow()); //.average(history_step.alpha);

            self.next_history_step.rotation_scalar = self
                .node_tree
                .render_nodes
                .rotation_scalar_node
                .compute(step_com_arg.reborrow()); //.average(history_step.rotation_scalar);

            self.next_history_step.translation_scalar = self
                .node_tree
                .render_nodes
                .translation_scalar_node
                .compute(step_com_arg.reborrow()); //.average(history_step.translation_scalar);

            self.next_history_step.offset_scalar = self
                .node_tree
                .render_nodes
                .offset_scalar_node
                .compute(step_com_arg.reborrow()); //.average(history_step.offset_scalar);

            self.next_history_step.to_scale_scalar = self
                .node_tree
                .render_nodes
                .to_scale_scalar_node
                .compute(step_com_arg.reborrow()); //.average(history_step.to_scale_scalar);

            self.next_history_step.from_scale_scalar = self
                .node_tree
                .render_nodes
                .from_scale_scalar_node
                .compute(step_com_arg.reborrow()); //.average(history_step.from_scale_scalar);

            // self.next_history_step.root_scalar = self
            //     .node_tree
            //     .render_nodes
            //     .root_scalar_node
            //     .compute(step_com_arg.reborrow())
                // .multiply(UNFloat::new_clamped(
                //     1.0 - self.average_update_stat.activity_value as f32,
                // ))
                // .multiply(UNFloat::new_clamped(
                //     1.0 - self.average_update_stat.alpha_value as f32,
                // ))
                // .multiply(UNFloat::new_clamped(
                //     self.average_update_stat.global_similarity_value as f32,
                // ))
                // .multiply(UNFloat::new_clamped(
                //     1.0 - self.average_update_stat.local_similarity_value as f32,
                // ))
                // .average(history_step.root_scalar);

            self.next_history_step.computed_texture =
                compute_texture(ctx, self.next_history_step.cell_array.view());

            self.node_tree
                .update_recursively(mutagen::State::default(), step_upd_arg.reborrow());

            let max_node_depth = self.nodes.len();

            for i in 0..max_node_depth {
                let (nodes_a, children) = self.nodes.split_at_mut(i + 1);
                let current = &mut nodes_a[i];

                let mut step_upd_arg = UpdArg {
                    coordinate_set: self.next_history_step.update_coordinate,
                    history: &self.history,
                    nodes: children,
                    data: &mut self.data,
                    depth: 0,
                    current_t,
                };

                current.update_recursively(mutagen::State::default(), step_upd_arg.reborrow());
            }

            // Rotate the buffers by swapping
            let h_len = self.history.history_steps.len();
            std::mem::swap(
                &mut self.history.history_steps[current_t % h_len],
                &mut self.next_history_step,
            );

            self.current_t += 1;
            self.cpu_t = next_cpu_t;
        }

        timer::yield_now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.last_render_t != timer::ticks(ctx) {
            let lerp_sub =
                (timer::ticks(ctx) % CONSTS.tics_per_update) as f32 / CONSTS.tics_per_update as f32;

            let lerp_len = CONSTS.cell_array_lerp_length;

            for i in 0..lerp_len {
                //let transparency = if i == 0 {1.0} else {if i == 1 {0.5} else {0.0}};
                let back_lerp_val = (i as f32 + (1.0 - lerp_sub)) / lerp_len as f32;
                let alpha = 1.0 - back_lerp_val;

                let _lerp_val = (i as f32 + lerp_sub) / lerp_len as f32;

                let history_len = self.history.history_steps.len();

                let prev_history_index =
                    (self.current_t + i + history_len - lerp_len - 1) % history_len;
                let history_index = (prev_history_index + 1) % history_len;

                let prev_history_step = &self.history.history_steps[prev_history_index];
                let history_step = &self.history.history_steps[history_index];

                let mut alpha =
                    (1.0 - ((alpha * 2.0) - 1.0).abs()) / CONSTS.cell_array_lerp_length as f32;

                let mut dest_x = CONSTS.initial_window_width * 0.5;
                let mut dest_y = CONSTS.initial_window_height * 0.5;

                let mut offset_x = 0.5;
                let mut offset_y = 0.5;

                let mut scale_x = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
                let mut scale_y = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

                let mut rotation = 0.0;

                if CONSTS.apply_frame_transformations || self.tree_dirty {
                    let root_scalar = lerp(
                        prev_history_step.root_scalar.into_inner(),
                        history_step.root_scalar.into_inner(),
                        back_lerp_val,
                    );

                    let rotation_scalar = lerp(
                        prev_history_step.rotation_scalar.into_inner(),
                        history_step.rotation_scalar.into_inner(),
                        back_lerp_val,
                    );

                    let translation_scalar = lerp(
                        prev_history_step.translation_scalar.into_inner(),
                        history_step.translation_scalar.into_inner(),
                        back_lerp_val,
                    );

                    let offset_scalar = lerp(
                        prev_history_step.offset_scalar.into_inner(),
                        history_step.offset_scalar.into_inner(),
                        back_lerp_val,
                    );

                    let from_scale_scalar = lerp(
                        prev_history_step.from_scale_scalar.into_inner(),
                        history_step.from_scale_scalar.into_inner(),
                        back_lerp_val,
                    );

                    let to_scale_scalar = lerp(
                        prev_history_step.to_scale_scalar.into_inner(),
                        history_step.to_scale_scalar.into_inner(),
                        back_lerp_val,
                    );

                    let translation_x = lerp(
                        prev_history_step.translation.into_inner().x,
                        history_step.translation.into_inner().x,
                        back_lerp_val,
                    ) * 0.5
                        * CONSTS.initial_window_width;

                    let translation_y = lerp(
                        prev_history_step.translation.into_inner().y,
                        history_step.translation.into_inner().y,
                        back_lerp_val,
                    ) * 0.5
                        * CONSTS.initial_window_height;

                    let offset_translation_x = lerp(
                        prev_history_step.offset.into_inner().x,
                        history_step.offset.into_inner().x,
                        back_lerp_val,
                    ) * 0.5;

                    let offset_translation_y = lerp(
                        prev_history_step.offset.into_inner().y,
                        history_step.offset.into_inner().y,
                        back_lerp_val,
                    ) * 0.5;

                    alpha *= lerp(1.0, history_step.alpha.into_inner(), root_scalar);
                    dest_x += translation_x * scale_x * translation_scalar * root_scalar;
                    dest_y += translation_y * scale_y * translation_scalar * root_scalar;
                    offset_x += offset_translation_x * scale_x * offset_scalar * root_scalar;
                    offset_y += offset_translation_y * scale_y * offset_scalar * root_scalar;

                    // NOTE PI is only subtracted because angles are 0..2PI currently
                    rotation += (1.0 - alpha) * (history_step.rotation.into_inner() - PI) * rotation_scalar * root_scalar;

                    scale_x *= lerp(
                        1.0,
                        lerp(
                            lerp(
                                1.0,
                                history_step.from_scale.into_inner().x,
                                from_scale_scalar,
                            ),
                            lerp(1.0, history_step.to_scale.into_inner().x, to_scale_scalar),
                            alpha,
                        ) * 2.0,
                        root_scalar,
                    );

                    scale_y *= lerp(
                        1.0,
                        lerp(
                            lerp(
                                1.0,
                                history_step.from_scale.into_inner().y,
                                from_scale_scalar,
                            ),
                            lerp(1.0, history_step.to_scale.into_inner().y, to_scale_scalar),
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

            self.last_render_t = timer::ticks(ctx);
        }
        graphics::present(ctx)?;

        Ok(())
    }
}
