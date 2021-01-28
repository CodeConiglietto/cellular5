#![allow(clippy::large_enum_variant)]

use std::{fs, rc::Rc};

use cpu_monitor::CpuInstant;
use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::Image as GgImage,
    input::keyboard,
    timer, Context, ContextBuilder, GameResult,
};
use log::{info, warn};
use mutagen::{Generatable, Mutatable, Reborrow, Updatable, UpdatableRecursively};
use ndarray::{s, ArrayViewMut1, Axis as NdAxis};
use rand::prelude::*;
use rayon::prelude::*;
use structopt::StructOpt;

use crate::{
    arena_wrappers::*, data_set::*, history::*, node_set::*, opts::Opts, prelude::*, ui::*,
    update_stat::UpdateStat,
};

// Shamelessly copied from the std implementation of dbg!
// Macro declaration order matters! Keep this BEFORE any code and any module declarations
macro_rules! ldbg {
    () => {
        ::log::trace!("[{}:{}]", ::std::file!(), ::std::line!())
    };

    ($val:expr) => {
        match $val {
            tmp => {
                ::log::trace!("[{}:{}] {} = {:#?}",
                              ::std::file!(), ::std::line!(), ::std::stringify!($val), &tmp);
                tmp
            }
        }
    };

    ($val:expr,) => {
        $crate::ldbg!($val)
    };

    ($($val:expr),+ $(,)?) => {
        ($($crate::ldbg!($val)),+,)
    };
}

pub mod arena_wrappers;
pub mod constants;
pub mod coordinate_set;
pub mod data_set;
pub mod datatype;
pub mod gamepad;
pub mod history;
pub mod mutagen_args;
pub mod node;
pub mod node_set;
pub mod opts;
pub mod preloader;
pub mod prelude;
pub mod profiler;
pub mod ui;
pub mod update_stat;
pub mod util;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let opts = Opts::from_args();

    // We initialize the preloader before the ggez context so it is destroyed after the context.
    // The preloader can take a while to destroy since it may be waiting on IO/network,and we want the window to close responsively
    let image_preloader = Rc::new(Preloader::new(32, RandomImageLoader::new));

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

    let mut my_game = MyGame::new(&mut ctx, opts, Rc::clone(&image_preloader));

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }

    if CONSTS.mutagen_profiler_graphs {
        println!("Generating graphs...");

        match MutagenProfiler::load(MutagenProfiler::default_path()) {
            Ok(profiler) => profiler
                .save_graphs(MutagenProfiler::default_graphs_path())
                .unwrap_or_else(|e| warn!("Failed to save profiler graphs: {}", e)),
            Err(e) => warn!("Failed to load profiler for graphing: {}", e),
        }

        println!("Done!");
    }
}

fn setup_logging(ui: &Ui) {
    let image_error_dispatch = fern::Dispatch::new()
        .level(log::LevelFilter::Off)
        .level_for(datatype::image::MODULE_PATH, log::LevelFilter::Error)
        .chain(fern::log_file(util::local_path("image_errors.log")).unwrap());

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
        .chain(ui.log_output())
        .apply()
        .unwrap();
}

#[derive(Debug, Generatable, Mutatable, UpdatableRecursively)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
struct NodeTree {
    /// The root node for the tree that computes the next screen state
    root_node: NodeBox<FloatColorNodes>,
    root_coordinate_node: NodeBox<CoordMapNodes>,
    root_frame_renderer: NodeBox<FrameRendererNodes>,
    compute_offset_node: NodeBox<CoordMapNodes>,
    fade_color_node: NodeBox<FloatColorNodes>,
    fade_color_alpha_multiplier: NodeBox<UNFloatNodes>,
    scaling_mode_node: NodeBox<BooleanNodes>,
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

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

struct MyGame {
    history: History,
    next_history_step: HistoryStep,

    blank_texture: GgImage,
    gamepads: Gamepads,

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
    last_mutation_t: usize,
    last_render_t: usize,
    cpu_t: CpuInstant,
    rng: DeterministicRng,
    ui: Ui,

