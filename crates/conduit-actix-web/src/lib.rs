use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    String::from("Hello world from actix web")
}

pub async fn start_app() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
