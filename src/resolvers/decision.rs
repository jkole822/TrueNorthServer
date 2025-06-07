use crate::models::{AuthUser, Decision, DecisionInput, DecisionRow};
use async_graphql::{Context, Error, ErrorExtensions, Object, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct DecisionQuery;

fn parse_created_at(created_at: String) -> core::result::Result<DateTime<Utc>, Error> {
    created_at.parse::<DateTime<Utc>>().map_err(|e| {
        Error::new("Invalid datetime format").extend_with(|_, ext| ext.set("source", e.to_string()))
    })
}

#[Object]
impl DecisionQuery {
    pub async fn decisions(&self, ctx: &Context<'_>) -> Result<Vec<Decision>> {
        ctx.data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;
        let pool = ctx.data::<SqlitePool>()?;

        let decision_rows = sqlx::query_as::<_, DecisionRow>("SELECT * FROM decisions")
            .fetch_all(pool)
            .await?;

        let decisions = decision_rows
            .into_iter()
            .map(|decision_row| {
                let created_at = parse_created_at(decision_row.created_at)?;

                Ok(Decision {
                    id: decision_row.id,
                    answer: decision_row.answer,
                    question: decision_row.question,
                    user_id: decision_row.user_id,
                    created_at,
                })
            })
            .collect::<Result<Vec<Decision>, Error>>()?;

        Ok(decisions)
    }

    pub async fn decision_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Decision>> {
        ctx.data::<AuthUser>()
            .map_err(|_| Error::new("You must be logged in to perform this action"))?;
        let pool = ctx.data::<SqlitePool>()?;

        let decision_row = sqlx::query_as::<_, DecisionRow>("SELECT * FROM decisions WHERE id = ?")
            .bind(id.clone())
            .fetch_optional(pool)
            .await?;

        let decision = match decision_row {
            Some(decision_row) => {
                let created_at = parse_created_at(decision_row.created_at)?;

                Some(Decision {
                    id: decision_row.id,
                    answer: decision_row.answer,
                    question: decision_row.question,
                    user_id: decision_row.user_id,
                    created_at,
                })
            }
            None => None,
        };

        Ok(decision)
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
        let pool = ctx.data::<SqlitePool>()?;
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

        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now();

        let query_result = sqlx::query(
            "INSERT INTO decisions (id, question, answer, user_id, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
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
                id,
                answer: Some(answer),
                question: question.to_string(),
                user_id: String::new(),
                created_at,
            }),
            Err(e) => Err(
                Error::new("Failed to create decision").extend_with(|_, ext| {
                    ext.set("error", format!("{}", e));
                    ext.set("id", id);
                }),
            ),
        }
    }
}
