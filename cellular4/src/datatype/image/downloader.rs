use failure::Fallible;
use reqwest::blocking::Client as HttpClient;

use crate::{datatype::image::Image, util::DeterministicRng};

mod smithsonian;
pub use smithsonian::*;

mod lorem_picsum;
pub use lorem_picsum::*;

mod gfycat;
pub use gfycat::*;

pub trait ImageDownloader {
    fn download_image(
        &mut self,
        rng: &mut DeterministicRng,
        http: &mut HttpClient,
    ) -> Fallible<Image>;
}
