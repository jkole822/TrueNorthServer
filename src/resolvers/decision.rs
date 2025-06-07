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
        ctx.data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;
        let pool = ctx.data::<PgPool>()?;

        let decision_rows = sqlx::query_as::<_, DecisionRow>("SELECT * FROM decisions")
            .fetch_all(pool)
            .await?;

        Ok(decision_rows
            .into_iter()
            .map(|decision_row| {
                Ok(Decision {
                    id: decision_row.id.to_string(),
                    answer: decision_row.answer,
                    question: decision_row.question,
                    user_id: decision_row.user_id.to_string(),
                    created_at: decision_row.created_at,
                })
            })
            .collect::<Result<Vec<Decision>, Error>>()?)
    }

    pub async fn decision_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Decision>> {
        ctx.data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;
        let pool = ctx.data::<PgPool>()?;

        let uuid = Uuid::parse_str(&id).map_err(|_| {
            Error::new("Invalid UUID format").extend_with(|_, e| e.set("field", "id"))
        })?;

        let decision_row =
            sqlx::query_as::<_, DecisionRow>("SELECT * FROM decisions WHERE id = $1")
                .bind(uuid)
                .fetch_optional(pool)
                .await?;

        Ok(decision_row.map(|decision| Decision {
            id: decision.id.to_string(),
            answer: decision.answer,
            question: decision.question,
            user_id: decision.user_id.to_string(),
            created_at: decision.created_at,
        }))
    }
}

pub struct DecisionMutation;

#[Object]
impl DecisionMutation {
    pub async fn create_decision(
        &self,
        ctx: &Context<'_>,
        input: DecisionInput,
    ) -> Result<Decision> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx
            .data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;

        let question = input.question.trim();
        if question.is_empty() {
            return Err(Error::new("Question cannot be empty")
                .extend_with(|_, e| e.set("field", "question")));
        }

        let prompt = format!(
            "Provide a one sentence answer to this question:\n{}",
            question
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

        let id = Uuid::new_v4();
        let created_at = Utc::now();

        let query_result = sqlx::query(
            "INSERT INTO decisions (id, question, answer, user_id, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(&question)
        .bind(&answer)
        .bind(&user.id)
        .bind(&created_at)
        .execute(pool)
        .await;

        match query_result {
            Ok(_) => Ok(Decision {
                id: id.to_string(),
                answer: Some(answer),
                question: question.to_string(),
                user_id: user.id.to_string(),
                created_at,
            }),
            Err(e) => Err(
                Error::new("Failed to create decision").extend_with(|_, ext| {
                    ext.set("error", format!("{}", e));
                    ext.set("id", id.to_string());
                }),
            ),
        }
    }
}
