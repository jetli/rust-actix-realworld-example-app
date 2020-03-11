use actix_web::{get, App, HttpServer, Responder};
use conduit_primitives::users::repos::UsersRepo;

#[get("/")]
async fn index() -> impl Responder {
    String::from("Hello world from actix web")
}

pub async fn start_app(repo: impl UsersRepo) -> std::io::Result<()> {
    let user = repo.get_by_id(123).await.unwrap();
    println!("user: {:?}", user);

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
