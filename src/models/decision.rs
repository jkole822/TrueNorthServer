use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(InputObject)]
pub struct DecisionInput {
    pub question: String,
}

#[derive(SimpleObject, Clone)]
pub struct Decision {
    pub id: String,
    pub answer: Option<String>,
    pub question: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct DecisionRow {
    pub id: String,
    pub answer: Option<String>,
    pub question: String,
    pub user_id: String,
    pub created_at: String,
}
