use failure::{format_err, Fallible};
use image::ImageFormat;
use log::debug;
use reqwest::blocking::Client as HttpClient;

use crate::{
    constants::CONSTS,
    datatype::image::{downloader::ImageDownloader, Image, ImageSource},
    util::DeterministicRng,
};

pub struct LoremPicsum;

impl LoremPicsum {
    pub fn new() -> Self {
        LoremPicsum
    }
}

impl ImageDownloader for LoremPicsum {
    fn download_image(
        &mut self,
        _rng: &mut DeterministicRng,
        client: &mut HttpClient,
    ) -> Fallible<Image> {
        let mut buf = Vec::new();

        let mut response = client
            .get(&format!(
                "https://picsum.photos/{}/{}",
                CONSTS.initial_window_width.floor() as usize,
                CONSTS.initial_window_height.floor() as usize,
            ))
            .send()?
            .error_for_status()?;

        response.copy_to(&mut buf)?;

        let url = response.url();
        let filename = url
            .path_segments()
            .ok_or_else(|| format_err!("Couldn't parse url: {}", url.as_str()))?
            .last()
            .ok_or_else(|| format_err!("Empty url: {}", url.as_str()))?;

        let name = format!("{} (Lorem Picsum)", &filename);
        let format = ImageFormat::from_path(&filename).ok();

        debug!("Downloaded image: {}", name);

        Ok(Image::load(ImageSource::Other(name), &buf, format)?)
    }
}
