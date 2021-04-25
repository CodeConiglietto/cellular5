use std::{
    iter::IntoIterator,
    ops::Deref,
    sync::{
        atomic::{self, AtomicBool},
        mpsc::{self, Receiver, SyncSender, TryRecvError},
        Arc,
    },
    thread::{self, JoinHandle},
};

use failure::{ensure, format_err, Fallible};
use float_ord::FloatOrd;
use log::{info, warn};
use ndarray::{prelude::*, Zip};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct CameraConfig {
    device_path: Option<String>,
    n_frames: Option<usize>,
}

pub struct Camera {
    empty_frames_sender: SyncSender<Array2<ByteColor>>,
    ready_frames_receiver: Receiver<Array2<ByteColor>>,
    worker_thread: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl GenericCamera for Camera {
    type Config = CameraConfig;

    fn new(config: Self::Config) -> Fallible<(Self, CameraFrames)> {
        let mut camera = rscam::Camera::new(
            config
                .device_path
                .as_ref()
                .map(String::as_str)
                .unwrap_or("/dev/video0"),
        )?;

        let formats: Vec<_> = camera
            .formats()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter_map(|f| ImageFormats::try_from_code(&f.format))
            .collect();

        ensure!(!formats.is_empty(), "No supported formats");

        let (format, resolution) = formats
            .into_iter()
            .map(|format| Ok((format, camera.resolutions(format.to_code())?)))
            .collect::<Fallible<Vec<_>>>()?
            .into_iter()
            .flat_map(|(format, info)| {
                match info {
                    rscam::ResolutionInfo::Discretes(resolutions) => {
                        get_best_resolution(resolutions.iter().copied())
                    }

                    rscam::ResolutionInfo::Stepwise { min, max, step } => get_best_resolution(
                        (min.0..=max.0).step_by(step.0 as usize).flat_map(|width| {
                            (min.1..=max.1)
                                .step_by(step.1 as usize)
                                .map(move |height| (width, height))
                        }),
                    ),
                }
                .map(|r| (format, r))
            })
            .max_by_key(|(_format, (width, height))| width * height)
            .ok_or_else(|| format_err!("No supported resolution"))?;

        let interval = match camera.intervals(format.to_code(), resolution)? {
            rscam::IntervalInfo::Discretes(intervals) => {
                get_best_interval(intervals.iter().copied())
            }

            rscam::IntervalInfo::Stepwise { min, max, step } => {
                get_best_interval((min.0..=max.0).step_by(step.0 as usize).flat_map(|n| {
                    (min.1..=max.1)
                        .step_by(step.1 as usize)
                        .map(move |d| (n, d))
                }))
            }
        }
        .ok_or_else(|| format_err!("No supported framerate"))?;

        let fps = interval.1 as f32 / interval.0 as f32;

        info!(
            "Initializing camera with format {}, resolution {}x{}, {} fps",
            String::from_utf8_lossy(format.to_code()),
            resolution.0,
            resolution.1,
            fps,
        );

        camera.start(&rscam::Config {
            format: format.to_code(),
            resolution,
            interval,
            ..Default::default()
        })?;

        let n_frames = config.n_frames.unwrap_or(CONSTS.cell_array_history_length);

        let running = Arc::new(AtomicBool::new(true));
        let worker_running = Arc::clone(&running);

        let (ready_frames_sender, ready_frames_receiver) = mpsc::sync_channel(n_frames);
        let (empty_frames_sender, empty_frames_receiver) = mpsc::sync_channel(n_frames);

        for _ in 0..n_frames {
            empty_frames_sender
                .send(Array2::default((
                    resolution.1 as usize,
                    resolution.0 as usize,
                )))
                .unwrap();
        }

        let worker_thread = thread::spawn(move || loop {
            if !worker_running.load(atomic::Ordering::Relaxed) {
                break;
            }

            let frame = match camera.capture() {
                Ok(f) => f,
                Err(e) => {
                    warn!("Error capturing frame: {}", e);
                    break;
                }
            };

            let format = ImageFormats::try_from_code(&frame.format).unwrap();

            let mut next_frame_buf = empty_frames_receiver.recv().unwrap();

            assert_eq!(frame.resolution.1 as usize, next_frame_buf.len_of(Axis(0)));
            assert_eq!(frame.resolution.0 as usize, next_frame_buf.len_of(Axis(1)));

            format.decode(frame.deref(), next_frame_buf.view_mut());

            ready_frames_sender.send(next_frame_buf).unwrap();
        });

        Ok((
            Self {
                ready_frames_receiver,
                empty_frames_sender,
                worker_thread: Some(worker_thread),
                running,
            },
            CameraFrames {
                frames: (0..n_frames)
                    .map(|_| Array2::default((resolution.1 as usize, resolution.0 as usize)))
                    .collect(),
                fps,
                resolution,
                current_t: 0,
            },
        ))
    }

    fn update(&mut self, frames: &mut CameraFrames, current_t: usize) -> Fallible<()> {
        loop {
            match self.ready_frames_receiver.try_recv() {
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Worker thread lost"),

                Ok(frame) => {
                    frames.frames.push_front(frame);
                    self.empty_frames_sender
                        .send(frames.frames.pop_back().unwrap())
                        .unwrap();
                }
            }
        }

        frames.current_t = current_t;

        Ok(())
    }
}

fn get_best_resolution<I>(it: I) -> Option<(u32, u32)>
where
    I: IntoIterator<Item = (u32, u32)> + Clone,
{
    it.clone()
        .into_iter()
        .filter(|(width, height)| {
            *width as usize >= CONSTS.cell_array_width
                && *height as usize >= CONSTS.cell_array_height
        })
        .min_by_key(|(width, height)| width * height)
        .or_else(|| it.into_iter().max_by_key(|(width, height)| width * height))
}

fn get_best_interval<I>(it: I) -> Option<(u32, u32)>
where
    I: IntoIterator<Item = (u32, u32)> + Clone,
{
    it.clone()
        .into_iter()
        .filter(|(n, d)| (*d as f32 / *n as f32) >= CONSTS.target_fps as f32)
        .min_by_key(|(n, d)| FloatOrd(*d as f32 / *n as f32))
        .or_else(|| {
            it.into_iter()
                .max_by_key(|(n, d)| FloatOrd(*d as f32 / *n as f32))
        })
}

impl Drop for Camera {
    fn drop(&mut self) {
        if let Some(handle) = self.worker_thread.take() {
            self.running.store(false, atomic::Ordering::Relaxed);
            handle.join().unwrap();
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ImageFormats {
    Yuyv,
}

impl ImageFormats {
    fn try_from_code(format: &[u8]) -> Option<Self> {
        match format {
            b"YUYV" => Some(ImageFormats::Yuyv),
            _ => None,
        }
    }

    fn to_code(&self) -> &'static [u8] {
        match self {
            ImageFormats::Yuyv => b"YUYV",
        }
    }

    fn decode(&self, data: &[u8], out: ArrayViewMut2<ByteColor>) {
        match self {
            ImageFormats::Yuyv => {
                let yuv_data =
                    ArrayView::from_shape((out.shape()[0], out.shape()[1] / 2, 2, 2), data)
                        .unwrap();

                let out_shape = (out.shape()[0], out.shape()[1] / 2, 2);

                let y_data = yuv_data.slice(s![.., .., .., 0]);
                let u_data = yuv_data.slice(s![.., .., 0, 1, NewAxis]);
                let v_data = yuv_data.slice(s![.., .., 1, 1, NewAxis]);

                Zip::from(y_data)
                    .and_broadcast(u_data)
                    .and_broadcast(v_data)
                    .map_assign_into(out.into_shape(out_shape).unwrap(), |y, u, v| {
                        let u = u.saturating_sub(127);
                        let v = v.saturating_sub(127);

                        let r = *y as f32 + 1.4075 * v as f32;
                        let g = *y as f32 - 0.3455 * u as f32 - 0.7169 * v as f32;
                        let b = *y as f32 + 1.779 * u as f32;

                        ByteColor {
                            r: Byte::new(num::clamp(r, 0.0, 255.0).round() as u8),
                            g: Byte::new(num::clamp(g, 0.0, 255.0).round() as u8),
                            b: Byte::new(num::clamp(b, 0.0, 255.0).round() as u8),
                            a: Byte::new(255),
                        }
                    });
            }
        }
    }
}
