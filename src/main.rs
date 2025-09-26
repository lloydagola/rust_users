mod models;
mod repository;
mod service;
mod handlers;

use axum::{Router, routing::{get, post, put, delete}, extract::State};
use sqlx::SqlitePool;
use std::new::SocketAddr;
use dotenvy::dotenv;
use crate::repository::UserRepository;
use crate::service::UserService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = std::env:var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users(
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL  
        );"
    )
    .execute(&pool)
    .await?;

    let repo = UserRepository::new(pool);
    let service = UserService::new(repo);

    let addr = SocketAddr::from (([127, 0, 0, 1], 8080));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}