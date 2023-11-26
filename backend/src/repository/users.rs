use anyhow::Context;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool};
use log::error;
use crate::entities::User;
use super::Repository;
use crate::schema::users;

impl Repository {
    pub fn insert_user(&self, db_pool: &Pool<ConnectionManager<PgConnection>>, to_insert: User) -> Result<User, anyhow::Error> {
        let conn_result = db_pool.get()
            .context("failed to get a connection from DB pool");

        let mut conn = match conn_result {
            Ok(c) => c,
            Err(e) => {
                error!("{}", e);
                return Err(e);
            }
        };

        let insert_result = diesel::insert_into(users::table)
            .values(&to_insert)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|e| {
                error!("{}", e);
                e
            })
            .context("failed to insert new user");

        let user = match insert_result {
            Ok(u) => u,
            Err(e) => {
                error!("{}", e);
                return Err(e);
            }
        };

        Ok(user)
    }
}