mod data;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Json, Router};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use data::QuoteStore;
use crate::data::{Pagination};

type Db = Arc<RwLock<QuoteStore>>;

#[tokio::main]
async fn main() {
    let info_file = rolling::daily("./logs", "info")
        .with_max_level(tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "quote_axum=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer()
            .with_writer(info_file)
            .with_ansi(false)
        )
        .init();

    // QuoteStore::get_random_quote(&Default::default(), 0);
    let db = Db::default();
    db.write().await.add_quotes().await;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .nest("/v1", Router::new()
            .route("/quotes", get(get_quotes))
            .route("/random-quote", get(get_random_quote)),
        )
        .with_state(db)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)))
        .layer(cors);

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
