use crate::models::oath_2::OauthSecrets;

use super::interfaces::t_oauth2_service::TOAuth2Service;
use anyhow::anyhow;
use oauth2::{
    basic::BasicClient,
    reqwest::{async_http_client, http_client},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use tokio::io::{stdin, AsyncReadExt};

#[derive(Default)]
pub struct OAuth2Service {}

impl TOAuth2Service for OAuth2Service {
    fn request_access_token(
        &self,
        secrets: &mut OauthSecrets,
    ) -> anyhow::Result<(), Box<dyn std::error::Error>> {
        let client = BasicClient::new(
            ClientId::new(secrets.client_id.clone()),
            Some(ClientSecret::new(secrets.client_secret.clone())),
            AuthUrl::new(secrets.auth_url.clone())?,
            Some(TokenUrl::new(secrets.token_url.clone())?),
        );
        // .set_redirect_uri(RedirectUrl::new(secrets.redirect_url.clone())?);

        // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL.
        // let (auth_url, csrf_token) = client
        //     .authorize_url(CsrfToken::new_random)
        //     // Set the desired scopes.
        //     // .add_scope(Scope::new("read".to_string()))
        //     // .add_scope(Scope::new("write".to_string()))
        //     .add_scope(Scope::new("https://www.googleapis.com/auth/youtube".to_string()))
        //     // Set the PKCE code challenge.
        //     .set_pkce_challenge(pkce_challenge)
        //     .url();

        // This is the URL you should redirect the user to, in order to trigger the authorization
        // process.
        // println!("Browse to: {}", auth_url);
        // let _ = std::io::stdin().read_line(&mut String::new());
        //

        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/youtube".to_string(),
            ))
            .request(http_client)?;

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
        // let access_token = token_result.unwrap().access_token().secret().to_string();
        if let access_token = token_result.access_token().secret().to_string() {
            if secrets.access_token.is_empty() {
                Err(anyhow!("Access token was not retrieved!").into())
            } else {
                Ok(())
            }
        } else {
            Err(anyhow!("Access token was not retrieved!").into())
        }
        // if access_token.is_empty() {
        //     secrets.access_token = access_token.clone();
        //     Ok(())
        // } else {
        //     Err(anyhow!("Access token was not retrieved!").into())
        // }
    }
}
