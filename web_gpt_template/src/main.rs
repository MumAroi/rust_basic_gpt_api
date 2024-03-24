use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::time::{SystemTime, UNIX_EPOCH};

async fn get_current_time() -> impl Responder {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    HttpResponse::Ok().body(format!("Current time: {}", current_time))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/time", web::get().to(get_current_time))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}