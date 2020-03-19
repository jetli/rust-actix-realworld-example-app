use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "actix_web")] {
        use conduit_actix_web::start_app;
        use actix_rt::main as runtime;
    }
}

cfg_if! {
    if #[cfg(feature = "tide")] {
        use conduit_tide::start_app;
        use async_std::main as runtime;
    }
}

cfg_if! {
    if #[cfg(feature = "warp")] {
        use conduit_warp::start_app;
        use tokio::main as runtime;
    }
}

cfg_if! {
    if #[cfg(feature = "hyper")] {
        use conduit_hyper::start_app;
        use tokio::main as runtime;
    }
}

cfg_if! {
    if #[cfg(feature = "roa")] {
        use conduit_roa::start_app;
        use async_std::main as runtime;
    }
}

cfg_if! {
    if #[cfg(feature = "diesel")] {
        use conduit_diesel::users::repos::UsersRepo;
    }
}

cfg_if! {
    if #[cfg(feature = "sqlx")] {
        use conduit_sqlx::users::repos::UsersRepo;
    }
}

#[runtime]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let repo = UsersRepo::new();
    start_app(repo).await
}
