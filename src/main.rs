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

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
