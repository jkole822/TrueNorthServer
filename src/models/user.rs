use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(InputObject)]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub is_superuser: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub is_superuser: bool,
    pub created_at: DateTime<Utc>,
}
