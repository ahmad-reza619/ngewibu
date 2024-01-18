mod providers;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use providers::mangabat::{get_manga_chapter, get_manga_detail, get_manga_list};
use providers::otakudesu::get_ongoing;
use serde::Deserialize;

#[get("/api/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/api/ongoing")]
async fn ongoing() -> impl Responder {
    let ongoing = get_ongoing().await;
    HttpResponse::Ok().json(ongoing)
}

#[get("/api/anime/{id}")]
async fn anime(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let anime = providers::otakudesu::get_anime(id).await;
    HttpResponse::Ok().json(anime)
}

#[get("/api/anime/eps/{eps}")]
async fn anime_eps(path: web::Path<String>) -> impl Responder {
    let eps = path.into_inner();
    HttpResponse::Ok().body("OK")
}

#[get("/api/manga-list")]
async fn manga_list() -> impl Responder {
    let list = match get_manga_list().await {
        Ok(list) => list,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    HttpResponse::Ok().json(list)
}

#[derive(Deserialize)]
struct MangaLink {
    link: String,
}

#[post("/api/manga")]
async fn manga_detail(manga: web::Json<MangaLink>) -> impl Responder {
    use providers::mangabat::MangaBatError;
    let detail = match get_manga_detail(manga.link.clone()).await {
        Ok(detail) => detail,
        Err(err) => {
            println!("{}", err);
            match err {
                MangaBatError::RequestError(_) => {
                    return HttpResponse::InternalServerError().body("Cannot get manga")
                }
                MangaBatError::ParsingError(_) => {
                    return HttpResponse::InternalServerError().body("Cannot parse manga")
                }
                MangaBatError::FailedSelector => {
                    return HttpResponse::InternalServerError().body("Failed to select")
                }
            }
        }
    };
    HttpResponse::Ok().json(detail)
}

#[post("/api/manga-chapter")]
async fn manga_read(manga: web::Json<MangaLink>) -> impl Responder {
    let detail = match get_manga_chapter(manga.link.clone()).await {
        Ok(detail) => detail,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    HttpResponse::Ok().json(detail)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(status)
            .service(ongoing)
            .service(anime)
            .service(anime_eps)
            .service(manga_list)
            .service(manga_detail)
            .service(manga_read)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
