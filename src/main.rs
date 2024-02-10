mod data;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use data::QuoteStore;
use crate::data::{Pagination};

type Db = Arc<RwLock<QuoteStore>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "quote_axum=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // QuoteStore::get_random_quote(&Default::default(), 0);
    let db = Db::default();
    db.write().await.add_quotes().await;

    let app = Router::new()
        .nest("/v1", Router::new()
            .route("/quotes", get(get_quotes))
            .route("/random-quote", get(get_random_quote)),
        )
        .with_state(db)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_quotes(pagination: Option<Query<Pagination>>, State(db): State<Db>) -> impl IntoResponse {
    let quotes = db.read().await;
    let Query(pagination) = pagination.unwrap_or_default();
    Json(quotes.get_quotes(pagination))
}

async fn get_random_quote(pagination: Option<Query<Pagination>>,
                          State(db): State<Db>)
    -> impl IntoResponse {
    let quote = db.read().await;
    let Query(pagination) = pagination.unwrap_or_default();
    if let Some(item) = quote.get_random_quote(pagination) {
        Json(item).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Not found").into_response()
    }
}
