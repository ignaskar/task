use super::Repository;
use crate::models::user::User;
use crate::schema::users;
use crate::schema::users::{email, id, password_hash};
use anyhow::Context;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{
    ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use log::error;
use r2d2::PooledConnection;
use uuid::Uuid;

impl Repository {
    pub fn insert_user(
        &self,
        db_pool: &Pool<ConnectionManager<PgConnection>>,
        to_insert: User,
    ) -> Result<User, anyhow::Error> {
        let mut conn = get_connection_from_pool(db_pool)?;

        let insert_result = diesel::insert_into(users::table)
            .values(&to_insert)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|diesel_error| {
                let err = log_error_with_context(diesel_error);
                anyhow::Error::from(err)
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

    pub fn get_stored_credentials(
        &self,
        email_: String,
        db_pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Option<(Uuid, Vec<u8>)>, anyhow::Error> {
        let mut conn = get_connection_from_pool(db_pool)?;

        let get_result = users::table
            .select((id, password_hash))
            .filter(email.eq(email_))
            .first::<(Uuid, Vec<u8>)>(&mut conn)
            .optional()
            .map_err(log_error_with_context)
            .context("failed to retrieve password hash from DB");

        match get_result {
            Ok(maybe_data) => Ok(maybe_data),
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    pub fn get_users(
        &self,
        db_pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<User>, anyhow::Error> {
        let mut conn = get_connection_from_pool(db_pool)?;

        let get_result = users::table
            .select(User::as_select())
            .load(&mut conn)
            .map_err(log_error_with_context)
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

fn get_connection_from_pool(
    db_pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, anyhow::Error> {
    let conn_result = db_pool
        .get()
        .context("failed to get a connection from DB pool");

    match conn_result {
        Ok(c) => Ok(c),
        Err(e) => {
            error!("{}", e.to_string());
            Err(e)
        }
    }
}

fn log_error_with_context(error: diesel::result::Error) -> diesel::result::Error {
    error!("{}", error);
    error
}
