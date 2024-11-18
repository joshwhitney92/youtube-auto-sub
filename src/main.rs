pub mod api;

use api::YouTubeAPI;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environemnt variables from .env file
    dotenv().ok();

    // Fetch API key from env variables
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set!");

    // Set the channel ID (change this to your desired YouTube channel)
    let channel_id = "UC-91UA-Xy2Cvb98deRXuggA";

    // Placeholder for function calls (to be implemented)
    println!("API Key: {}, Channel ID: {}", api_key, channel_id);

    let api = YouTubeAPI::new();
    let videos = api.fetch_videos(&api_key, channel_id, 1).await?;

    // Write the videos to file
    api.write_to_csv(videos);

    Ok(())
}
