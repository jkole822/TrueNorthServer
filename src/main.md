# Rust + Axum + async-graphql `main.rs` Review

This is a full breakdown of the `main.rs` file used in the backend of your project, which combines Axum (web server), async-graphql (GraphQL schema/resolvers), and SQLx (SQLite database).

---

## 🧠 File Overview

```rust
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::Html,
    routing::{get, post},
    Extension, Router, Server,
};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

mod models;
mod schema;
use schema::{AppSchema, MutationRoot, QueryRoot};
```

### What These Do:

* `Schema`, `EmptySubscription`: define your GraphQL schema types.
* `GraphQLRequest`, `GraphQLResponse`: request/response for GraphQL POSTs.
* `axum::*`: routing, server setup, and dependency injection via `Extension`.
* `SqlitePool`: the async SQLite database connection manager.
* `SocketAddr`: IP/port config.
* `mod schema` / `mod models`: your modules for schema logic and data types.

---

## 🔧 main Function

```rust
#[tokio::main]
async fn main() {
```

Starts the Tokio async runtime and enables use of `.await`.

```rust
    let pool = SqlitePool::connect("sqlite:db.sqlite3").await.unwrap();
```

Establishes a connection pool to a local SQLite file-based database.

```rust
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();
```

Creates the GraphQL schema and injects the database pool into its context so you can use it in your resolver functions.

```rust
    async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }
```

Defines the handler function that receives GraphQL POST requests, executes them using your schema, and returns a structured GraphQL response.

```rust
    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(Extension(pool));
```

Sets up the Axum router:

* `/` serves the GraphiQL playground
* `/graphql` accepts GraphQL POSTs
* `.layer(Extension(...))` injects shared data into the route handlers and resolvers

```rust
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
```

Starts the Axum/Hyper server and binds it to localhost on port 8080.

---

## 🎨 GraphiQL Handler

```rust
async fn graphiql() -> Html<&'static str> {
    Html(include_str!("graphiql.html"))
}
```

Serves a static HTML file for the GraphiQL UI so you can test your GraphQL queries and mutations in the browser.

---

## 🧩 Flow Summary

1. Server connects to SQLite and builds the GraphQL schema.
2. It exposes two routes:

    * `/`: loads GraphiQL (HTML UI)
    * `/graphql`: executes GraphQL queries/mutations
3. The database and schema are injected into handlers and resolvers.
4. You can interact with everything using the browser or tools like `curl`, Postman, or a Flutter frontend.
