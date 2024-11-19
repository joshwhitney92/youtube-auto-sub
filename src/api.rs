mod interfaces;
pub mod youtube_repo;

use interfaces::TYouTubeRepository;
use reqwest::Client;
use serde_json::Value;
use crate::tools::interfaces::t_csv_writer::TCSVWriter;


pub struct YouTubeService<W, R>
where
    W: TCSVWriter,
    R: TYouTubeRepository
{
    writer: W,
    repository: R
}



impl<W, R> YouTubeService<W, R>
where 
    W: TCSVWriter,
    R: TYouTubeRepository
{
    pub fn new(writer: W, repository: R) -> Self {
        Self {
            repository,
            writer
        }
    }

    pub async fn get_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        self.repository.fetch_videos(api_key, channel_id, max_results).await
    }

    pub fn write_to_csv(&self, videos: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        self.writer.write_records(videos)?;
        Ok(())
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;

}
