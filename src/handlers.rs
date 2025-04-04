pub mod auth;
pub mod content;

use crate::commons::types::DbPool;
use crate::handlers::{auth as auth_handler, content as content_handler};
use crate::middlewares::auth::{auth_middleware, option_auth_middleware};
use crate::use_cases::Modules;
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{any, get, post},
};
use std::sync::Arc;

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

    Router::new().nest(
        "/service",
        Router::new()
            .nest("/contents", content_handler)
            .nest("/auth", auth_handler)
            .with_state(module),
    )
}
