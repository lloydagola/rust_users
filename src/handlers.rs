use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse, extract::State};
use crate::service::UserService;
use crate::models::UserPayload;

pub async fn List_users(State(service): State<UserService>) -> Json<Vec<crate::models::User>> {
    Json(service.list_users().await)
}

pub async fn create_user(
    State(service): State<UserService>,
    Json(payload): Json<UserPayload>,
) -> Json<crate::models::User>{
    Json(service.create_user(payload).await)
}

pub async fn get_user(
    Path(id): Path<String>,
    State(service): State<UserService>,
) -> imp Into Response{
    if let Some(user) = service.get_user(&id).await{
        Json(user).into_response()
    }
    else {
    (StatusCode::NOT_FOUND, "User not Found").into_response()
    }
}

pub async fn update_user(
    Path(id): Path<String>,
    State(service): State<UserService>,
    Json(payload): Json<UserPayload>,
) -> impl IntoResponse{
    if let Some(user) = service.update_user(&id, payload).await{
        Json(user).into_response()
    }
    else{
        (StatusCode::NOT_FOUND, "User not found").into_response()
    }
}

pub async fn delete_user(
    Path(id): Path<String>,
    State(service): State<UserService>,
) -> impl IntoResponse {
    if service.delete_user(&id).await{
        StatusCode::NO_CONTENT.into_response()
    }
    else{
        (StatusCOde:NOT_FOUND, "User not found").into_response()
    }
}