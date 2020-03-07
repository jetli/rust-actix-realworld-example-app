use std::convert::Infallible;
use warp::Filter;

async fn index() -> Result<impl warp::Reply, Infallible> {
    Ok(String::from("Hello world from warp"))
}

pub async fn start_app() -> std::io::Result<()> {
    let routes = warp::any().and_then(index);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}
