use serde_json::Value;

use super::interfaces::{
    t_oauth2_service::TOAuth2Service, t_youtube_repository::TYouTubeRepository,
    t_youtube_service::TYouTubeService,
};
use crate::{
    models::{
        oath_2::OauthSecrets,
        youtube::{FailedYoutubeSubscription, YouTubeChannel, YouTubeSubscriptionResult},
    },
    tools::interfaces::t_csv_writer::TCSVWriter,
};

pub struct YouTubeService<'a, W, R, O>
where
    W: TCSVWriter,
    R: TYouTubeRepository,
    O: TOAuth2Service,
{
    writer: &'a W,
    repository: &'a R,
    oauth2_service: &'a O,
}

impl<'a, W, R, O> TYouTubeService for YouTubeService<'a, W, R, O>
where
    W: TCSVWriter + Send + Sync,
    R: TYouTubeRepository + Send + Sync,
    O: TOAuth2Service + Send + Sync,
{
    async fn get_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
    ) -> anyhow::Result<Vec<Value>, Box<dyn std::error::Error>> {
        self.repository
            .fetch_videos(api_key, channel_id, max_results)
            .await
    }

    fn write_to_csv(
        &self,
        videos: Vec<Value>,
        path: &str,
        headers: &[String],
    ) -> anyhow::Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        self.writer.write_records(videos, path, headers)?;
        Ok(())
    }

    async fn subscribe(
        &self,
        api_key: &str,
        channels: &[YouTubeChannel],
        secrets: &mut OauthSecrets,
    ) -> anyhow::Result<YouTubeSubscriptionResult, Box<dyn std::error::Error>> {
        let mut result = YouTubeSubscriptionResult {
            expected: channels.len() as i32,
            ..Default::default()
        };

        for channel in channels {
            match self.repository.subscribe(api_key, channel, secrets).await {
                Ok(_) => {
                    result.successful += 1;
                }
                Err(e) => {
                    result.failed.push(FailedYoutubeSubscription {
                        channel_url: channel.channel_url.clone(),
                        channel_id: channel.channel_id.clone(),
                        error: e,
                    });
                }
            }
        }
        Ok(result)
    }
}

impl<'a, W, R, O> YouTubeService<'a, W, R, O>
where
    W: TCSVWriter,
    R: TYouTubeRepository,
    O: TOAuth2Service,
{
    pub fn new(writer: &'a W, repository: &'a R, oauth2_service: &'a O) -> Self {
        Self {
            repository,
            writer,
            oauth2_service,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::BorrowMut,
        cell::RefCell,
        sync::{Arc, Mutex},
    };

    use crate::{api::oauth2_service::OAuth2Service, tools::csv_writer::CSVWriter};

    use super::*;
    use serde_json::json;
    use tokio::stream;

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
        // NOTE: Should this field just be public?
        // NOTE: MockWriter must be Send + Sync in order to fufill trait bound on TYouTubeService!
        // NOTE: A type is only Send + Sync if all of it's members are Send + Sync!
        records: Arc<Mutex<Vec<Value>>>,
    }

    impl MockWriter {
        pub fn builder() -> MockWriterBuilder {
            MockWriterBuilder::default()
        }

        /// This is a getter for the records field.
        pub fn records(&self) -> &Arc<Mutex<Vec<Value>>> {
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
                records: Arc::new(Mutex::new(self.records)),
            }
        }
    }

    impl TCSVWriter for MockWriter {
        fn write_records(
            &self,
            records: Vec<Value>,
            path: &str,
            headers: &[String],
        ) -> anyhow::Result<(), Box<dyn std::error::Error>> {
            for record in records {
                self.records.lock().unwrap().push(record);
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
        ) -> anyhow::Result<Vec<Value>, Box<dyn std::error::Error>> {
            Ok(self.videos.clone())
        }

        async fn subscribe(
            &self,
            api_key: &str,
            channel: &YouTubeChannel,
            secrets: &mut OauthSecrets,
        ) -> Result<(), Box<dyn std::error::Error>> {
            todo!()
        }
    }

    #[tokio::test]
    async fn get_videos_works() {
        // arrange
        let videos = vec![json!("a string")];
        let repo: MockRepo = MockRepo::builder().with_videos(videos).build();
        let writer = MockWriter::builder().build();
        let oauth2_service = OAuth2Service::default();
        let sut = YouTubeService::new(&writer, &repo, &oauth2_service);
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
        let repo: MockRepo = MockRepo::builder().build();
        let writer: MockWriter = MockWriter::builder().build();
        let oauth2_service = OAuth2Service::default();
        let sut = YouTubeService::new(&writer, &repo, &oauth2_service);

        // act
        println!("{} records to write!", videos.len());
        let headers = vec![String::new()];
        let result = sut.write_to_csv(videos, "path", &headers);

        // Assert
        assert!(result.is_ok());
        println!(
            "{} records written!",
            writer.records().lock().unwrap().len()
        );
        assert!(writer.records().lock().unwrap().len() == 1 as usize);
    }
}
