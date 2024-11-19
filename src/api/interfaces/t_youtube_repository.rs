use serde_json::Value;

/// Methods for fetching data from the YouTube api.
pub trait TYouTubeRepository {
    async fn fetch_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>>;
}
