use crate::state::models::tasks::GetTask;
use crate::state::AppState;

use actix_web::{get, web, Responder};

#[get("/task_pending")]
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

//curl -d '{"amount":1,"assigned_worker":20}' -H "Content-Type: application/json" -X GET http://localhost:8080/task_test
//pub amount: i64,
//pub assigned_worker: i64,
