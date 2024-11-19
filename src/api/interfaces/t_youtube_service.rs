use serde_json::Value;

pub trait TYouTubeService {
    async fn get_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>>;

    fn write_to_csv(&self, videos: Vec<Value>) -> Result<(), Box<dyn std::error::Error>>;
}
