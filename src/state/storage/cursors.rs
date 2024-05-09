use crate::state::*;
impl SQLiteBackend {
    pub async fn setup_cursors(&self) -> String {
        let res = sqlx::query(
            "CREATE TABLE
             IF NOT EXISTS cursors (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
                name TEXT NOT NULL,
                position INTEGER,
            );",
        )
        .execute(&self.conn.clone())
        .await
        .unwrap();
        format!("✅ Initialized tasks: {:?}", res)
    }
}
