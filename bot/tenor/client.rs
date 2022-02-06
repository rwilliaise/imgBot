use crate::tenor::{GifsResponse, MediaObject};
use regex::Regex;
use shared::TenorError;
use std::collections::HashMap;
use std::env;

pub struct TenorClient {
    inner: reqwest::Client,
    key: Option<String>,
    cache: HashMap<String, MediaObject>,
}

impl TenorClient {
    pub fn new(inner: reqwest::Client) -> Self {
        let key = env::var("IMGBOT_TENOR_APIKEY").ok();

        Self {
            inner,
            key,
            cache: Default::default(),
        }
    }

    pub fn is_available(&self) -> bool {
        self.key.is_some()
    }

    pub async fn fetch<'a>(&mut self, url: String) -> Result<MediaObject, TenorError> {
        if !self.is_available() {
            return Err(TenorError::Unavailable);
        }

        let cached = self.cache.get(url.as_str());
        if cached.is_some() {
            return Ok(cached.unwrap().clone());
        }

        let id = Regex::new("(\\d+$)");

        if let Err(_) = id {
            return Err(TenorError::NoProcessed);
        }

        let matched = id.unwrap().find(url.as_str());

        if let None = matched {
            return Err(TenorError::InvalidLink);
        }

        let request = format!(
            "https://g.tenor.com/v1/gifs?ids={}&key={}&limit=1",
            matched.unwrap().as_str(),
            self.key.clone().unwrap()
        );
        let response = self.inner.get(request).send().await;

        if let Err(_) = response {
            return Err(TenorError::BadResponse("Non-2xx error code"));
        }

        let response = response.unwrap();
        let response = response.json::<GifsResponse>().await;

        if let Err(e) = response {
            return Err(TenorError::CannotParse(e.into()));
        }

        let response = response.unwrap();

        if response.results.len() == 0 {
            return Err(TenorError::BadResponse("Got 0 responses"));
        }

        let response = response.results.get(0).unwrap();

        if response.media.len() == 0 {
            return Err(TenorError::BadResponse("Got 0 media responses"));
        }

        let response = response.media.get(0).unwrap();
        let out = response.mediumgif.clone().unwrap_or(
            response
                .gif
                .clone()
                .ok_or(TenorError::BadResponse("No gif"))?,
        );
        self.cache.insert(url, out.clone());

        Ok(out)
    }
}
