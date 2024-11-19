pub mod api;
pub(crate) mod tools;

use api::{youtube_repo::YouTubeRepository, YouTubeService};
use dotenv::dotenv;
use std::env;
use tools::csv_writer::CSVWriter;

const MAX_RESULTS: i32 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environemnt variables from .env file
    dotenv().ok();

    // Fetch API key from env variables
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set!");

    // Set the channel ID (change this to your desired YouTube channel)
    let channel_id = "UC-97WdoeCQenTaTyplgsoBg";

    // Placeholder for function calls (to be implemented)
    println!("API Key: {}, Channel ID: {}", api_key, channel_id);

    // Pass in dependnecies for the service
    let api = YouTubeService::new(
        CSVWriter::new(
            "youtube_videos.csv",
            // This should be a scruct
            vec![
                "Video ID".to_owned(),
                "Title".to_owned(),
                "Description".to_owned(),
                "Published At".to_owned(),
            ],
        ),
        YouTubeRepository::default(),
    );
    let videos = api.get_videos(&api_key, channel_id, MAX_RESULTS).await?;

    // Write the videos to file
    api.write_to_csv(videos)?;

    Ok(())
}
