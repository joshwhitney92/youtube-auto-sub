use csv::Writer;
use reqwest::Client;
use serde_json::Value;

pub struct YouTubeAPI {
    client: Client,
}

impl YouTubeAPI {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn write_to_csv(&self, videos: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        let mut writer = Writer::from_path("youtube_videos.csv")?;

        // Write the header row
        writer.write_record(&["Video ID", "Title", "Description", "Published At"])?;

        for video in videos {
            let snippet = &video["snippet"];

            // Write each video's data to the CSV file
            writer.write_record(&[
                video["id"]["videoId"].as_str().unwrap_or(""),
                snippet["title"].as_str().unwrap_or(""),
                snippet["description"].as_str().unwrap_or(""),
                snippet["publishedAt"].as_str().unwrap_or(""),
            ])?;
        }

        // Ensure all data is written to the file
        writer.flush()?;

        Ok(())
    }

    /// Fetch vidoes for a given channel_id.
    ///  # Parameters
    ///  `api_key`: Your API key.
    ///  `channel_id`: YouTube channel id to fetch videos from.
    ///  `max_results`: Max number of videos to fetch.
    pub async fn fetch_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let mut videos: Vec<Value> = Vec::new();
        let mut page_token = String::new();

        loop {
            if videos.len() > max_results as usize {
                break;
            }

            // build the api request url
            let url = format!(
            "https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults={}&type=video&pageToken={}",
            api_key,
            channel_id,
            max_results,
            page_token);

            let response = self.client.get(&url).send().await?;

            // Check if the response was successful
            if !response.status().is_success() {
                println!("API request failed with status: {}", response.status());
                println!("Response body: {}", response.text().await?);

                return Err("API request failed".into());
            }

            // Parse the response body
            let json: Value = response.json().await?;

            // Check for API errors
            if let Some(error) = json.get("error") {
                print!("API returned an error: {:?}", error);
                return Err("API returned an error".into());
            }

            // Extract video items and add to the videos vector
            if let Some(items) = json["items"].as_array() {
                videos.extend(items.clone());
            }

            // Handle pagination by checking for the nextPageToken
            if let Some(next_page_token) = json["nextPageToken"].as_str() {
                page_token = next_page_token.to_string();
            } else {
                break;
            }
        }

        Ok(videos)
    }
}
