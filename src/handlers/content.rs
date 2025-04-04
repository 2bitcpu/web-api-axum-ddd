use crate::models::dtos::{content::ContentDto, member::AuthMemberDto};
use crate::use_cases::{Modules, ModulesExt};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn post(
    _autn_member: AuthMemberDto,
    State(modules): State<Arc<Modules>>,
    Json(dto): Json<ContentDto>,
) -> impl IntoResponse {
    let result = modules.content().post(dto).await;
    match result {
        Ok(entity) => (StatusCode::OK, Json(entity)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": err.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get(
    _autn_member: AuthMemberDto,
    State(modules): State<Arc<Modules>>,
    Path(content_id): Path<i64>,
) -> impl IntoResponse {
    let result = modules.content().get(content_id).await;
    match result {
        Ok(entity) => match entity {
            Some(entity) => (StatusCode::OK, Json(entity)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"message": "not found"})),
            )
                .into_response(),
        },
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": err.to_string()})),
        )
            .into_response(),
    }
}

pub async fn edit(
    _autn_member: AuthMemberDto,
    State(modules): State<Arc<Modules>>,
    Json(dto): Json<ContentDto>,
) -> impl IntoResponse {
    let result = modules.content().edit(dto).await;
    match result {
        Ok(entity) => (StatusCode::OK, Json(entity)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": err.to_string()})),
        )
            .into_response(),
    }
}

pub async fn remove(
    _autn_member: AuthMemberDto,
    State(modules): State<Arc<Modules>>,
    Path(content_id): Path<i64>,
) -> impl IntoResponse {
    let result = modules.content().remove(content_id).await;
    match result {
        Ok(count) => {
            if count == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"message": "not found"})),
                )
                    .into_response()
            } else {
                (StatusCode::OK).into_response()
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": err.to_string()})),
        )
            .into_response(),
    }
}
