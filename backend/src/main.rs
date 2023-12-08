use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web_lab::middleware::from_fn;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use env_logger::Env;
use log::info;
use backend::{
    api::routes::register,
    configuration,
    service::UserService,
};
use backend::api::routes::{get_users, login};
use backend::middleware::requires_authentication::requires_authentication;
use backend::service::{AuthOptions, AuthService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("main.rs - unable to read configuration file.");
    let database_url = configuration.database.get_connection_string();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder().build(manager).expect("main.rs - cannot build connection pool");

    let user_service = UserService::new(db_pool);
    let auth_service = AuthService::new(AuthOptions {
        encoding_key: configuration.authentication.secret_key,
        audience: configuration.authentication.audience,
        token_expiration_in_seconds: configuration.authentication.token_expiration_in_seconds
    });

    info!("starting server on {}:{}", configuration.application.host, configuration.application.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("users")
                            .wrap(from_fn(requires_authentication))
                            .route("/", web::get().to(get_users))
                    )
                    .service(
                        web::scope("auth")
                            .route("/register", web::post().to(register))
                            .route("/login", web::post().to(login))
                    )

            )
            .app_data(Data::new(user_service.clone()))
            .app_data(Data::new(auth_service.clone()))
    })
        .bind((configuration.application.host, configuration.application.port))?
        .run()
        .await
}