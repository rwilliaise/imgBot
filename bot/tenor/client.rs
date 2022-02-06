use crate::tenor::{GifsResponse, MediaObject};
use regex::Regex;
use std::env;
use err_context::AnyError;
use shared::TenorError;

pub struct TenorClient {
    inner: reqwest::Client,
    key: Option<String>,
}

impl TenorClient {
    pub fn new(inner: reqwest::Client) -> Self {
        let key = env::var("IMGBOT_TENOR_APIKEY").ok();

        Self { inner, key }
    }

    pub fn is_available(&self) -> bool {
        self.key.is_some()
    }

    pub async fn get_gif<'a>(&self, url: String) -> Result<MediaObject, TenorError> {
        if !self.is_available() {
            return Err(TenorError::Unavailable);
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
            "https://g.tenor.com/v1/gifs?ids={}&key={}&media_filter=normal&limit=1",
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

        Ok(response.mediumgif.clone().ok_or(TenorError::BadResponse("No gif"))?)
    }
}
