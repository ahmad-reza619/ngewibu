mod providers;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use providers::otakudesu::get_ongoing;

#[get("/api/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}


#[get("/api/ongoing")]
async fn ongoing() -> impl Responder {
    let ongoing = get_ongoing().await;
    HttpResponse::Ok().json(ongoing)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(status).service(ongoing)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
