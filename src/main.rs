use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse};
use actix_files::NamedFile;
use std::path::PathBuf;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: Vec<&'a str>
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let people: Vec<&str> = vec!["Danny", "Ewan", "Dan"];
    let html = IndexTemplate {
        name: people
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/js", "./js").show_files_listing())
    )
        .bind("127.0.0.1:4200")?
        .run()
        .await
}
