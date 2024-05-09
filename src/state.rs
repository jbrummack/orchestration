pub mod models;
pub mod storage;

//use crate::models::TaskRequest;
//use crate::state::models::TaskRequest;
//use crate::state:

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub struct AppState {
    pub storage: SQLiteBackend,
}

#[derive(Debug, Clone)]
pub struct SQLiteBackend {
    conn: SqlitePool,
}

impl SQLiteBackend {
    pub async fn new() -> Self {
        let sqlite_pool = SqlitePool::connect(DB_URL).await.unwrap();
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("⚙️ Creating database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("✅ Created db successfully"),
                Err(error) => panic!("🚨 {}", error),
            }
        } else {
            println!("✅ Database already exists ");
        }
        let dbw = SQLiteBackend {
            conn: sqlite_pool.clone(),
        };
        println!("{}", dbw.setup_task().await);

        dbw
    }
}
