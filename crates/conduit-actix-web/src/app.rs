use actix_web::{App, HttpServer};
use conduit_primitives::users::repos::UsersRepo;

use crate::routes::index;

pub async fn start_app(repo: impl UsersRepo) -> std::io::Result<()> {
    let user = repo.get_by_id(1).await;
    println!("user: {:?}", user);

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
