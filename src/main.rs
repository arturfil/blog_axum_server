mod config;
mod handlers;
mod router;

use std::{process::exit, env::var_os, sync::Arc};

use axum::http::{header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}, HeaderValue, Method};
use config::config::Config;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use tower_http::cors::CorsLayer;
use crate::router::router::create_router;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await

    {
        Ok(pool) => {
            println!("✅ Connection to database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            exit(1);
        }
    };


    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    }))
    .layer(cors);

    // let port: String = "0.0.0.0:" + var_os("PORT");

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
