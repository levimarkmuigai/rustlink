use axum::{
    routing::{get, post},
    Router,
};
use dotenvy;
use rustlink::application::service::LinkService;
use rustlink::application::usecase::{LinkPersistenceService, LinkQueryService};
use rustlink::infrastructure::handlers::{create_link, delete_link, redirect_link, AppState};
use rustlink::infrastructure::repository::PgPoolRepository;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("FATAL: DATABASE_URL NOT SET");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("FATAL: FAILED TO CONNECT TO DATABASE");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("FATAL: FAILED TO RUN MIGRATION");

    type P = PgPoolRepository;
    type Q = PgPoolRepository;
    type RealService = LinkService<P, Q>;
    type RealState = AppState<P, Q>;

    let repo = PgPoolRepository::new(pool);

    let link_service_persistence = LinkPersistenceService::new(repo.clone());
    let link_service_query = LinkQueryService::new(repo);

    let link_service = RealService::new(link_service_persistence, link_service_query).await;

    let state = RealState {
        link_service: Arc::new(link_service),
    };

    let app = Router::new()
        .route("/links", post(create_link))
        .route("/l/:code", get(redirect_link))
        .route("/links/:id/delete", post(delete_link))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    println!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
