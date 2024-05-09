//use crate::state::models::tasks::GetTaskResponse;
use crate::state::*;

impl SQLiteBackend {
    pub async fn setup_workers(&self) -> String {
        let res = sqlx::query(
            "CREATE TABLE
             IF NOT EXISTS workers (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                name TEXT UNIQUE,
                auth TEXT,
                privilege INTEGER
            );",
        )
        .execute(&self.conn.clone())
        .await
        .unwrap();
        format!("{:?}", res)
    }
    pub async fn assign_worker_task(&self, amount: i64, assigned_worker: i64) {
        let q = sqlx::query(
            "UPDATE tasks
                 SET assigned_worker = ?
                 WHERE result_id = 0
                 AND assigned_worker = 0

            ;",
        ) //LIMIT ?
        .bind(assigned_worker)
        .bind(amount)
        .fetch_all(&self.conn.clone())
        .await
        .unwrap();
        println!("assigned {} tasks", q.len());
        //println!("{:?}");
    }
}
