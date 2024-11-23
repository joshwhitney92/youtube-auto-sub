pub mod api;
pub(crate) mod consts;
pub(crate) mod models;
pub(crate) mod tools;

use anyhow::anyhow;
use api::{
    interfaces::{t_oauth2_service::TOAuth2Service, t_youtube_service::TYouTubeService},
    oauth2_service::OAuth2Service,
    youtube_repo::YouTubeRepository,
    youtube_service::YouTubeService,
};
use dotenv::dotenv;
use models::{oath_2::OauthSecrets, youtube::YouTubeChannel};
use std::{
    env::{self},
    ffi::OsString,
    fs::File,
};
use tokio::task::spawn_blocking;
use tools::{csv_reader::CSVReader, csv_writer::CSVWriter, interfaces::t_csv_reader::TCSVReader};

const MAX_RESULTS: i32 = 1;

fn get_first_arg() -> Result<OsString, anyhow::Error> {
    match env::args_os().nth(1) {
        None => Err(anyhow!("expected 1 argument!").into()),
        Some(file_path) => Ok(file_path),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    // Load environemnt variables from .env file
    dotenv().ok();

    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let channels: Vec<YouTubeChannel> = CSVReader::read_records(&file)?;

    // Fetch API key from env variables
    let api_key = env::var(consts::YOUTUBE_API_KEY).expect("YOUTUBE_API_KEY must be set!");

    // Fech OAuth secrets from env file
    let mut secrets = OauthSecrets {
        client_id: env::var(consts::CLIENT_ID).expect("CLIENT_ID must be set!"),
        client_secret: env::var(consts::CLIENT_SECRET).expect("CLIENT_SECRET must be set!"),
        auth_url: env::var(consts::AUTH_URI).expect("AUTH_URI must be set!"),
        token_url: env::var(consts::TOKEN_URI).expect("TOKEN_URI must be set!"),
        redirect_url: env::var(consts::REDIRECT_URI).expect("REDIRECT_URI must be set!"),
        access_token: String::from(""),
    };

    // TODO: add a set_headers(headers) method to the CSVWriter so i don't have to pass it to the
    // constructor.
    // let headers = vec![
    //     "Video ID".to_owned(),
    //     "Title".to_owned(),
    //     "Description".to_owned(),
    //     "Published At".to_owned(),
    // ];

    // Instantiate the dependnecies.
    let writer = CSVWriter::default();
    let repo = YouTubeRepository::default();
    let oauth2_service = OAuth2Service::default();

    // Retrieve the token_secrets from Google via Oauth
    let token_secrets =
        spawn_blocking(move || OAuth2Service::request_access_token(&mut secrets)).await?;

    let mut token_secrets = match token_secrets {
        Ok(it) => it,
        _ => {
            return Err(anyhow!("Could not retrieve token secrets!").into());
        }
    };

    // Pass in dependnecies for the service
    let api = YouTubeService::new(&writer, &repo, &oauth2_service);

    /* Writing videos to file
        let videos = api.get_videos(&api_key, channel_id, MAX_RESULTS).await?;

        Write the videos to file
        api.write_to_csv(videos);

         let channels =  vec![YouTubeChannel{
             channel_url: "url".to_string(),
             // channel_id: "UC_x5XG1OV2P6uZZ5FSM9Ttw".to_string()
             channel_id: "UCmXIqVsp5QWiVDpyBP32O0Q".to_string(),
             channel_title: String::new()
         }];
    */

    // Attempt to subscribe to channel
    // NOTE: Can't subscribe to more than 200 channels in a day (10k token limit from Google)
    // let result = api.subscribe(&api_key, &channels, &mut token_secrets).await?;
    // let videos = api.get_videos("api_key", "chanel id", MAX_RESULTS).await?;

    let result = api
        .subscribe(&api_key, &channels, &mut token_secrets)
        .await?;

    for failed_sub in result.failed {
        println!("Failed to subscribe to: {}", failed_sub.channel_id);
        eprintln!("With Error: {:?}", failed_sub.error);
    }
    println!("{} successful subs!", result.successful);

    Ok(())
}
