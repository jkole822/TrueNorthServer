use crate::models::{AuthUser, Claims};
use async_graphql::Result;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let auth_header = headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                String::from("Missing authorization header"),
            ))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid auth header format".into(),
            ));
        }

        let token = &auth_header[7..];
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| (StatusCode::UNAUTHORIZED, "JWT_SECRET missing".into()))?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

        Ok(AuthUser {
            id: token_data.claims.sub,
            is_superuser: token_data.claims.is_superuser,
        })
    }
}
