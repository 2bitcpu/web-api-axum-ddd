use tokio::net::TcpListener;
use web_api::commons::setup::initialize_db;
use web_api::commons::{
    config::{DB_URL, HOST_NAME},
    types::{BoxError, DbPool},
};
use web_api::handlers::create_handlers;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let pool: DbPool = initialize_db(&*DB_URL).await?;

    let app = create_handlers(pool);

    let listener = TcpListener::bind(&*HOST_NAME).await?;
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
