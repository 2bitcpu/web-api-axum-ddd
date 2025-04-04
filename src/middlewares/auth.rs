use crate::models::dtos::member::{AuthMemberDto, OptionAuthMemberDto};
use crate::use_cases::Modules;
use axum::middleware::Next;
use axum::{
    RequestExt,
    extract::{FromRequestParts, Request, State},
    http::{StatusCode, request::Parts},
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::sync::Arc;

impl<S> FromRequestParts<S> for AuthMemberDto
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_member = parts
            .extensions
            .get::<Self>()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(auth_member.clone())
    }
}

pub async fn auth_middleware(
    State(module): State<Arc<Modules>>,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response> {
    let bearer = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let token = bearer.token();

    let auth_member = module
        .auth
        .authenticate(token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(auth_member.clone());

    Ok(next.run(request).await)
}

impl<S> FromRequestParts<S> for OptionAuthMemberDto
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_member = parts
            .extensions
            .get::<Self>()
            .unwrap_or(&OptionAuthMemberDto { auth_member: None });

        Ok(auth_member.clone())
    }
}

pub async fn option_auth_middleware(
    State(module): State<Arc<Modules>>,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response> {
    let bearer = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await;

    let auth_member = match bearer {
        Ok(bearer) => {
            let token = bearer.token();

            module
                .auth
                .authenticate(token)
                .await
                .map(|auth_member| OptionAuthMemberDto {
                    auth_member: Some(auth_member),
                })
                .unwrap_or(OptionAuthMemberDto { auth_member: None })
        }
        Err(_) => OptionAuthMemberDto { auth_member: None },
    };

    request.extensions_mut().insert(auth_member.clone());

    Ok(next.run(request).await)
}