    image_preloader: Rc<Preloader<Image>>,
    profiler: Option<MutagenProfiler>,
}

impl MyGame {
    pub fn new(ctx: &mut Context, opts: Opts, image_preloader: Rc<Preloader<Image>>) -> MyGame {
        if let Some(seed) = opts.seed {
            info!("Manually setting RNG seed");
            *RNG_SEED.lock().unwrap() = seed;
        }

        fs::write(
            util::local_path("last_seed.txt"),
            &RNG_SEED.lock().unwrap().to_string(),
        )
        .unwrap();

        let mut rng = DeterministicRng::new();

        let history = History::new(
            ctx,
            CONSTS.cell_array_width,
            CONSTS.cell_array_height,
            CONSTS.cell_array_history_length,
        );

        let mut nodes: Vec<_> = (0..=node::max_node_depth())
            .map(|_| NodeSet::new())
            .collect();
        let mut data = DataSet::new();

        let ui = Ui::new();
        setup_logging(&ui);

        let mut profiler = if CONSTS.mutagen_profiler {
            Some(
                MutagenProfiler::load(MutagenProfiler::default_path()).unwrap_or_else(|e| {
                    warn!("Failed to load profiler data: {}", e);
                    MutagenProfiler::new()
                }),
            )
        } else {
            None
        };

        let mut gamepads = Gamepads::new();

        MyGame {
            blank_texture: compute_blank_texture(ctx),
            next_history_step: HistoryStep::new(
                ctx,
                CONSTS.cell_array_width,
                CONSTS.cell_array_height,
                false,
            ),
            rolling_update_stat_total: UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
                graph_stability: 0.0,
                cpu_usage: 0.0,
            },
            average_update_stat: UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
                graph_stability: 0.0,
                cpu_usage: 0.0,
            },

            node_tree: Generatable::generate_rng(
                &mut rng,
                GenArg {
                    nodes: &mut nodes,
                    data: &mut data,
                    depth: 0,
                    current_t: 0,
                    history: &history,
                    coordinate_set: history.history_steps[0].update_coordinate,
                    image_preloader: &*image_preloader,
                    profiler: &mut profiler,
                    gamepads: &mut gamepads,
                },
            ),

            nodes,
            data,

            //record_tree: false,
            tree_dirty: false,
            current_t: 0,
            last_mutation_t: 0,
            last_render_t: 0,
            cpu_t: CpuInstant::now().unwrap(),
            ui,
            rng,
            history,
            image_preloader,
            profiler,
            gamepads,
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

    fn gamepad_button_down_event(&mut self, ctx: &mut Context, _btn: GgButton, id: GgGamepadId) {
        self.gamepads.register_gamepad(ctx, id);
    }

    fn gamepad_button_up_event(&mut self, ctx: &mut Context, _btn: GgButton, id: GgGamepadId) {
        self.gamepads.register_gamepad(ctx, id);
    }

    fn gamepad_axis_event(
        &mut self,
        ctx: &mut Context,
        _axis: GgAxis,
        _value: f32,
        id: GgGamepadId,
    ) {
        self.gamepads.register_gamepad(ctx, id);
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.tree_dirty = true;
        }

        self.gamepads.update(ctx);

        let current_t = self.current_t;

        let slice_height = CONSTS.cell_array_height / CONSTS.tics_per_update;
        let slice_y = (timer::ticks(ctx) % CONSTS.tics_per_update) * slice_height;
        let slice_y_range = slice_y..slice_y + slice_height;

        let mut new_update_slice =
            self.next_history_step
                .cell_array
                .slice_mut(s![slice_y_range, .., ..]);
        let new_update_iter = new_update_slice.lanes_mut(NdAxis(2));

        let history = &self.history;
        let gamepads = &self.gamepads;

        //let rule_sets = self.rule_sets;

        let root_node = &self.node_tree.root_node;
        let root_coordinate_node = &self.node_tree.root_coordinate_node;
        let nodes = &self.nodes;
        let data = &self.data;
        let total_cells = CONSTS.cell_array_width * CONSTS.cell_array_height;

