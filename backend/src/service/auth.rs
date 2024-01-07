use crate::models::claims::Claims;
use crate::service::AuthService;
use anyhow::Context;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use log::error;
use std::time::Duration;
use uuid::Uuid;

impl AuthService {
    pub fn generate_token(&self, user_id: Uuid) -> Result<String, anyhow::Error> {
        let now = Utc::now();
        let expires_in = now + Duration::from_secs(self.options.token_expiration_in_seconds);

        let claims = Claims {
            aud: self.options.audience.clone(),
            sub: user_id,
            exp: expires_in.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.options.encoding_key.as_bytes()),
        )
        .map_err(|e| {
            error!("{}", e.to_string());
            e
        })
        .context("failed to encode JWT")
    }

    pub fn verify_token(&self, token: String) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[self.options.audience.clone()]);

        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.options.encoding_key.as_bytes()),
            &validation,
        )
    }

    pub fn hash_password(&self, password: String) -> Result<Vec<u8>, anyhow::Error> {
        let hashed = bcrypt::hash(password, 12).context("failed to hash user's password")?;
        Ok(hashed.as_bytes().to_vec())
    }

    pub fn compare_hash_and_password(
        &self,
        password: String,
        hash_bytes: Vec<u8>,
    ) -> Result<bool, anyhow::Error> {
        let hash_as_str =
            std::str::from_utf8(&hash_bytes).context("failed to convert hash bytes to string")?;
        let verification_result =
            bcrypt::verify(password, hash_as_str).context("failed to verify hash with password")?;
        Ok(verification_result)
    }
}
