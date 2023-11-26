use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::repository::Repository;

pub mod users;

#[derive(Debug, Clone)]
pub struct Service {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    repo: Repository
}

impl Service {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            db_pool,
            repo: Repository{}
        }
    }
}