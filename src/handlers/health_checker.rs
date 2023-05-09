use axum::{response::IntoResponse, Json};


pub async fn health_checker() -> impl IntoResponse {
    const MESSAGE: &str = "Rust blog server";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)

}
