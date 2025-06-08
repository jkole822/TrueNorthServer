use crate::models::{AuthUser, Decision, DecisionInput, DecisionRow};
use async_graphql::{Context, Error, ErrorExtensions, Object, Result};
use chrono::Utc;
use reqwest::Client;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub struct DecisionQuery;

#[Object]
impl DecisionQuery {
    pub async fn decisions(&self, ctx: &Context<'_>) -> Result<Vec<Decision>> {
        let user = ctx
            .data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;
        let pool = ctx.data::<PgPool>()?;

        let decision_rows =
            sqlx::query_as::<_, DecisionRow>("SELECT * FROM decisions WHERE user_id = $1")
                .bind(user.id)
                .fetch_all(pool)
                .await?;

        Ok(decision_rows
            .into_iter()
            .map(|decision_row| {
                Ok(Decision {
                    id: decision_row.id.to_string(),
                    answer: decision_row.answer,
                    category: decision_row.category,
                    desired_outcome: decision_row.desired_outcome,
                    emotions: decision_row.emotions,
                    question: decision_row.question,
                    user_id: decision_row.user_id.to_string(),
                    created_at: decision_row.created_at,
                })
            })
            .collect::<Result<Vec<Decision>, Error>>()?)
    }

    pub async fn decision_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Decision>> {
        let user = ctx
            .data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;
        let pool = ctx.data::<PgPool>()?;

        let uuid = Uuid::parse_str(&id).map_err(|_| {
            Error::new("Invalid UUID format").extend_with(|_, e| e.set("field", "id"))
        })?;

        let decision_row = sqlx::query_as::<_, DecisionRow>(
            "SELECT * FROM decisions WHERE id = $1 and user_id = $2",
        )
        .bind(uuid)
        .bind(user.id)
        .fetch_optional(pool)
        .await?;

        Ok(decision_row.map(|decision| Decision {
            id: decision.id.to_string(),
            answer: decision.answer,
            category: decision.category,
            desired_outcome: decision.desired_outcome,
            emotions: decision.emotions,
            question: decision.question,
            user_id: decision.user_id.to_string(),
            created_at: decision.created_at,
        }))
    }
}

pub struct DecisionMutation;

#[Object]
impl DecisionMutation {
    pub async fn create_decision(&self, ctx: &Context<'_>, input: DecisionInput) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx
            .data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;

        let mut context_parts = vec![];

        let category = input.category;
        if let Some(category) = &category {
            context_parts.push(format!("Category: {}.", category));
        }

        let emotions = input.emotions;
        if let Some(emotions) = &emotions {
            context_parts.push(format!("Current emotional state: {}.", emotions.join(", ")));
        }

        let desired_outcome = input.desired_outcome;
        if let Some(outcome) = &desired_outcome {
            context_parts.push(format!(
                "What I hope to feel or achieve: {}.",
                outcome.trim()
            ));
        }

        let context_string = if !context_parts.is_empty() {
            format!("{}\n", context_parts.join("\n"))
        } else {
            String::new()
        };

        let question = input.question.trim();
        if question.is_empty() {
            return Err(Error::new("Question cannot be empty")
                .extend_with(|_, e| e.set("field", "question")));
        }

        let prompt = format!(
            "{}Provide a one paragraph answer to this question:\n{}",
            context_string, question
        );

        let openai_api_key =
            std::env::var("OPENAI_API_KEY").map_err(|_| Error::new("Missing OpenAI API Key"))?;

        let client = Client::new();
        let openai_res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(openai_api_key)
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    { "role": "user", "content": prompt }
                ],
                "temperature": 0.7
            }))
            .send()
            .await
            .map_err(|e| {
                Error::new("Failed to contact OpenAI")
                    .extend_with(|_, ext| ext.set("source", format!("{}", e)))
            })?;

        let openai_json: serde_json::Value = openai_res.json().await.map_err(|e| {
            Error::new("Invalid OpenAI response")
                .extend_with(|_, ext| ext.set("source", format!("{}", e)))
        })?;

        let answer = openai_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No answer received")
            .trim()
            .to_string();

        sqlx::query(
            "INSERT INTO decisions (id, answer, category, desired_outcome, emotions, question, user_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(Uuid::new_v4())
        .bind(answer)
        .bind(category)
        .bind(desired_outcome)
        .bind(emotions)
        .bind(question)
        .bind(user.id)
        .bind(Utc::now())
        .execute(pool)
        .await.map_err(|e| {
            Error::new("Failed to execute mutation").extend_with(|_, ext| ext.set("error", e.to_string()))
        })?;

        Ok(true)
    }

    pub async fn delete_decision(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        ctx.data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;

        let uuid = Uuid::parse_str(&id).map_err(|_| {
            Error::new("Invalid UUID format").extend_with(|_, e| e.set("field", "id"))
        })?;

        sqlx::query("DELETE FROM decisions WHERE id = $1")
            .bind(uuid)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                Error::new("Failed to execute mutation")
                    .extend_with(|_, ext| ext.set("error", e.to_string()))
            })?;

        Ok(true)
    }
}
