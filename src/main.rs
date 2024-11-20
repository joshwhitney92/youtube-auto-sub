pub mod api;
pub(crate) mod consts;
pub(crate) mod models;
pub(crate) mod tools;

use anyhow::anyhow;
use api::{
    interfaces::{t_oauth2_service::TOAuth2Service, t_youtube_service::TYouTubeService}, oauth2_service::{self, OAuth2Service}, youtube_repo::YouTubeRepository, youtube_service::YouTubeService
};
use dotenv::dotenv;
use models::{oath_2::OauthSecrets, youtube::YouTubeChannel};
use tokio::task::spawn_blocking;
use std::{env::{self}, result};
use tools::csv_writer::CSVWriter;

const MAX_RESULTS: i32 = 1;

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    // Load environemnt variables from .env file
    dotenv().ok();

    // Fetch API key from env variables
    let api_key = env::var(consts::YOUTUBE_API_KEY).expect("YOUTUBE_API_KEY must be set!");

    let mut secrets = OauthSecrets {
        client_id: env::var(consts::CLIENT_ID).expect("CLIENT_ID must be set!"),
        client_secret: env::var(consts::CLIENT_SECRET).expect("CLIENT_SECRET must be set!"),
        auth_url: env::var(consts::AUTH_URI).expect("AUTH_URI must be set!"),
        token_url: env::var(consts::TOKEN_URI).expect("TOKEN_URI must be set!"),
        redirect_url: env::var(consts::REDIRECT_URI).expect("REDIRECT_URI must be set!"),
        access_token: String::from("")
    };

    // Set the channel ID (change this to your desired YouTube channel)
    let channel_id = "UC-97WdoeCQenTaTyplgsoBg";

    // Placeholder for function calls (to be implemented)
    println!("API Key: {}, Channel ID: {}", api_key, channel_id);

    let headers = vec![
        "Video ID".to_owned(),
        "Title".to_owned(),
        "Description".to_owned(),
        "Published At".to_owned(),
    ];

    let writer = CSVWriter::new(
        "youtube_videos.csv",
        // This should be a scruct
        &headers,
    );

    let repo = YouTubeRepository::default();
    let oauth2_service  = OAuth2Service::default();

    let  token_secrets = spawn_blocking(move || {
        OAuth2Service::request_access_token(&mut secrets)
    }).await?;

    let mut token_secrets = match token_secrets {
       Ok(it)  => it,
       _ => {
           return Err(anyhow!("MOTHER FUCKER 4").into());
       }
    };


    // Pass in dependnecies for the service
    let api = YouTubeService::new(&writer, &repo, &oauth2_service);
    // let videos = api.get_videos(&api_key, channel_id, MAX_RESULTS).await?;

    // Write the videos to file
    // api.write_to_csv(videos);

    let channels =  vec![YouTubeChannel{
        channel_url: "url".to_string(),
        // channel_id: "UC_x5XG1OV2P6uZZ5FSM9Ttw".to_string()
        channel_id: "UCmXIqVsp5QWiVDpyBP32O0Q".to_string()
    }];
    
    // Attempt to subscribe to channel
    let result = api.subscribe(&api_key, &channels, &mut token_secrets).await?;
    for failed_sub in result.failed {
        println!("Failed to subscribe to: {}", failed_sub.channel_id);
        eprintln!("With Error: {:?}", failed_sub.error);

    }
    println!("{} successful subs!", result.successful);

    Ok(())
}
