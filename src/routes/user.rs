use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;
use sqlx::PgPool;

use crate::models::user::User;

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "Missing 'name'".into()))?;

    match User::create(&pool, name).await {
        Ok(user) => {
            let body = json!({ "id": user.id, "name": user.name });
            Ok((StatusCode::CREATED, Json(body)))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_user(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match User::get(&pool, id).await {
        Ok(Some(user)) => {
            let body = json!({ "id": user.id, "name": user.name });
            Ok(Json(body))
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found".into())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn update_user(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "Missing 'name'".into()))?;

    match User::update(&pool, id, name).await {
        Ok(user) => {
            let body = json!({ "id": user.id, "name": user.name });
            Ok(Json(body))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match User::delete(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn list_user(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match User::list(&pool).await {
        Ok(users) => {
            let body = json!(users.into_iter().map(|user| {
                json!({ "id": user.id, "name": user.name })
            }).collect::<Vec<_>>());
            Ok(Json(body))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}