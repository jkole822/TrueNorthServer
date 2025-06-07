use crate::models::{Claims, LoginInput, RegisterInput, UserRow};
use async_graphql::{Context, Error, ErrorExtensions, Object, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::SqlitePool;
use std::env;
use uuid::Uuid;

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    pub async fn register_user(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<bool> {
        let pool = ctx.data::<SqlitePool>()?;
        let hashed_password = hash(input.password, DEFAULT_COST).map_err(|e| {
            Error::new("Hashing failed").extend_with(|_, ext| ext.set("error", e.to_string()))
        })?;

        sqlx::query("INSERT INTO users (id, email, password, is_superuser, created_at) VALUES (?1, ?2, ?3, ?4, ?5)")
            .bind(Uuid::new_v4().to_string())
            .bind(input.email)
            .bind(hashed_password)
            .bind(false)
            .bind(chrono::Utc::now())
            .execute(pool)
            .await
            .map_err(|e| Error::new("DB insert failed").extend_with(|_, ext| ext.set("error", e.to_string())))?;

        Ok(true)
    }

    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let pool = ctx.data::<SqlitePool>()?;

        let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = ?")
            .bind(input.email)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                Error::new("Database error").extend_with(|_, ext| ext.set("source", e.to_string()))
            })?;

        let Some(user) = user else {
            return Err(Error::new("Validation failed"));
        };

        let is_valid = verify(&input.password, &user.password).map_err(|e| {
            Error::new("Validation failed").extend_with(|_, ext| ext.set("source", e.to_string()))
        })?;

        if !is_valid {
            return Err(Error::new("Invalid password"));
        }

        let expiration = Utc::now()
            .checked_add_signed(Duration::days(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            exp: expiration,
            is_superuser: user.is_superuser,
        };

        let secret = env::var("JWT_SECRET").map_err(|_| Error::new("JWT_SECRET must be set"))?;

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| {
            Error::new("Failed to encode token")
                .extend_with(|_, ext| ext.set("source", e.to_string()))
        })?;

        Ok(token)
    }
}
