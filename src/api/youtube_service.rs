use serde_json::Value;

use super::
    interfaces::{
        t_oauth2_service::TOAuth2Service, t_youtube_repository::TYouTubeRepository,
        t_youtube_service::TYouTubeService,
    }
;
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
        Ok(self
            .repository
            .fetch_videos(api_key, channel_id, max_results)
            .await?)
    }

    fn write_to_csv(&self, videos: Vec<Value>) -> anyhow::Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        self.writer.write_records(videos)?;
        Ok(())
    }

    async fn subscribe(
        &self,
        api_key: &str,
        channels: &Vec<YouTubeChannel>,
        secrets: &mut OauthSecrets,
    ) -> anyhow::Result<YouTubeSubscriptionResult, Box<dyn std::error::Error>> {
        let mut result = YouTubeSubscriptionResult::default();
        result.expected = channels.len() as i32;
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
    use super::*;
    use mocks::*;
    use oauth2_service::OAuth2Service;
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
            // NOTE: Should this field just be public?
            records: RefCell<Vec<Value>>,
        }

        impl MockWriter {
            pub fn builder() -> MockWriterBuilder {
                MockWriterBuilder::default()
            }

            /// This is a getter for the records field.
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
            fn write_records(
                &self,
                records: Vec<Value>,
            ) -> anyhow::Result<(), Box<dyn std::error::Error>> {
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
        let result = sut.write_to_csv(videos);

        // Assert
        assert!(result.is_ok());
        println!("{} records written!", writer.records().borrow().len());
        assert!(writer.records().borrow().len() == 1 as usize);
    }
}
