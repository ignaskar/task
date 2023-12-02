use anyhow::Context;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool};
use log::error;
use r2d2::PooledConnection;
use crate::entities::User;
use super::Repository;
use crate::schema::users;

impl Repository {
    pub fn insert_user(&self, db_pool: &Pool<ConnectionManager<PgConnection>>, to_insert: User) -> Result<User, anyhow::Error> {
        let mut conn = get_connection_from_pool(db_pool)?;

        let insert_result = diesel::insert_into(users::table)
            .values(&to_insert)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|e| {
                error!("{}", e);
                e
            })
            .context("failed to insert new user to DB");

        match insert_result {
            Ok(u) => Ok(u),
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    pub fn get_users(&self, db_pool: &Pool<ConnectionManager<PgConnection>>) -> Result<Vec<User>, anyhow::Error> {
        let mut conn = get_connection_from_pool(db_pool)?;

        let get_result = users::table
            .select(User::as_select())
            .load(&mut conn)
            .map_err(|e| {
                error!("{}", e);
                e
            })
            .context("failed to get users from DB");

        match get_result {
            Ok(us) => Ok(us),
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }
}

fn get_connection_from_pool(db_pool: &Pool<ConnectionManager<PgConnection>>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, anyhow::Error> {
    let conn_result = db_pool.get()
        .context("failed to get a connection from DB pool");

    match conn_result {
        Ok(c) => Ok(c),
        Err (e) => {
            error!("{}", e.to_string());
            Err(e)
        }
    }
}