use std::time::Duration;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use uuid::Uuid;
use crate::models::claims::Claims;
use crate::service::AuthService;

impl AuthService {
    pub fn generate_token(&self, user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expires_in = now + Duration::from_secs(self.options.token_expiration_in_seconds);

        let claims = Claims {
            aud: self.options.audience.clone(),
            sub: user_id,
            exp: expires_in.timestamp(),
            iat: now.timestamp()
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.options.encoding_key.as_bytes()))
    }

    pub fn verify_token(&self, token: String) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[self.options.audience.clone()]);

        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.options.encoding_key.as_bytes()),
            &validation
        )
    }
}
