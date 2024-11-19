use serde_json::Value;

use crate::tools::interfaces::t_csv_writer::TCSVWriter;
use super::interfaces::{t_youtube_repository::TYouTubeRepository, t_youtube_service::TYouTubeService};

pub struct YouTubeService<W, R>
where
    W: TCSVWriter,
    R: TYouTubeRepository
{
    writer: W,
    repository: R
}

impl<W, R> TYouTubeService for YouTubeService<W, R>
where 
    W: TCSVWriter,
    R: TYouTubeRepository
{
    async fn get_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        self.repository.fetch_videos(api_key, channel_id, max_results).await
    }

    fn write_to_csv(&self, videos: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        self.writer.write_records(videos)?;
        Ok(())
    }
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
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[derive(Default)]
    struct MockRepo {}
    impl TYouTubeRepository for MockRepo {
        async fn fetch_videos(
            &self,
            api_key: &str,
            channel_id: &str,
            max_results: i32,
        ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
            let videos: Vec<Value> = Vec::new();
            Ok(videos)
        }
    }


    #[derive(Default)]
    struct MockWriter {}
    impl TCSVWriter for MockWriter {
        fn write_records(&self, records: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }


    #[tokio::test]
    async fn it_works() {
        // arrange
        let sut = YouTubeService::new(MockWriter::default(), MockRepo::default());
        let api_key = "key";
        let channel_id = "channel_id";
        let max_results = 1;

        // act
        let result = sut.get_videos(api_key, channel_id, max_results).await;
        
        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().len() > 1 as usize);
    }
}
