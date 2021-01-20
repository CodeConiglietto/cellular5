use failure::{format_err, Fallible};
use image::ImageFormat;
use log::debug;
use rand::prelude::*;
use reqwest::{blocking::Client as HttpClient, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    datatype::image::{downloader::ImageDownloader, Image, ImageSource},
    util::DeterministicRng,
};

mod endpoints {
    pub const TOKEN: &str = "https://api.gfycat.com/v1/oauth/token";
    pub const TRENDING: &str = "https://api.gfycat.com/v1/reactions/populated";
}

pub struct Gfycat {
    client_id: String,
    client_secret: String,
    token: Token,
}

impl Gfycat {
    pub fn new(client_id: String, client_secret: String, http: &mut HttpClient) -> Fallible<Self> {
        let token = auth(&client_id, &client_secret, http)?;

        Ok(Self {
            client_id,
            client_secret,
            token,
        })
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
        let mut response = http
            .get(endpoints::TRENDING)
            .query(&[("tagName", "trending")])
            .bearer_auth(&self.token.access_token)
            .send()?;

        if response.status() == StatusCode::UNAUTHORIZED {
            debug!("Unauthorized response, reauthenticating");
            self.token = auth(&self.client_id, &self.client_secret, http)?;

            response = http
                .get(endpoints::TRENDING)
                .query(&[("tagName", "trending")])
                .bearer_auth(&self.token.access_token)
                .send()?;
        }

        let response: TrendingResponse = response.error_for_status()?.json()?;

        let entry = response
            .gfycats
            .choose(rng)
            .ok_or_else(|| format_err!("No results returned"))?;

        let mut buf = Vec::new();
        let mut response = http.get(&entry.gif_url).send()?.error_for_status()?;
        response.copy_to(&mut buf)?;

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
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct GfycatEntry {
    gfy_id: String,
    gif_url: String,
    title: String,
}
