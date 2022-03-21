use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i64>,
}

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("healthy")
}

async fn handlercounter(data: web::Data<AppState>) -> HttpResponse {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Request Count: {counter}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(healthcheck))
            .route("/counter", web::get().to(handlercounter))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}