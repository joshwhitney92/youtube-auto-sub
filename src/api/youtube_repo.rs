use crate::{
    api::interfaces::t_youtube_repository::TYouTubeRepository,
    models::{oath_2::OauthSecrets, youtube::YouTubeChannel},
};
// use anyhow::Ok;
use reqwest::Client;
use serde_json::{json, Value};

#[derive(Default)]
pub struct YouTubeRepository {}

// Implement the Repository for the YouTube service
impl TYouTubeRepository for YouTubeRepository {
    /// Fetch vidoes for a given channel_id.
    ///  # Parameters
    ///  `api_key`: Your API key.
    ///  `channel_id`: YouTube channel id to fetch videos from.
    ///  `max_results`: Max number of videos to fetch.
    async fn fetch_videos(
        &self,
        api_key: &str,
        channel_id: &str,
        max_results: i32,
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

            // Is there a way to abstract out this Client::new() dependency?
            let response = Client::new().get(&url).send().await?;

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

    async fn subscribe(
        &self,
        api_key: &str,
        channel: &YouTubeChannel,
        secrets: &mut OauthSecrets,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // build the api request url
        let url = format!(
            "https://youtube.googleapis.com/youtube/v3/subscriptions?part=snippet&key={} HTTP/1.1",
            api_key
        );
        // let url_old = format!(
        //     r#"POST https://youtube.googleapis.com/youtube/v3/subscriptions?part=snippet&key={} HTTP/1.1

        //        Authorization: Bearer {}
        //        Accept: application/json
        //        Content-Type: application/json
        //
        //        {{
        //          "snippet": {{
        //            "resourceId": {{
        //              "kind": "youtube#channel",
        //              "channelId": "{}"
        //            }}
        //          }}
        //        }}"#,
        //     api_key, access_token, channel.channel_id
        // );
        let json_payload = json!({
           "snippet": {
               "resourceId": {
                   "kind": "youtube#channel",
                   "channelId": channel.channel_id
               }
           }
        });

        // Is there a way to abstract out this Client::new() dependency?
        let response = Client::new()
            .post(&url)
            .header("Authorization", format!("Bearer {}", secrets.access_token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&json_payload)
            .send()
            .await?;

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
            println!("API returned an error: {:?}", error);
            return Err("API returned an error".into());
        } else {
            Ok(())
        }
    }
}
