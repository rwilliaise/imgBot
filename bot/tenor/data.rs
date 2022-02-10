#[derive(serde::Deserialize, Debug, Clone)]
pub struct MediaObject {
    pub preview: String,
    pub url: String,
    pub dims: Vec<u32>,
    pub size: u32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GifMedia {
    // minimal
    #[serde(default)]
    pub gif: Option<MediaObject>,
    #[serde(default)]
    pub tinygif: Option<MediaObject>,
    #[serde(default)]
    pub mp4: Option<MediaObject>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GifObject {
    pub created: f32,
    pub hasaudio: bool,
    pub id: String,
    pub media: Vec<GifMedia>,
    pub tags: Vec<String>,
    pub title: String,
    pub itemurl: String,
    pub hascaption: bool,
    pub url: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GifsResponse {
    pub next: String,
    pub results: Vec<GifObject>,
}
