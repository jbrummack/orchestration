use crate::get_task_amount::get_task_amount;
use crate::put_result::put_result;
use crate::readonly_task_amount::readonly_task_amount;
use crate::services::*;
use actix_web::{web, App, HttpServer};
use services::post_task::post_task;

const ADDRESS: &'static str = "127.0.0.1";
const PORT: u16 = 8080;

mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("⚙️ Initializing SQL-Backend...");
    let dbw = state::SQLiteBackend::new().await;
    println!("🌐 Orchestration server starting at http://{ADDRESS}:{PORT}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state::AppState {
                storage: dbw.clone(),
            }))
            .service(post_task)
            .service(put_result)
            .service(get_task_amount)
            .service(put_result)
            .service(readonly_task_amount)
    })
    .bind((ADDRESS, PORT))?
    .run()
    .await
}
