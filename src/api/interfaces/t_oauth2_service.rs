use crate::models::oath_2::OauthSecrets;

pub trait TOAuth2Service {
    fn request_access_token(
        secrets: &mut OauthSecrets,
    ) -> anyhow::Result<OauthSecrets, Box<dyn std::error::Error + Send>>;
}
