use crate::state::models::tasks::TaskResult;
use crate::state::*;
use actix_web::{put, web};

//Result IDs:
//0 |> Not done yet
//1 |> Success
#[put("/result")]
pub async fn put_result(put: web::Json<TaskResult>, data: web::Data<AppState>) -> String {
    data.storage
        .add_result(put.task_result.to_owned(), put.result_code, put.id)
        .await
}
/*
{
    "task_result": String,
    "result_code": u32,
    "id": u32,
}
*/

//curl -d '{"task_result":"hello_world i guess","result_code":1,"id":20}' -H "Content-Type: application/json" -X PUT http://localhost:8080/task
