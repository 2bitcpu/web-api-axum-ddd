use crate::models::dtos::{
    auth::{SigninDto, SignupDto},
    member::OptionAuthMemberDto,
};
use crate::use_cases::{Modules, ModulesExt};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn signup(
    State(modules): State<Arc<Modules>>,
    Json(dto): Json<SignupDto>,
) -> impl IntoResponse {
    let result = modules.auth().signup(dto).await;
    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": err.to_string()})),
        )
            .into_response(),
    }
}

pub async fn signin(
    State(modules): State<Arc<Modules>>,
    Json(dto): Json<SigninDto>,
) -> impl IntoResponse {
    let result = modules.auth().signin(dto).await;
    match result {
        Ok(token) => (StatusCode::OK, Json(serde_json::json!({"token": token}))).into_response(),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"message": err.to_string()})),
        )
            .into_response(),
    }
}

pub async fn signout(
    option_autn_member: OptionAuthMemberDto,
    State(modules): State<Arc<Modules>>,
) -> impl IntoResponse {
    if option_autn_member.auth_member.is_none() {
        return StatusCode::OK.into_response();
    }
    let auth_member = option_autn_member.auth_member.unwrap();

    let _ = modules.auth().signout(&auth_member.account).await;

    StatusCode::OK.into_response()
}
