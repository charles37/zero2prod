
use actix_web::{web, App, HttpServer, HttpResponse};

//async fn greet(req: HttpRequest) -> impl Responder {
//    let name = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", &name)
//}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()

}

pub async fn run () -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}


