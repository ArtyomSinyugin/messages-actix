use actix_web::{get, web, Result};
use serde::Serialize;
use super::AppState;

#[derive(Serialize)]
struct LookupResponse {
    server_id: usize,                
    request_count: usize,
    result: Option<String>,
}

#[get("/lookup/{index}")]  
async fn lookup(state: web::Data<AppState>, idx: web::Path<usize>) -> Result<web::Json<LookupResponse>> {
// стр. 87
    let requst_count = state.request_count.get() + 1;
    state.request_count.set(requst_count);

    let ms = state.messages.lock().unwrap();
    let result = ms.get(idx.into_inner()).cloned();

    Ok(web::Json(LookupResponse {
        server_id: state.server_id,
        request_count: requst_count,
        result,
    }))
}