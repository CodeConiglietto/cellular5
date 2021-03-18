use std::collections::{hash_map, HashMap};

use failure::{format_err, Fallible};
use image::ImageFormat;
use log::{debug, warn};
use rand::prelude::*;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;

use crate::{
    datatype::image::{downloader::ImageDownloader, Image, ImageSource},
    util::DeterministicRng,
};

pub struct Smithsonian {
    api_key: String,
    topics: Vec<String>,
    topic_items: HashMap<String, Vec<MediaItem>>,
}

mod endpoints {
    pub const TOPICS: &str = "https://api.si.edu/openaccess/api/v1.0/terms/topic";
    pub const SEARCH: &str = "https://api.si.edu/openaccess/api/v1.0/search";
}

impl Smithsonian {
    pub fn new(api_key: String, client: &mut HttpClient) -> Fallible<Self> {
        debug!("Querying list of topics");

        let topics: Response<Terms> = client
            .get(endpoints::TOPICS)
            .query(&[("api_key", &api_key)])
            .send()?
            .error_for_status()?
            .json()?;

        let topics = topics.response.terms;

        debug!("Obtained list of {} topics", topics.len());

        Ok(Self {
            api_key,
            topics,
            topic_items: HashMap::new(),
        })
    }
}

impl ImageDownloader for Smithsonian {
    fn download_image(
        &mut self,
        rng: &mut DeterministicRng,
        client: &mut HttpClient,
    ) -> Fallible<Image> {
        let items = loop {
            if self.topics.is_empty() {
                return Err(format_err!("No topics available"));
            }

            let topic_idx = rng.gen_range(0, self.topics.len());
            let topic = &self.topics[topic_idx];

            match self.topic_items.entry(String::from(topic)) {
                hash_map::Entry::Vacant(e) => {
                    debug!("Querying topic {}", topic);

                    let search: Response<Search> = client
                        .get(endpoints::SEARCH)
                        .query(&[
                            ("api_key", self.api_key.as_str()),
                            ("rows", "1000"),
                            (
                                "q",
                                format!("online_media_type:Images AND topic:\"{}\"", topic)
                                    .as_str(),
                            ),
                        ])
                        .send()?
                        .error_for_status()?
                        .json()?;

                    let mut topic_items = Vec::new();

                    for row in search.response.rows {
                        let title = row.content.descriptive_non_repeating.title.content;

                        if let Some(online_media) =
                            row.content.descriptive_non_repeating.online_media
                        {
                            for media in online_media.media {
                                if media._type == "Images" {
                                    const FORMATS: &[ImageFormat] =
                                        &[ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::Gif];

                                    let found_resource =
                                        media.resources.into_iter().find_map(|resource| {
                                            ImageFormat::from_path(&resource.url)
                                                .ok()
                                                .filter(|format| FORMATS.contains(&format))
                                                .map(|format| (resource, format))
                                        });

                                    let item = if let Some((resource, format)) = found_resource {
                                        MediaItem {
                                            url: resource.url,
                                            title: title.clone(),
                                            format: Some(format),
                                        }
                                    } else {
                                        MediaItem {
                                            url: media.content,
                                            title: title.clone(),
                                            format: None,
                                        }
                                    };

                                    topic_items.push(item);
                                }
                            }
                        }
                    }

                    if topic_items.is_empty() {
                        warn!("Discarding topic {}: no images found", topic);
                        self.topics.remove(topic_idx);
                    } else {
                        debug!("Found {} images on topic {}", topic_items.len(), topic);
                        break &*e.insert(topic_items);
                    }
                }

                hash_map::Entry::Occupied(e) => break &*e.into_mut(),
            }
        };

        let item = items.choose(rng).expect("Empty items list");

        let mut buf = Vec::new();
        let mut response = client.get(&item.url).send()?.error_for_status()?;
        response.copy_to(&mut buf)?;

        let name = format!("{} (Smithsonian)", &item.title);

        Ok(Image::load(ImageSource::Other(name), &buf, item.format)?)
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Response<T> {
    status: u16,
    #[serde(rename = "responseCode")]
    response_code: u16,
    response: T,
}

#[derive(Deserialize)]
struct Terms {
    terms: Vec<String>,
}

#[derive(Deserialize)]
struct Search {
    #[serde(default)]
    rows: Vec<Row>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Row {
    id: String,
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    #[serde(rename = "descriptiveNonRepeating")]
    descriptive_non_repeating: Dnr,
}

#[derive(Deserialize)]
struct Dnr {
    online_media: Option<OnlineMedia>,
    title: Title,
}

#[derive(Deserialize)]
struct OnlineMedia {
    #[serde(default)]
    media: Vec<Media>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Media {
    #[serde(rename = "type")]
    pub _type: String,
    pub content: String,
    #[serde(default)]
    pub resources: Vec<Resource>,
}

#[derive(Deserialize)]
struct Resource {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize)]
struct Title {
    content: String,
}

struct MediaItem {
    url: String,
    title: String,
    format: Option<ImageFormat>,
}
