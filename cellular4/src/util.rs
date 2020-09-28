use ggez::{graphics::Image as GgImage, graphics::Drawable, Context};

use std::{
    path::{Path, PathBuf},
    sync::Mutex,
    time::SystemTime,
};

use lazy_static::lazy_static;
use log::debug;
use nalgebra::*;
use ndarray::{Array3, ArrayView3};
use rand::{RngCore, SeedableRng};
use walkdir::WalkDir;

pub fn collect_filenames<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut vec: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| {
            e.ok().and_then(|e| {
                if e.file_type().is_file() {
                    Some(e.path().to_owned())
                } else {
                    None
                }
            })
        })
        .collect();

    vec.sort();

    vec
}

lazy_static! {
    pub static ref RNG_SEED: Mutex<u128> =
        Mutex::new(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis());
}

#[cfg(target_pointer_width = "64")]
type DeterministicRngImpl = rand_pcg::Pcg64Mcg;

#[cfg(target_pointer_width = "32")]
type DeterministicRngImpl = rand_pcg::Pcg32;

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Only 32 and 64 bit systems supported");

pub struct DeterministicRng {
    rng: DeterministicRngImpl,
}

impl Default for DeterministicRng {
    fn default() -> Self {
        Self::new()
    }
}

impl RngCore for DeterministicRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}

impl SeedableRng for DeterministicRng {
    type Seed = <DeterministicRngImpl as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            rng: DeterministicRngImpl::from_seed(seed),
        }
    }
}

impl DeterministicRng {
    pub fn new() -> Self {
        let seed = *RNG_SEED.lock().unwrap();
        debug!("Initializing RNG with seed {}", seed);
        Self::from_seed(seed.to_le_bytes())
    }
}

#[inline(always)]
pub fn map_range(value: f32, from: (f32, f32), to: (f32, f32)) -> f32 {
    let (from_min, from_max) = from;
    let (to_min, to_max) = to;

    assert!(
        from_min < from_max,
        "Invalid range argument to map_range: from_min: {}, from_max: {}",
        from_min,
        from_max
    );
    assert!(
        from_min <= value && value <= from_max,
        "Invalid value argument to map_range: from_min: {}, from_max: {} value: {}",
        from_min,
        from_max,
        value
    );
    assert!(
        to_min < to_max,
        "Invalid range argument to map_range: to_min: {}, to_max: {}",
        to_min,
        to_max
    );

    let out = ((value - from_min) / (from_max - from_min)) * (to_max - to_min) + to_min;

    debug_assert!(
        to_min <= out && out <= to_max,
        "Internal error in map_range: value: {}, from: {:?}, to: {:?}, out: {:?}",
        value,
        from,
        to,
        out
    );

    out
}

#[inline(always)]
pub fn escape_time_system<I, E>(
    mut c: Complex<f64>,
    max_iterations: usize,
    mut iteration: I,
    mut escape: E,
) -> (Complex<f64>, usize)
where
    I: FnMut(Complex<f64>, usize) -> Complex<f64>,
    E: FnMut(Complex<f64>, usize) -> bool,
{
    for i in 0..max_iterations {
        if escape(c, i) {
            return (c, i);
        }
        c = iteration(c, i);
    }

    (c, max_iterations)
}

pub fn init_cell_array(width: usize, height: usize) -> Array3<u8> {
    Array3::from_shape_fn((height, width, 4), |(_y, _x, c)| {
        if c == 3 {
            255
        } else {
            0
            // thread_rng().gen::<u8>()
        }
    })
}

pub fn compute_texture(ctx: &mut Context, cell_array: ArrayView3<u8>) -> GgImage {
    let (height, width, _) = cell_array.dim();
    let mut image = GgImage::from_rgba8(
        ctx,
        width as u16,
        height as u16,
        cell_array.as_slice().unwrap(),
    )
    .unwrap();

    // image.set_blend_mode(Some(ggez::graphics::BlendMode::Multiply));
    // image.set_filter(ggez::graphics::FilterMode::Nearest);
    image
}

pub fn lerp(a: f32, b: f32, value: f32) -> f32 {
    a + (b - a) * value
}
