use std::collections::HashMap;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use anyhow::Error;
use rss::Channel;

#[derive(Deserialize, Debug)]
pub struct RecentEps {
    pub episodes: Vec<Episode>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PodcastSearchResult {
    pub(crate) status: String,
    pub(crate) feeds: Vec<Podcast>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct Podcast {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) url: String,
    #[allow(non_snake_case)]
    pub(crate) originalUrl: String,
    pub(crate) link: String,
    pub(crate) description: String,
    pub(crate) author: String,
    #[allow(non_snake_case)]
    pub(crate) ownerName: String,
    pub(crate) image: String,
    pub(crate) artwork: String,
    #[allow(non_snake_case)]
    pub(crate) lastUpdateTime: i64,
    pub(crate) categories: Option<HashMap<String, String>>,
    pub(crate) explicit: bool,
    #[allow(non_snake_case)]
    pub(crate) episodeCount: i32,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Episode {
    pub title: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub links: Vec<String>,
    pub enclosure_url: Option<String>,
    pub enclosure_length: Option<String>,
    pub artwork: Option<String>,
    pub content: Option<String>,
    pub authors: Vec<String>,
    pub guid: String,
    pub duration: Option<String>
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PodcastFeedResult {
    // ... other fields ...
    pub(crate) episodes: Vec<Episode>,
}

pub async fn call_get_podcast_info(podcast_value: &String, search_api_url: &Option<String>, search_index: &str) -> Result<PodcastSearchResult, anyhow::Error> {
    let url = if let Some(api_url) = search_api_url {
        format!("{}?query={}&index={}", api_url, podcast_value, search_index)
    } else {
        return Err(anyhow::Error::msg("API URL is not provided"));
    };

    let response = Request::get(&url).send().await.map_err(|err| anyhow::Error::new(err))?;

    if response.ok() {
        let response_text = response.text().await.map_err(|err| anyhow::Error::new(err))?;
        web_sys::console::log_1(&format!("Raw Response: {}", response_text).into());

        let search_results: PodcastSearchResult = serde_json::from_str(&response_text)?;
        Ok(search_results)
    } else {
        Err(anyhow::Error::msg(format!("Failed to fetch podcast info: {}", response.status_text())))
    }
}

pub async fn test_connection(search_api_url: &Option<String>) -> Result<(), Error> {
    let url = search_api_url.as_ref().ok_or_else(|| Error::msg("API URL is missing"))?;

    match Request::get(url).send().await {
        Ok(response) => {
            if response.ok() {
                Ok(())
            } else {
                Err(Error::msg(format!("HTTP error occurred: {}", response.status_text())))
            }
        }
        Err(err) => Err(Error::new(err)),
    }
}

// In Databases

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub search_term: String,
    pub user_id: i32,
}