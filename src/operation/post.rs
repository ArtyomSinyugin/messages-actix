use actix_web::{web, Result};
use serde::{Serialize, Deserialize};
use super::AppState;

#[derive(Deserialize)]
pub struct PostInput {
    message: String,
}

#[derive(Serialize)]
pub struct PostResponse {
    server_id: usize, 
    request_count: usize,
    message: String,
}

pub async fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
// стр. 76
    let requst_count = state.request_count.get() + 1;
    state.request_count.set(requst_count);

    let mut ms = state.messages.lock().unwrap();
    ms.push(msg.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count: requst_count,
        message: msg.message.clone(),
    }))
}