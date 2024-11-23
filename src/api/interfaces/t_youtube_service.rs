use serde_json::Value;

use crate::models::{
    oath_2::OauthSecrets,
    youtube::{YouTubeChannel, YouTubeSubscriptionResult},
};

pub trait TYouTubeService {
    async fn get_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> anyhow::Result<Vec<Value>, Box<dyn std::error::Error>>;

    fn write_to_csv(&self, videos: Vec<Value>, path: &str, headers: &Vec<String>) -> anyhow::Result<(), Box<dyn std::error::Error>>;

    async fn subscribe(
        &self,
        api_key: &str,
        channels: &Vec<YouTubeChannel>,
        secrets: &mut OauthSecrets,
    ) -> anyhow::Result<YouTubeSubscriptionResult, Box<dyn std::error::Error>>;
}
