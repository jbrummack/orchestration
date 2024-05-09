use serde::{Deserialize, Serialize};
use sqlx::FromRow;
/*
{
    "task_result": String,
    "result_code": u32,
    "id": u32,
}

*/
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResult {
    pub task_result: String,
    pub result_code: u32,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTask {
    pub amount: i64,
    pub assigned_worker: i64,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct GetTaskResponse {
    pub input: String, //task_input
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InferenceJob {
    pub link: String,
    pub usr_id: u64,
}
