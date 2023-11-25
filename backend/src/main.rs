use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use backend::configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("main.rs - unable to read configuration file.");

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind((configuration.application.host, configuration.application.port))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}