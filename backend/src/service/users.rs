use std::sync::Arc;
use log::error;
use thiserror::Error;
use uuid::Uuid;
use crate::api::contracts;
use crate::models::user::User;
use super::{AuthService, UserService};

impl UserService {
    pub fn register(&self, auth_service: Arc<AuthService>, request: contracts::RegisterUserRequest) -> Result<User, ServiceError> {
        let password_hash = match auth_service.hash_password(request.password) {
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
                .is_some_and(|err| matches!(err, diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _))) => ServiceError::EmailAlreadyExists,
            _ => {
                error!("{}", e);
                ServiceError::Internal(e)
            }
        })
    }

    pub fn login(&self, auth_service: Arc<AuthService>, request: contracts::LoginUserRequest) -> Result<String, AuthError> {
        if let Some((user_id, hash_bytes)) = self.repo.get_stored_credentials(request.email, &self.db_pool)? {
            let is_matching = auth_service.compare_hash_and_password(request.password, hash_bytes)?;
            if !is_matching {
                return Err(AuthError::InvalidCredentials);
            }

            let token = auth_service.generate_token(user_id)?;

            return Ok(token);
        }

        Err(AuthError::InvalidCredentials)
    }

    pub fn get_users(&self) -> Result<Vec<User>, anyhow::Error> {
        self.repo.get_users(&self.db_pool)
    }
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