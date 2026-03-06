use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i32)]
pub enum Status {
    Unknown = 0,
    Ongoing = 1,
    Completed = 2,
    Cancelled = 3,
    Hiatus = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i32)]
pub enum ContentRating {
    Safe = 0,
    Suggestive = 1,
    Nsfw = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Episode {
    pub key: String,
    pub title: Option<String>,
    pub episode: Option<f32>,
    pub date_updated: Option<f64>,
    pub url: Option<String>,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioTrack {
    pub url: String,
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subtitle {
    pub url: String,
    pub language: String,
    pub format: String,
    pub is_hardsub: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Season {
    pub key: String,
    pub title: String,
    pub is_current: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub url: String,
    pub quality: String,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub audio_tracks: Option<Vec<AudioTrack>>,
    pub subtitles: Option<Vec<Subtitle>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anime {
    pub key: String,
    pub title: String,
    pub studios: Option<Vec<String>>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub cover: Option<String>,
    pub url: Option<String>,
    pub status: Status,
    pub content_rating: ContentRating,
    pub nsfw: i32,
    pub episodes: Option<Vec<Episode>>,
    pub seasons: Option<Vec<Season>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageResult {
    pub entries: Vec<Anime>,
    pub has_next_page: bool,
}
