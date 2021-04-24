use std::{iter::IntoIterator, ops::Deref};

use failure::{ensure, format_err, Fallible};
use float_ord::FloatOrd;
use log::info;
use ndarray::{prelude::*, Zip};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct CameraConfig {
    device_path: Option<String>,
    n_frames: Option<usize>,
}

pub struct Camera {
    camera: rscam::Camera,
    frames: Array3<ByteColor>,
    skip_ratio: usize,
    current_frame: usize,
    skipped_updates: usize,
}

impl GenericCamera for Camera {
    type Config = CameraConfig;

    fn new(config: Self::Config) -> Fallible<Self> {
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

        let camera_fps = interval.1 as f32 / interval.0 as f32;
        let skip_ratio = (CONSTS.target_fps as f32 / camera_fps).ceil() as usize;

        info!(
            "Initializing camera with format {}, resolution {}x{}, {} fps",
            String::from_utf8_lossy(format.to_code()),
            resolution.0,
            resolution.1,
            camera_fps,
        );

        camera.start(&rscam::Config {
            format: format.to_code(),
            resolution,
            interval,
            ..Default::default()
        })?;

        Ok(Self {
            camera,
            frames: Array3::default((
                config.n_frames.unwrap_or(CONSTS.cell_array_history_length),
                resolution.1 as usize,
                resolution.0 as usize,
            )),
            skip_ratio,
            current_frame: 0,
            skipped_updates: 0,
        })
    }

    fn update(&mut self) -> Fallible<()> {
        if self.skipped_updates < self.skip_ratio - 1 {
            self.skipped_updates += 1;
            return Ok(());
        }

        self.skipped_updates = 0;

        let frame = self.camera.capture()?;
        let format = ImageFormats::try_from_code(&frame.format).unwrap();

        ensure!(
            frame.resolution.1 as usize == self.frames.len_of(Axis(1))
                && frame.resolution.0 as usize == self.frames.len_of(Axis(2)),
            "Mismatched resolution"
        );

        self.current_frame = (self.current_frame + 1) % self.frames.len_of(Axis(0));

        format.decode(
            frame.deref(),
            self.frames.slice_mut(s![self.current_frame, .., ..]),
        );

        Ok(())
    }

    fn get(&self, pos: SNPoint, t: usize) -> ByteColor {
        let h = self.frames.len_of(Axis(1));
        let w = self.frames.len_of(Axis(2));

        self.frames[[
            ((t - self.skipped_updates) / self.skip_ratio) % self.frames.len_of(Axis(0)),
            ((pos.y().to_unsigned().into_inner() * h as f32).round() as usize).min(h - 1),
            ((pos.x().to_unsigned().into_inner() * w as f32).round() as usize).min(w - 1),
        ]]
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
