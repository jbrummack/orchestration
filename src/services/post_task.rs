use crate::state::models::tasks::PostTask;
use crate::state::*;

use actix_web::{post, web};
/*
curl -d '{"link":"openfoodfacts","usr_id":0}' -H "Content-Type: application/json" -X POST http://localhost:8080/task
*/
#[post("/task")]
pub async fn post_task(post: web::Json<PostTask>, data: web::Data<AppState>) -> String {
    //println!("{:?}", &post);
    data.storage.add_task(post.task.clone()).await
}
