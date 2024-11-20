use crate::models::oath_2::OauthSecrets;

pub trait TOAuth2Service {
    fn request_access_token(
        &self,
        secrets: &mut OauthSecrets,
    ) -> anyhow::Result<(), Box<dyn std::error::Error>>;
}
