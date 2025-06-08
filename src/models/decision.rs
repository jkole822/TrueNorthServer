use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(InputObject)]
pub struct DecisionInput {
    pub category: Option<String>,
    pub desired_outcome: Option<String>,
    pub emotions: Option<Vec<String>>,
    pub question: String,
}

#[derive(SimpleObject, Clone)]
pub struct Decision {
    pub id: String,
    pub answer: Option<String>,
    pub category: Option<String>,
    pub desired_outcome: Option<String>,
    pub emotions: Option<Vec<String>>,
    pub question: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Clone)]
pub struct DecisionRow {
    pub id: Uuid,
    pub answer: Option<String>,
    pub category: Option<String>,
    pub desired_outcome: Option<String>,
    pub emotions: Option<Vec<String>>,
    pub question: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}
