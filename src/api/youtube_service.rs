use serde_json::Value;

use super::interfaces::{
    t_youtube_repository::TYouTubeRepository, t_youtube_service::TYouTubeService,
};
use crate::tools::interfaces::t_csv_writer::TCSVWriter;

pub struct YouTubeService<'a, W, R>
where
    W: TCSVWriter,
    R: TYouTubeRepository,
{
    writer: &'a W,
    repository: &'a R,
}

impl<'a, W, R> TYouTubeService for YouTubeService<'a, W, R>
where
    W: TCSVWriter,
    R: TYouTubeRepository,
{
    async fn get_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        Ok(self
            .repository
            .fetch_videos(api_key, channel_id, max_results)
            .await?)
    }

    fn write_to_csv(&self, videos: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        self.writer.write_records(videos)?;
        Ok(())
    }
}

impl<'a, W, R> YouTubeService<'a, W, R>
where
    W: TCSVWriter,
    R: TYouTubeRepository,
{
    pub fn new(writer: &'a W, repository: &'a R) -> Self {
        Self { repository, writer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mocks::*;
    use serde_json::json;

    mod mocks {
        use std::cell::RefCell;

        use super::*;

        pub struct MockRepo {
            videos: Vec<Value>,
        }
        impl MockRepo {
            pub fn builder() -> MockRepoBuilder {
                MockRepoBuilder::default()
            }
        }

        #[derive(Default)]
        pub struct MockRepoBuilder {
            videos: Vec<Value>,
        }

        impl MockRepoBuilder {
            pub fn with_videos(mut self, videos: Vec<Value>) -> MockRepoBuilder {
                self.videos = videos;
                self
            }

            pub fn build(self) -> MockRepo {
                MockRepo {
                    videos: self.videos,
                }
            }
        }
        #[derive(Default)]
        pub struct MockWriter {
            // Need interior mutability here to represent records written to a file.
            records: RefCell<Vec<Value>>,
        }

        impl MockWriter {
            pub fn builder() -> MockWriterBuilder {
                MockWriterBuilder::default()
            }

            pub fn records(&self) -> &RefCell<Vec<Value>> {
                &self.records
            }
        }

        #[derive(Default)]
        pub struct MockWriterBuilder {
            records: Vec<Value>,
        }

        impl MockWriterBuilder {
            pub fn with_records(mut self, records: Vec<Value>) -> MockWriterBuilder {
                self.records = records;
                self
            }
            pub fn build(self) -> MockWriter {
                MockWriter {
                    records: RefCell::new(self.records),
                }
            }
        }

        impl TCSVWriter for MockWriter {
            fn write_records(&self, records: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
                for record in records {
                    self.records.borrow_mut().push(record);
                }

                Ok(())
            }
        }

        impl TYouTubeRepository for MockRepo {
            async fn fetch_videos(
                &self,
                api_key: &str,
                channel_id: &str,
                max_results: i32,
            ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
                Ok(self.videos.clone())
            }
        }
    }

    #[tokio::test]
    async fn get_videos_works() {
        // arrange
        let videos = vec![json!("a string")];
        // let videos = Vec::new();
        let repo: MockRepo = MockRepo::builder().with_videos(videos).build();
        let writer = MockWriter::builder().build();
        let sut = YouTubeService::new(&writer, &repo);
        let api_key = "key";
        let channel_id = "channel_id";
        let max_results = 1;

        // act
        let result = sut.get_videos(api_key, channel_id, max_results).await;

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 1 as usize);
    }

    #[tokio::test]
    async fn write_to_csv_works() {
        // arrange
        let videos = vec![json!("a string")];
        // let videos = Vec::new();
        let repo: MockRepo = MockRepo::builder().build();
        let writer: MockWriter = MockWriter::builder().build();
        let sut = YouTubeService::new(&writer, &repo);

        // act
        println!("{} records to write!", videos.len());
        let result = sut.write_to_csv(videos);

        // Assert
        assert!(result.is_ok());
        println!("{} records written!", writer.records().borrow().len());
        assert!(writer.records().borrow().len() == 1 as usize);
    }
}
