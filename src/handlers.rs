pub mod auth;
pub mod content;

use crate::commons::{
    config::{CORS_ORIGINS, SERVE_DIR},
    types::DbPool,
};
use crate::handlers::{auth as auth_handler, content as content_handler};
use crate::middlewares::auth::{auth_middleware, option_auth_middleware};
use crate::use_cases::Modules;
use axum::{
    Router,
    http::{HeaderValue, Method},
    middleware::from_fn_with_state,
    routing::{any, get, get_service, post},
};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub fn create_handlers(pool: DbPool) -> Router {
    let module = Arc::new(Modules::new(pool));

    let auth_handler = Router::new()
        .route("/signin", post(auth_handler::signin))
        .route("/signup", post(auth_handler::signup));

    let option_auth_handler = Router::new()
        .route("/signout", any(auth_handler::signout))
        .route_layer(from_fn_with_state(module.clone(), option_auth_middleware));

    let auth_handler = auth_handler.merge(option_auth_handler);

    let content_handler = Router::new()
        .route("/post", post(content_handler::post))
        .route("/get/{content_id}", get(content_handler::get))
        .route("/edit", post(content_handler::edit))
        .route("/remove/{content_id}", get(content_handler::remove))
        .route_layer(from_fn_with_state(module.clone(), auth_middleware));

    let api = Router::new()
        .nest("/auth", auth_handler)
        .nest("/contents", content_handler)
        .with_state(module);

    let api = match &*CORS_ORIGINS {
        Some(origins) => match origins.len() {
            0 => api,
            _ => {
                let cors = CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin(
                        origins
                            .iter()
                            .map(|s| s.parse::<HeaderValue>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .allow_credentials(true);
                tracing::info!("CORS enabled {:?}", origins);
                api.layer(cors)
            }
        },
        None => api,
    };

    match &*SERVE_DIR {
        Some(dir) => Router::new()
            .nest("/service", api)
            .fallback(get_service(ServeDir::new(dir))),
        None => Router::new().nest("/service", api),
    }
}
