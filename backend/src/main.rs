use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use env_logger::Env;
use backend::{
    api::routes::register,
    configuration,
    service::Service
};
use backend::api::routes::get_users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("main.rs - unable to read configuration file.");
    let database_url = configuration.database.get_connection_string();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder().build(manager).expect("main.rs - cannot build connection pool");

    let service = Service::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("users")
                    .route("", web::get().to(get_users))
                    .route("register", web::post().to(register))
            )
            .app_data(Data::new(service.clone()))
    })
        .bind((configuration.application.host, configuration.application.port))?
        .run()
        .await
}