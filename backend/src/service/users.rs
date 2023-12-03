use anyhow::Context;
use log::error;
use thiserror::Error;
use uuid::Uuid;
use crate::api::contracts;
use crate::entities::User;
use super::Service;

impl Service {
    pub fn register(&self, request: contracts::RegisterUserRequest) -> Result<User, ServiceError> {
        let password_hash_result = hash_password(request.password);

        let password_hash = match password_hash_result {
            Ok(p) => p,
            Err(e) => {
                error!("{}", e);
                return Err(ServiceError::Internal(e));
            }
        };

        let to_insert = User {
            id: Uuid::new_v4(),
            name: request.name,
            email: request.email,
            password_hash,
        };

        self.repo.insert_user(&self.db_pool, to_insert).map_err(|e| match e.source() {
            Some(source) if source.downcast_ref::<diesel::result::Error>()
                .map(|err| matches!(err, diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)))
                .unwrap_or(false) => ServiceError::EmailAlreadyExists,
            _ => {
                error!("{}", e);
                ServiceError::Internal(e)
            }
        })
    }

    pub fn login(&self, request: contracts::LoginUserRequest) -> Result<(), AuthError> {
        if let Some(hash_bytes) = self.repo.get_stored_credentials(request.email, &self.db_pool)? {
            let is_matching = compare_hash_and_password(request.password, hash_bytes)?;
            if !is_matching {
                return Err(AuthError::InvalidCredentials);
            }

            return Ok(());
        }

        Err(AuthError::InvalidCredentials)
    }

    pub fn get_users(&self) -> Result<Vec<User>, anyhow::Error> {
        self.repo.get_users(&self.db_pool)
    }
}

fn hash_password(password: String) -> Result<Vec<u8>, anyhow::Error> {
    let hashed = bcrypt::hash(password, 12)
        .context("failed to hash user's password")?;
    Ok(hashed.as_bytes().to_vec())
}

fn compare_hash_and_password(password: String, hash_bytes: Vec<u8>) -> Result<bool, anyhow::Error> {
    let hash_as_str = std::str::from_utf8(&hash_bytes)
        .context("failed to convert hash bytes to string")?;
    let verification_result = bcrypt::verify(password, hash_as_str)
        .context("failed to verify hash with password")?;
    Ok(verification_result)
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error(transparent)]
    Internal(#[from] anyhow::Error)
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid credentials supplied")]
    InvalidCredentials,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}