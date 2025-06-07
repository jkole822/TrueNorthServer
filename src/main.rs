use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::Html,
    routing::{get, post},
    Extension, Router, Server,
};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

mod middleware;
mod models;
mod resolvers;
mod schema;

use crate::models::AuthUser;
use schema::{AppSchema, MutationRoot, QueryRoot};

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");
    let pool = SqlitePool::connect("sqlite:db.sqlite3").await.unwrap();
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();

    async fn graphql_handler(
        schema: Extension<AppSchema>,
        auth_user: Option<AuthUser>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        let mut request = req.into_inner();

        if let Some(user) = auth_user {
            request.data.insert(user);
        }

        schema.execute(request).await.into()
    }

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphiql() -> Html<&'static str> {
    Html(include_str!("graphiql.html"))
}
