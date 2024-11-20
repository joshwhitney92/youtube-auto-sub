use serde_json::Value;

use crate::models::{oath_2::OauthSecrets, youtube::YouTubeChannel};

/// Methods for fetching data from the YouTube api.
pub trait TYouTubeRepository {
    async fn fetch_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> anyhow::Result<Vec<Value>, Box<dyn std::error::Error>>;

    async fn subscribe(&self, api_key: &str, channel: &YouTubeChannel, secrets: &mut OauthSecrets) -> anyhow::Result<(), Box<dyn std::error::Error>>;
}
