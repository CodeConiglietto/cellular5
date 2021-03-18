use std::{
    fmt::{self, Debug, Display, Formatter},
    fs,
    io::Cursor,
    path::{Path, PathBuf},
    sync::Arc,
};

use failure::{format_err, Fallible};
use image::{
    codecs::gif, imageops, imageops::FilterType, AnimationDecoder, ImageFormat, RgbaImage,
};
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use rand::prelude::*;
use reqwest::blocking::Client as HttpClient;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{colors::ByteColor, continuous::*},
    mutagen_args::*,
    preloader::Generator,
    util::{self, DeterministicRng},
};

mod downloader;
use downloader::*;

pub const MODULE_PATH: &str = module_path!();

lazy_static! {
    static ref ALL_IMAGES: Vec<PathBuf> = util::collect_filenames(&CONSTS.image_path);
    static ref FALLBACK_IMAGE: Image =
        Image::load(ImageSource::Fallback, FALLBACK_IMAGE_DATA, None).unwrap_or_else(|e| {
            error!("Error loading fallback image: {}", e);
            panic!()
        });
}

const FALLBACK_IMAGE_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/eyeball2.png"));

pub struct RandomImageLoader {
    rng: DeterministicRng,
    http: HttpClient,
    downloaders: Vec<Box<dyn ImageDownloader + Send>>,
}

impl RandomImageLoader {
    pub fn new() -> Self {
        let mut http = HttpClient::new();

        let mut downloaders: Vec<Box<dyn ImageDownloader + Send>> =
            vec![Box::new(LoremPicsum::new())];

        if let Some(api_key) = &CONSTS.smithsonian_api_key {
            match Smithsonian::new(api_key.clone(), &mut http) {
                Ok(s) => {
                    info!("Initialized Smithsonian API");
                    downloaders.push(Box::new(s));
                }
                Err(e) => error!("Failed to initialize Smithsonian API: {}", e),
            }
        }

        if let Some(config) = &CONSTS.gfycat {
            match Gfycat::new(config, &mut http) {
                Ok(s) => {
                    info!("Initialized Gfycat API");
                    downloaders.push(Box::new(s));
                }
                Err(e) => error!("Failed to initialize Gfycat API: {}", e),
            }
        }

        Self {
            rng: DeterministicRng::new(),
            http,
            downloaders,
        }
    }

    fn download_image(&mut self) -> Fallible<Image> {
        let image = self
            .downloaders
            .choose_mut(&mut self.rng)
            .ok_or_else(|| format_err!("No downloaders available"))?
            .download_image(&mut self.rng, &mut self.http)?;

        debug!("Downloaded image: {}", image.source());

        Ok(image)
    }
}

impl Default for RandomImageLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator for RandomImageLoader {
    type Output = Image;

    fn generate(&mut self) -> Self::Output {
        if self.rng.gen_bool(CONSTS.image_download_probability) {
            self.download_image().unwrap_or_else(|e| {
                warn!("Failed to download image: {}", e);
                load_random_image_file(&mut self.rng)
            })
        } else {
            load_random_image_file(&mut self.rng)
        }
    }
}

fn load_random_image_file<R: Rng + ?Sized>(rng: &mut R) -> Image {
    if let Some(filename) = ALL_IMAGES.choose(rng) {
        debug!("Loading image file '{}'", filename.to_string_lossy());
        Image::load_file(&filename).unwrap_or_else(|e| {
            error!(
                "Failed to load image file '{}': {}",
                filename.to_string_lossy(),
                e
            );
            FALLBACK_IMAGE.clone()
        })
    } else {
        debug!("No images found, loading fallback image");
        FALLBACK_IMAGE.clone()
    }
}

#[derive(Clone)]
pub struct Image(Arc<ImageData>);

pub struct ImageData {
    source: ImageSource,
    frames: Vec<RgbaImage>,
}

