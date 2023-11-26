use anyhow::Context;
use log::error;
use uuid::Uuid;
use crate::api::contracts;
use crate::entities::User;
use super::Service;

impl Service {
    pub fn register_user(&self, request: contracts::RegisterUserRequest) -> Result<User, anyhow::Error> {
        let password_hash_result = hash_password(request.password);

        let password_hash = match password_hash_result {
            Ok(p) => p,
            Err(e) => {
                error!("{}", e);
                return Err(e)
            }
        };

        let to_insert = User {
            id: Uuid::new_v4(),
            name: request.name,
            email: request.email,
            password_hash
        };

        let user = self.repo.insert_user(&self.db_pool, to_insert)?;

        Ok(user)
    }
}

fn hash_password(password: String) -> Result<Vec<u8>, anyhow::Error> {
    let hashed = bcrypt::hash(password, 12)
        .context("failed to hash user's password")?;
    Ok(hashed.as_bytes().to_vec())
}