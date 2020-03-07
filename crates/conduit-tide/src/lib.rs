use tide::Request;

async fn index(_req: Request<()>) -> String {
    String::from("Hello world from tide")
}

pub async fn start_app() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("/").get(index);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
