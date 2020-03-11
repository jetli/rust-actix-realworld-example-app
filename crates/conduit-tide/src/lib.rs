use conduit_primitives::users::repos::UsersRepo;
use tide::Request;

async fn index(_req: Request<()>) -> String {
    String::from("Hello world from tide")
}

pub async fn start_app(repo: impl UsersRepo) -> std::io::Result<()> {
    let user = repo.get_by_id(123).await.unwrap();
    println!("user: {:?}", user);

    let mut app = tide::new();
    app.at("/").get(index);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
