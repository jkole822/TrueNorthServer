use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::{header, Method},
    response::Html,
    routing::{get, post},
    Extension, Router, Server,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod middleware;
mod models;
mod resolvers;
mod schema;

use crate::models::AuthUser;
use schema::{AppSchema, MutationRoot, QueryRoot};

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let pool = PgPool::connect(&db_url).await.unwrap();
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

    let cors = CorsLayer::new()
        .allow_origin(Any) // or .allow_origin("https://yourfrontend.com".parse().unwrap())
        .allow_methods([Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .route("/healthz", get(|| async { "OK" }))
        .layer(cors)
        .layer(Extension(schema))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphiql() -> Html<&'static str> {
    Html(include_str!("graphiql.html"))
}
