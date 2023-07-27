use actix_web::{get, App, HttpServer};
use actix_web::Responder;
use actix_web::HttpResponse;

#[get("/")]
async fn index() -> &'static str {
    "This is the index page"
}

#[get("/html")]
async fn html() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
        r#"<!DOCTYPE html>
            <html>
                <head>
                    <title>This is an HTML page</title>
                </head>
            <body>
                <h1>This is an HTML page</h1>
            </body>
        </html>"#
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(html))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
