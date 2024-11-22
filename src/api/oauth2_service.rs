use crate::models::oath_2::OauthSecrets;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use ::reqwest::{blocking::ClientBuilder, redirect::Policy};
use url::Url;

use super::interfaces::t_oauth2_service::TOAuth2Service;
use anyhow::anyhow;
use oauth2::{basic::BasicClient, TokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};

#[derive(Default)]
pub struct OAuth2Service {}

impl TOAuth2Service for OAuth2Service {
    fn request_access_token(
        secrets: &mut OauthSecrets,
    ) -> anyhow::Result<OauthSecrets, Box<dyn std::error::Error + Send>> {
        let auth_url = match AuthUrl::new(secrets.auth_url.clone()) {
            Ok(url) => url,
            _ => {
                println!("MOTHER FUCKER!");
                return Err(anyhow!("Mother FUCKER!").into());
            }
        };

        let token_url = match TokenUrl::new(secrets.token_url.clone()) {
            Ok(url) => Some(url),
            _ => {
                println!("MOTHER FUCKER 2!");
                return Err(anyhow!("Mother FUCKER 2!").into());
            }
        };

        let client = BasicClient::new(
            ClientId::new(secrets.client_id.clone()),
            Some(ClientSecret::new(secrets.client_secret.clone())),
            // AuthUrl::new(secrets.auth_url.clone())?,
            auth_url,
            token_url,
        )
        .set_redirect_uri(
            // RedirectUrl::new(secrets.redirect_url.clone()).expect("Invalid redirec url"),
            RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect url"),
        );

        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL.
        let (auth_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            // Set the desired scopes.
            // .add_scope(Scope::new("read".to_string()))
            // .add_scope(Scope::new("write".to_string()))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/youtube".to_string(),
            ))
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        // This is the URL you should redirect the user to, in order to trigger the authorization
        // process.
        println!("Browse to: {}", auth_url);

        // let _ = std::io::stdin().read_line(&mut String::new());
        let (code, state) = {
            // A very naive implementation of the redirec server.
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

            // The server will terminate itself after collecting the first code.
            let Some(mut stream) = listener.incoming().flatten().next() else {
                panic!("listener terminated without accepting a connection");
            };

            let mut reader = BufReader::new(&stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                .unwrap();

            let state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, state)| CsrfToken::new(state.into_owned()))
                .unwrap();

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );

            stream.write_all(response.as_bytes()).unwrap();

            (code, state)
        };

        println!("Google returned the following code:\n{}\n", code.secret());
        println!(
            "Google returned the following state:\n{} (expected `{}`)\n",
            state.secret(),
            csrf_state.secret()
        );


        // Exchange the code with a token.
        let token_response = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request(oauth2::reqwest::http_client);

        println!("Google returned the following token:\n{token_response:?}\n");

        let access_token = token_response.unwrap().access_token().secret().to_string();
        secrets.access_token = access_token;
        if secrets.access_token.is_empty() {
            Err(anyhow!("Access token was not retrieved!").into())
        } else {
            Ok(secrets.clone())
        }

        // if access_token.is_empty() {
        //     secrets.access_token = access_token.clone();
        //     Ok(())
        // } else {
        //     Err(anyhow!("Access token was not retrieved!").into())
        // }

        //  // Revoke the obtained token
        //  let token_response = token_response.unwrap();
        //  let token_to_revoke: StandardRevocableToken = match token_response.refresh_token() {
        //      Some(token) => token.into(),
        //      None => token_response.access_token().into(),
        //  };

        //  client
        //      .revoke_token(token_to_revoke)
        //      .unwrap()
        //      .request(&http_client)
        //      .expect("Failed to revoke token");
        // let token_result = client
        //     .exchange_client_credentials()
        //     .add_scope(Scope::new(
        //         "https://www.googleapis.com/auth/youtube".to_string(),
        //     ))
        //     .request(http_client);

        // let token_result = match token_result {
        //     Ok(result) => result,
        //     Err(e) => {
        //         println!("error: {:?}", e);
        //         return Err(anyhow!("MOTHER FUCKER 3!").into());
        //     }
        // };

        // Now you can trade it for an access token.
        // let token_result = client
        //     .exchange_code(AuthorizationCode::new(
        //         "some authorization code".to_string(),
        //     ))
        //     // Set the PKCE code verifier.
        //     .set_pkce_verifier(pkce_verifier)
        //     .request_async(async_http_client)
        //     .await?;

        // let access_token = token_result.access_token().secret().to_string();
    }
}
