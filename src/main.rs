use tokio::net::TcpListener;
use web_api::commons::setup::initialize_db;
use web_api::commons::{
    config::{DB_URL, HOST_NAME},
    setup,
    types::{BoxError, DbPool},
};
use web_api::handlers::create_handlers;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    setup::init_tracing();

    let pool: DbPool = initialize_db(&*DB_URL).await?;

    let app = create_handlers(pool);

    let listener = TcpListener::bind(&*HOST_NAME).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