        let update_step = |y, x, mut new: ArrayViewMut1<u8>| {
            let coordinate_set = CoordinateSet {
                x: UNFloat::new(x as f32 / CONSTS.cell_array_width as f32).to_signed(),
                y: UNFloat::new((y + slice_y as usize) as f32 / CONSTS.cell_array_height as f32)
                    .to_signed(),
                t: current_t as f32,
            };

            let mut compute_arg = ComArg {
                nodes,
                data,
                current_t,
                coordinate_set,
                history,
                depth: 0,
                gamepads,
            };

            let transformed_coords = root_coordinate_node.compute(compute_arg.reborrow());

            let new_color = ByteColor::from(
                root_node.compute(compute_arg.replace_coordinate_set(&transformed_coords)),
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
                graph_stability: 0.0, //we don't accumulate this here because we set it below
                cpu_usage: 0.0,       //we don't accumulate this here because we set it below
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
            let graph_stability = 1.0 - 0.95_f64.powf((current_t - self.last_mutation_t) as f64);

            self.average_update_stat =
                ((self.average_update_stat + self.rolling_update_stat_total) / 2.0).clamp_values();

            //dbg!(timer::fps(ctx));

            self.rolling_update_stat_total = UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
                graph_stability,
                cpu_usage,
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

            //dbg!(&self.average_update_stat);
            //dbg!(mutation_likelihood);

            let history_len = self.history.history_steps.len();
            let history_index = self.current_t.saturating_sub(1) % history_len;
            let history_step = &self.history.history_steps[history_index];

            if self.tree_dirty
                || (CONSTS.auto_mutate
                    && (
                        cpu_usage >= CONSTS.auto_mutate_above_cpu_usage
                            || self.average_update_stat.should_mutate()
                        // || dbg!(thread_rng().gen::<usize>() % CONSTS.graph_mutation_divisor) == 0
                    ))
            {
                info!("====TIC: {} MUTATING TREE====", self.current_t);
                if thread_rng().gen_bool(0.5) {
                    info!("MUTATING ROOT NODE");
                    self.node_tree.root_node.mutate_rng(
                        &mut self.rng,
                        MutArg {
                            nodes: &mut self.nodes,
                            data: &mut self.data,
                            depth: 0,
                            current_t,
                            coordinate_set: history_step.update_coordinate,
                            history: &self.history,
                            image_preloader: &mut self.image_preloader,
                            profiler: &mut self.profiler,
                            gamepads: &mut self.gamepads,
                        },
                    );
                } else {
                    if thread_rng().gen_bool(0.5) {
                        info!("MUTATING COORD NODE");
                        self.node_tree.root_coordinate_node.mutate_rng(
                            &mut self.rng,
                            MutArg {
                                nodes: &mut self.nodes,
                                data: &mut self.data,
                                depth: 0,
                                current_t,
                                coordinate_set: history_step.update_coordinate,
                                history: &self.history,
                                image_preloader: &mut self.image_preloader,
                                profiler: &mut self.profiler,
                                gamepads: &mut self.gamepads,
                            },
                        );
                    } else {
                        info!("MUTATING RENDERER");
                        self.node_tree.root_frame_renderer.mutate_rng(
                            &mut self.rng,
                            MutArg {
                                nodes: &mut self.nodes,
                                data: &mut self.data,
                                depth: 0,
                                current_t,
                                coordinate_set: history_step.update_coordinate,
                                history: &self.history,
                                image_preloader: &mut self.image_preloader,
                                profiler: &mut self.profiler,
                                gamepads: &mut self.gamepads,
                            },
                        );
                    }
                }
                // // info!("{:#?}", &self.root_node);
                // if self.record_tree {
                //     self.node_tree.save("latest");
                // }
                self.last_mutation_t = self.current_t;
                self.tree_dirty = false;
            }

            // let last_update_state = UpdateState {
            //     coordinate_set: history_step.update_coordinate,
            //     history: &self.history,
            // };

            self.gamepads.clear_in_use();

            let last_update_arg = UpdArg {
                coordinate_set: history_step.update_coordinate,
                history: &self.history,
                nodes: &mut self.nodes,
                data: &mut self.data,
                depth: 0,
                image_preloader: &mut self.image_preloader,
                profiler: &mut self.profiler,
                gamepads: &mut self.gamepads,
                current_t,
            };

            // dbg!(last_update_arg.coordinate_set);

            self.next_history_step.update_coordinate = self
                .node_tree
                .compute_offset_node
                .compute(last_update_arg.into());

            // dbg!(self.next_history_step.update_coordinate);

            //Workaround, TODO:please fix
            //double TODO: fix this please it could be breaking other stuff
            //triple TODO: please it's important
            self.next_history_step.update_coordinate.t = current_t as f32;

            let mut step_upd_arg = UpdArg {
                coordinate_set: self.next_history_step.update_coordinate,
                history: &self.history,
                nodes: &mut self.nodes,
                data: &mut self.data,
                depth: 0,
                image_preloader: &mut self.image_preloader,
                profiler: &mut self.profiler,
                gamepads: &mut self.gamepads,
                current_t,
            };

            let mut step_com_arg: ComArg = step_upd_arg.reborrow().into();

            self.next_history_step.fade_color = self
                .node_tree
                .fade_color_node
                .compute(step_com_arg.reborrow());
            self.next_history_step.alpha_multiplier = self
                .node_tree
                .fade_color_alpha_multiplier
                .compute(step_com_arg.reborrow());

            self.next_history_step.root_scalar = UNFloat::new(mutation_likelihood.powf(2.0) as f32);

            self.next_history_step.frame_renderer = self
                .node_tree
                .root_frame_renderer
                .compute(step_com_arg.reborrow());

            let use_nearest_neighbour_scaling = self
                .node_tree
                .scaling_mode_node
                .compute(step_com_arg.reborrow())
                .into_inner();

            self.next_history_step.computed_texture = compute_texture(
                ctx,
                self.next_history_step.cell_array.view(),
                use_nearest_neighbour_scaling,
            );

            self.node_tree.update_recursively(step_upd_arg.reborrow());

            for depth in 0..self.nodes.len() {
                let (current, children) = self.nodes[depth..].split_first_mut().unwrap();

                let mut step_upd_arg = UpdArg {
                    coordinate_set: self.next_history_step.update_coordinate,
                    history: &self.history,
                    nodes: children,
                    data: &mut self.data,
                    image_preloader: &mut self.image_preloader,
                    profiler: &mut self.profiler,
                    gamepads: &mut self.gamepads,
                    depth,
                    current_t,
                };

                current.update_recursively(step_upd_arg.reborrow());
            }

            // Rotate the buffers by swapping
            let h_len = self.history.history_steps.len();
            std::mem::swap(
                &mut self.history.history_steps[current_t % h_len],
                &mut self.next_history_step,
            );

            self.ui.draw(&self.average_update_stat, &self.gamepads);
            if let Some(profiler) = &self.profiler {
                profiler
                    .save(MutagenProfiler::default_path())
                    .unwrap_or_else(|e| warn!("Failed to save profiler data: {}", e));
            }

            self.current_t += 1;
            self.cpu_t = next_cpu_t;
        }

        timer::yield_now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        assert!(CONSTS.cell_array_history_length > CONSTS.cell_array_lerp_length);

        if self.last_render_t != timer::ticks(ctx) {
            let lerp_sub =
                (timer::ticks(ctx) % CONSTS.tics_per_update) as f32 / CONSTS.tics_per_update as f32;

            let fresh_frame = timer::ticks(ctx) % CONSTS.tics_per_update == 0;

            for lerp_i in 0..CONSTS.cell_array_lerp_length {
                let args = RenderArgs {
                    ctx,
                    history: &self.history,
                    current_t: self.current_t,
                    lerp_sub,
                    lerp_i,
                    blank_texture: &self.blank_texture,
                    fresh_frame,
                };

                args.history_step().frame_renderer.draw(args)?;
            }

            self.last_render_t = timer::ticks(ctx);
            graphics::present(ctx)?;
        }

        Ok(())
    }
}
