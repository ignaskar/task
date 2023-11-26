use actix_web::{App, HttpServer, web};
use backend::api::routes::register;
use backend::configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("main.rs - unable to read configuration file.");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("api/users")
                    .route("register", web::post().to(register))
            )
    })
        .bind((configuration.application.host, configuration.application.port))?
        .run()
        .await
}