use std::fmt::{self, Debug, Display, Formatter};

use chrono::{serde::ts_seconds, DateTime, Utc};
use failure::Fallible;
use reqwest::{
    blocking::{Client as HttpClient, RequestBuilder},
    Method,
};
use serde::{
    de::{Deserializer, Error},
    Deserialize, Serialize,
};
use url::Url;

const USER_AGENT: &str = "cellular";

pub struct Endpoint {
    method: Method,
    url: Url,
}

mod endpoints {
    use super::*;

    fn root_url() -> Url {
        as_url("https://www.shadertoy.com")
    }

    fn root_api_url() -> Url {
        join_url(&root_url(), "api/v1")
    }

    fn api_url(path: &str) -> Url {
        root_api_url()
            .join(path)
            .unwrap_or_else(|e| panic!("Invalid endpoint url: {} ({})", path, e))
    }

    pub fn list() -> Endpoint {
        endpoint(Method::GET, "shaders")
    }

    pub fn search(query: &str) -> Endpoint {
        endpoint(Method::GET, &format!("shaders/query/{}", query)) // TODO Check if this encodes correctly?
    }

    pub fn get_shader(id: &str) -> Endpoint {
        endpoint(Method::GET, &format!("shaders/{}", id)) // TODO Check if this encodes correctly?
    }

    fn endpoint(method: Method, path: &str) -> Endpoint {
        Endpoint {
            method,
            url: api_url(path),
        }
    }
}

mod headers {
    pub use reqwest::header::USER_AGENT;
}

pub struct Client {
    http: HttpClient,
    api_key: Redacted<String>,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            http: HttpClient::new(),
            api_key: Redacted(api_key),
        }
    }

    fn raw_request(&self, endpoint: Endpoint) -> RequestBuilder {
        self.http
            .request(endpoint.method, endpoint.url)
            .header(headers::USER_AGENT, USER_AGENT)
    }

    fn authed_request(&self, endpoint: Endpoint) -> RequestBuilder {
        self.raw_request(endpoint).query(&["key", &self.api_key.0])
    }

    pub fn list(&self) -> Fallible<Vec<String>> {
        Ok(self
            .authed_request(endpoints::list())
            .send()?
            .error_for_status()?
            .json::<ShaderListResponse>()?
            .results)
    }

    pub fn search(
        &self,
        query: &str,
        sort: Option<Sort>,
        filter: Option<Filter>,
    ) -> Fallible<Vec<String>> {
        let mut req = self.authed_request(endpoints::search(query));

        if let Some(sort) = sort {
            req = req.query(&[("sort", sort)]);
        }

        if let Some(filter) = filter {
            req = req.query(&[("filter", filter)]);
        }

        Ok(req
            .send()?
            .error_for_status()?
            .json::<ShaderListResponse>()?
            .results)
    }

    pub fn get_shader(&self, id: &str) -> Fallible<Shader> {
        Ok(self
            .authed_request(endpoints::get_shader(id))
            .send()?
            .error_for_status()?
            .json::<ShaderResponse>()?
            .shader)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Name,
    Love,
    Popular,
    Newest,
    Hot,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Filter {
    Vr,
    SoundOutput,
    SoundInput,
    Webcam,
    MultiPass,
    MusicStream,
}

#[derive(Deserialize)]
pub struct Shader {
    #[serde(rename = "ver")]
    pub version: String,
    pub info: ShaderInfo,
    #[serde(rename = "renderpass")]
    pub passes: Vec<RenderPass>,
}

#[derive(Deserialize)]
pub struct ShaderInfo {
    pub id: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub viewed: u16,
    pub name: String,
    pub username: String,
    pub description: String,
    pub likes: u16,
    pub published: u16,
    pub flags: u32,
    pub tags: Vec<String>,
    pub hasliked: u16,
}

#[derive(Deserialize)]
pub struct RenderPass {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub code: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize)]
pub struct Input {
    pub id: u16,
    pub src: String,
    pub ctype: CType,
    pub channel: u8,
    pub sampler: Sampler,
    #[serde(deserialize_with = "bool_as_0_1")]
    pub published: bool,
}

#[derive(Deserialize)]
pub struct Output {
    pub id: u16,
    pub channel: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CType {
    Buffer,
    CubeMap,
    Keyboard,
    Music,
    MusicStream,
    Texture,
    Video,
    Webcam,
}

#[derive(Deserialize)]
pub struct Sampler {
    pub filter: SamplerFilter,
    pub wrap: SamplerWrap,
    pub vflip: bool,
    pub srgb: bool,
    pub internal: SamplerInternalType,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SamplerFilter {
    Nearest,
    Linear,
    Mipmap,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SamplerWrap {
    Clamp,
    Repeat,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SamplerInternalType {
    Byte,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ShaderResponse {
    shader: Shader,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ShaderListResponse {
    results: Vec<String>,
}

fn as_url(url: &str) -> Url {
    Url::parse(url).unwrap_or_else(|e| panic!("Invalid url: {}", e))
}

fn join_url(url: &Url, path: &str) -> Url {
    url.join(path)
        .unwrap_or_else(|e| panic!("Invalid url join: {} + {} ({})", url, path, e))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Redacted<T>(pub T);

impl<T> Debug for Redacted<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl<T> Display for Redacted<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

pub fn bool_as_0_1<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let digit = u8::deserialize(d)?;

    match digit {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(D::Error::custom(format!(
            "Invalid boolean digit: {}",
            digit
        ))),
    }
}
