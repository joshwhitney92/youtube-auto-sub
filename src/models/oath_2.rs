#[derive(Debug, Clone, Default)]
pub struct OauthSecrets {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
    pub access_token: String
}
