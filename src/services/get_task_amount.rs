use crate::state::models::tasks::GetTask;
use crate::state::AppState;

use actix_web::{get, web, Responder};

#[get("/task")]
pub async fn get_task_amount(
    data: web::Data<AppState>,
    post: web::Json<GetTask>,
) -> impl Responder {
    let assigned_worker = post.assigned_worker;
    let amount = post.amount;
    data.storage
        .assign_worker_task(amount, assigned_worker)
        .await;

    let db = data.storage.fetch_task(amount, assigned_worker).await;

    web::Json(db)
}
