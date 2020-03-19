use conduit_primitives::users::repos::UsersRepo;
use std::convert::Infallible;
use warp::Filter;

async fn index() -> Result<impl warp::Reply, Infallible> {
    Ok(String::from("Hello world from warp"))
}

pub async fn start_app(repo: impl UsersRepo) -> std::io::Result<()> {
    let user = repo.get_by_id(1).await;
    println!("user: {:?}", user);

    let routes = warp::any().and_then(index);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}
