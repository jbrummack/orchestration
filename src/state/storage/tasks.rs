use crate::state::models::tasks::GetTask;
use crate::state::models::tasks::GetTaskResponse;

use crate::state::*;

use serde::Serialize;
use serde_json::map;
use sqlx::{migrate::MigrateDatabase, Execute, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

impl SQLiteBackend {
    //tasks.result_id
    //0 |> Pending
    //1 |> Error
    pub async fn setup_task(&self) -> String {
        let res = sqlx::query(
            "CREATE TABLE
             IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
                input TEXT,
                result_id INTEGER,
                assigned_worker INTEGER,
                result TEXT
            );",
        )
        .execute(&self.conn.clone())
        .await
        .unwrap();
        format!("✅ Initialized tasks: {:?}", res)
    }

    pub async fn fetch_task(&self, amount: i64, assigned_worker: i64) -> Vec<GetTaskResponse> {
        //println!("{assigned_worker}");
        sqlx::query_as::<_, GetTaskResponse>(
            "SELECT input, id
                 FROM tasks
                 WHERE result_id = 0
                 AND assigned_worker = ?
                 LIMIT ?
            ;",
        )
        .bind(assigned_worker)
        .bind(amount)
        .fetch_all(&self.conn.clone())
        .await
        .unwrap()
    }
    pub async fn add_task(&self, task: String) -> String {
        let res = sqlx::query(
            "INSERT INTO tasks
                 (input, result_id, assigned_worker, result)
                 VALUES
                 (?, 0, 0, '')
            ;",
        )
        .bind(task)
        .execute(&self.conn.clone())
        .await
        .unwrap();
        format!("{:?}", res)
    }
    pub async fn add_result(&self, task_result: String, result_code: u32, id: u32) -> String {
        let res = sqlx::query(
            "UPDATE tasks SET
                result_id = ?,
                result = ?
            WHERE id = ?
            ;",
        )
        .bind(result_code)
        .bind(task_result)
        .bind(id)
        .execute(&self.conn.clone())
        .await
        .unwrap();
        format!("{:?}", res)
    }
}
