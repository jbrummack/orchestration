use crate::state::models::tasks::GetTask;
use crate::state::AppState;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/task_test")]
pub async fn readonly_task_amount(
    data: web::Data<AppState>,
    post: web::Json<GetTask>,
) -> impl Responder {
    let assigned_worker = post.assigned_worker;
    let amount = post.amount;
    let db = data.storage.fetch_task(amount, assigned_worker).await;
    //HttpResponse::Ok().body(web::Json(db))
    web::Json(db)
}
