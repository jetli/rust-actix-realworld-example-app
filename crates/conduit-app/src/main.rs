#[cfg(feature = "actix_web")]
use conduit_actix_web::start_app;

#[cfg(feature = "actix_web")]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    start_app().await
}
