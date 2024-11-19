pub(crate) mod tools;
pub mod api;

use api::YouTubeService;
use dotenv::dotenv;
use tools::{csv_writer::CSVWriter};
use std::env;

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

    // let api = YouTubeService::new();
    let api = YouTubeService::new(CSVWriter::new(
        "youtube_videos.csv",
        // This should be a scruct
        vec![
            "Video ID".to_owned(),
            "Title".to_owned(),
            "Description".to_owned(),
            "Published At".to_owned(),
        ],
    ));
    // let videos = api.get_videos(&api_key, channel_id, 1).await?;
    // let videos = api.fetch_videos(&api_key, channel_id, 1).await?;
    let videos = api.get_videos(&api_key, channel_id, MAX_RESULTS).await?;
    // let writer = YouTubeCSVWriter::new(path, headers);
    // writer.write_records(records);

    // Write the videos to file
    api.write_to_csv(videos);

    Ok(())
}
