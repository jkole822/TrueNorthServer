use crate::models::{Decision, DecisionInput, LoginInput, RegisterInput, UpdateDecisionInput};
use crate::resolvers::{AuthMutation, DecisionMutation, DecisionQuery};
use async_graphql::{Context, EmptySubscription, Result, Schema};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Default)]
pub struct QueryRoot;

#[derive(Default)]
pub struct MutationRoot;

use async_graphql::Object;

#[Object]
impl MutationRoot {
    async fn create_decision(&self, ctx: &Context<'_>, input: DecisionInput) -> Result<bool> {
        DecisionMutation.create_decision(ctx, input).await
    }

    async fn delete_decision(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        DecisionMutation.delete_decision(ctx, id).await
    }

    async fn update_decision(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateDecisionInput,
    ) -> Result<bool> {
        DecisionMutation.update_decision(ctx, id, input).await
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        AuthMutation.login(ctx, input).await
    }

    async fn register_user(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<bool> {
        AuthMutation.register_user(ctx, input).await
    }
}

#[Object]
impl QueryRoot {
    async fn decisions(&self, ctx: &Context<'_>) -> Result<Vec<Decision>> {
        DecisionQuery.decisions(ctx).await
    }

    async fn decision_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Decision>> {
        DecisionQuery.decision_by_id(ctx, id).await
    }
}