impl Image {
    pub fn new(source: ImageSource, frames: Vec<RgbaImage>) -> Self {
        Self(Arc::new(ImageData { source, frames }))
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> image::ImageResult<Self> {
        Ok(Self::new(
            ImageSource::Local(path.as_ref().to_owned()),
            load_frames(&fs::read(&path)?, ImageFormat::from_path(&path).ok())?,
        ))
    }

    pub fn load(
        source: ImageSource,
        data: &[u8],
        format: Option<ImageFormat>,
    ) -> image::ImageResult<Self> {
        Ok(Self::new(source, load_frames(data, format)?))
    }

    pub fn get_pixel_wrapped(&self, x: u32, y: u32, t: u32) -> ByteColor {
        let frame_count = self.0.frames.len();
        let t_value = ((t as usize % frame_count) + frame_count) % frame_count;

        let image_width = self.0.frames[t_value].width();
        let image_height = self.0.frames[t_value].height();

        //TODO refactor into helper method
        (*self.0.frames[t_value].get_pixel(
            ((x % image_width) + image_width) % image_width,
            ((y % image_height) + image_height) % image_height,
        ))
        .into()
    }

    //get a pixel from coords (-1.0..1.0, -1.0..1.0, 0.0..infinity)
    pub fn get_pixel_normalised(&self, x: SNFloat, y: SNFloat, t: f32) -> ByteColor {
        let frame_count = self.0.frames.len();
        let t_value = ((t as usize % frame_count) + frame_count) % frame_count;

        let image_width = self.0.frames[t_value].width() as f32;
        let image_height = self.0.frames[t_value].height() as f32;

        self.get_pixel_wrapped(
            (x.to_unsigned().into_inner() * image_width) as u32,
            (y.to_unsigned().into_inner() * image_height) as u32,
            t_value as u32,
        )
    }

    pub fn source(&self) -> &ImageSource {
        &self.0.source
    }

    pub fn info(&self) -> ImageInfo {
        ImageInfo {
            source: self.0.source.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ImageSource {
    Fallback,
    Local(PathBuf),
    Other(String),
}

impl Display for ImageSource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ImageSource::Fallback => write!(f, "Fallback"),
            ImageSource::Local(p) => write!(f, "{}", p.to_string_lossy()),
            ImageSource::Other(s) => write!(f, "{}", s),
        }
    }
}

fn load_frames(data: &[u8], format: Option<ImageFormat>) -> image::ImageResult<Vec<RgbaImage>> {
    // Special handling for gifs in case they are animated
    match format {
        Some(ImageFormat::Gif) => Ok(gif::GifDecoder::new(Cursor::new(data))?
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(|f| {
                imageops::resize(
                    &f.into_buffer(),
                    CONSTS.cell_array_width as u32,
                    CONSTS.cell_array_height as u32,
                    FilterType::Gaussian,
                )
            })
            .collect()),

        Some(format) => Ok(vec![imageops::resize(
            &image::load_from_memory_with_format(data, format)?.to_rgba8(),
            CONSTS.cell_array_width as u32,
            CONSTS.cell_array_height as u32,
            FilterType::Gaussian,
        )]),

        None => Ok(vec![imageops::resize(
            &image::load_from_memory(data)?.to_rgba8(),
            CONSTS.cell_array_width as u32,
            CONSTS.cell_array_height as u32,
            FilterType::Gaussian,
        )]),
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Image")
            .field("source", &self.0.source)
            .field("frames", &self.0.frames.len())
            .finish()
    }
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.info().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ImageInfo::deserialize(deserializer)?
            .try_load()
            .unwrap_or_else(|_| FALLBACK_IMAGE.clone()))
    }
}

impl<'a> Generatable<'a> for Image {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R, arg: GenArg<'a>) -> Self {
        arg.image_preloader.try_get_next().unwrap_or_else(|| {
            debug!("Preloader has no image ready, loading fallback");
            FALLBACK_IMAGE.clone()
        })
    }
}

impl<'a> Mutatable<'a> for Image {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, _rng: &mut R, arg: Self::MutArg) {
        *self = arg
            .image_preloader
            .try_get_next()
            .unwrap_or_else(|| FALLBACK_IMAGE.clone());
    }
}

impl<'a> Updatable<'a> for Image {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for Image {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageInfo {
    source: ImageSource,
}

impl ImageInfo {
    pub fn try_load(&self) -> Fallible<Image> {
        match &self.source {
            ImageSource::Fallback => Ok(FALLBACK_IMAGE.clone()),
            ImageSource::Local(p) => Ok(Image::load_file(p)?),
            ImageSource::Other(_) => {
                Err(format_err!("Cannot load from source: {:?}", &self.source))
            }
        }
    }
}
