use std::fmt::{self, Display, Formatter};

use failure::{format_err, Fallible};
use image::ImageFormat;
use log::{debug, warn};
use rand::prelude::*;
use reqwest::{blocking::Client as HttpClient, StatusCode};
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};

use crate::{
    constants::GfycatConfig,
    datatype::image::{downloader::ImageDownloader, Image, ImageSource},
    util::DeterministicRng,
};

mod endpoints {
    pub const TOKEN: &str = "https://api.gfycat.com/v1/oauth/token";
    pub const TRENDING: &str = "https://api.gfycat.com/v1/gfycats/trending";
    pub const TRENDING_TAGS: &str = "https://api.gfycat.com/v1/tags/trending";
    pub const SEARCH: &str = "https://api.gfycat.com/v1/gfycats/search";
}

pub struct Gfycat {
    config: GfycatConfig,
    token: Token,
    tags: Vec<String>,
}

impl Gfycat {
    pub fn new(config: &GfycatConfig, http: &mut HttpClient) -> Fallible<Self> {
        let token = auth(&config.client_id, &config.client_secret, http)?;
        let tags = if config.trending {
            http.get(endpoints::TRENDING_TAGS)
                .query(&[("tagCount", "100")])
                .bearer_auth(&token.access_token)
                .send()?
                .error_for_status()?
                .json()?
        } else {
            Vec::new()
        };

        Ok(Self {
            config: config.clone(),
            token,
            tags,
        })
    }

    fn download_from_trending_tag(
        &mut self,
        rng: &mut DeterministicRng,
        http: &mut HttpClient,
    ) -> Fallible<Option<Vec<GfycatEntry>>> {
        let idx = rng.gen_range(0, self.tags.len());
        let tag = &self.tags[idx];

        debug!("Querying tag {}", tag);

        let request = http
            .get(endpoints::TRENDING)
            .query(&[("tagName", tag.as_str()), ("count", "100")])
            .bearer_auth(&self.token.access_token);

        let mut response = request
            .try_clone()
            .ok_or_else(|| format_err!("Failed to clone request"))?
            .send()?;

        if response.status() == StatusCode::UNAUTHORIZED {
            debug!("Unauthorized response, reauthenticating");
            self.token = auth(&self.config.client_id, &self.config.client_secret, http)?;
            response = request.send()?;
        }

        let response: TrendingResponse = response.error_for_status()?.json()?;
        let mut items = response.gfycats;
        items.retain(|e| e.published && !e.nsfw);

        if items.is_empty() {
            warn!("Discarding tag {}: no images found", tag);
            self.tags.remove(idx);
            Ok(None)
        } else {
            debug!("Found {} images for tag {}", items.len(), tag);
            Ok(Some(items))
        }
    }

    fn download_from_search(
        &mut self,
        rng: &mut DeterministicRng,
        http: &mut HttpClient,
    ) -> Fallible<Option<Vec<GfycatEntry>>> {
        let idx = rng.gen_range(0, self.config.search_terms.len());
        let term = &self.config.search_terms[idx];

        debug!("Querying search term {}", term);

        let request = http
            .get(endpoints::SEARCH)
            .query(&[("search_text", term.as_str()), ("count", "100")])
            .bearer_auth(&self.token.access_token);

        let mut response = request
            .try_clone()
            .ok_or_else(|| format_err!("Failed to clone request"))?
            .send()?;

        if response.status() == StatusCode::UNAUTHORIZED {
            debug!("Unauthorized response, reauthenticating");
            self.token = auth(&self.config.client_id, &self.config.client_secret, http)?;
            response = request.send()?;
        }

        let response: SearchResponse = response.error_for_status()?.json()?;
        let mut items = response.gfycats;
        items.retain(|e| e.published && !e.nsfw);

        if items.is_empty() {
            warn!("Discarding search term {}: no images found", term);
            self.config.search_terms.remove(idx);
            Ok(None)
        } else {
            debug!("Found {} images for search term {}", items.len(), term);
            Ok(Some(items))
        }
    }
}

fn auth(client_id: &str, client_secret: &str, http: &mut HttpClient) -> Fallible<Token> {
    Ok(http
        .post(endpoints::TOKEN)
        .json(&TokenRequest {
            grant_type: String::from("client_credentials"),
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
        })
        .send()?
        .error_for_status()?
        .json()?)
}

impl ImageDownloader for Gfycat {
    fn download_image(
        &mut self,
        rng: &mut DeterministicRng,
        http: &mut HttpClient,
    ) -> Fallible<Image> {
        let items = loop {
            let list = match (self.tags.is_empty(), self.config.search_terms.is_empty()) {
                (false, false) => {
                    if rng.gen_bool(0.5) {
                        self.download_from_search(rng, http)?
                    } else {
                        self.download_from_trending_tag(rng, http)?
                    }
                }

                (true, false) => self.download_from_search(rng, http)?,
                (false, true) => self.download_from_trending_tag(rng, http)?,

                (true, true) => return Err(format_err!("No tags or search terms available")),
            };

            if let Some(list) = list {
                break list;
            }
        };

        let entry = items
            .choose(rng)
            .ok_or_else(|| format_err!("No results returned"))?;

        let mut buf = Vec::new();
        let mut response = http
            .get(&entry.max_2_mb_gif)
            .bearer_auth(&self.token.access_token)
            .send()?;

        response.copy_to(&mut buf)?;

        if !response.status().is_success() {
            let error: Result<ErrorResponse, _> = serde_json::from_slice(&buf);

            return match error {
                Ok(error) => Err(format_err!("{}", error)),
                Err(e) => Err(format_err!("{} ({})", String::from_utf8_lossy(&buf), e)),
            };
        }

        let name = format!("{} (Gfycat)", &entry.title);

        Ok(Image::load(
            ImageSource::Other(name),
            &buf,
            Some(ImageFormat::GIF),
        )?)
    }
}

#[derive(Serialize)]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Token {
    token_type: String,
    expires_in: u64,
    access_token: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct TrendingResponse {
    tag: String,
    cursor: String,
    gfycats: Vec<GfycatEntry>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SearchResponse {
    found: u64,
    cursor: String,
    gfycats: Vec<GfycatEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct GfycatEntry {
    gfy_id: String,
    gif_url: String,
    #[serde(rename = "max1mbGif")]
    max_1_mb_gif: String,
    #[serde(rename = "max2mbGif")]
    max_2_mb_gif: String,
    title: String,
    #[serde(deserialize_with = "bool_from_int")]
    published: bool,
    #[serde(deserialize_with = "bool_from_int")]
    nsfw: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    error_message: ErrorMessage,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.error_message.code, self.error_message.description
        )
    }
}

#[derive(Deserialize)]
struct ErrorMessage {
    code: String,
    description: String,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BoolFromIntVisitor)
}

struct BoolFromIntVisitor;

impl<'de> Visitor<'de> for BoolFromIntVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or string")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != "0")
    }
}
