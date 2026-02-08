use crate::{
    error::{AppError, AppResult},
    models::{CreateTaskRequest, Task, UpdateTaskRequest},
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn list_tasks(State(state): State<AppState>) -> AppResult<Json<Vec<Task>>> {
    let tasks = state.tasks.read().await;
    let task_list: Vec<Task> = tasks.values().cloned().collect();
    Ok(Json(task_list))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Task>> {
    let tasks = state.tasks.read().await;
    tasks.get(&id).cloned().map(Json).ok_or(AppError::NotFound)
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> AppResult<(StatusCode, Json<Task>)> {
    // By default rust axum returns 200, here for creation we need to return instead of 200.
    // that's why we explicitly need to mention StatusCode in AppResult
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".into()));
    }

    let task = Task::new(payload.title, payload.description);
    let mut taks = state.tasks.write().await;
    taks.insert(task.id, task.clone());

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> AppResult<Json<Task>> {
    let mut tasks = state.tasks.write().await;

    let task = tasks.get_mut(&id).ok_or(AppError::NotFound)?;

    if let Some(title) = payload.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("Title cannot be empty".to_string()));
        }
        task.title = title;
    }

    if let Some(description) = payload.description {
        task.description = Some(description);
    }

    if let Some(completed) = payload.completed {
        task.completed = completed;
    }

    task.updated_at = Some(chrono::Utc::now());

    Ok(Json(task.clone()))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let mut tasks = state.tasks.write().await;
    tasks.remove(&id).ok_or(AppError::NotFound)?;
    Ok(StatusCode::NO_CONTENT)
}
