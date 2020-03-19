use conduit_primitives::users::repos::UsersRepo;
use roa::preload::*;
use roa::{App, Context, Result};

async fn index(mut ctx: Context<()>) -> Result {
    ctx.resp_mut().write("Hello world from roa");
    Ok(())
}

pub async fn start_app(repo: impl UsersRepo) -> std::io::Result<()> {
    let user = repo.get_by_id(1).await;
    println!("user: {:?}", user);

    let mut app = App::new(());
    app.end(index);
    if let Err(err) = app.listen("127.0.0.1:8080", |_| {})?.await {
        eprintln!("Server error: {}", err);
    }
    Ok(())
}
