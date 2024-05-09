use crate::state::models::tasks::GetTask;
use crate::state::AppState;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/task")]
pub async fn get_task_amount(data: web::Data<AppState>, post: web::Json<GetTask>) -> String {
    let assigned_worker = post.assigned_worker;
    let amount = post.amount;
    data.storage
        .assign_worker_task(amount, assigned_worker)
        .await;
    format!(
        "{:?}",
        data.storage.fetch_task(amount, assigned_worker).await
    )
}
