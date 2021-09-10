use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    names: Vec<&'a str>
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let people: Vec<&str> = vec!["Danny", "Jason", "Jeremy"];
    let html = IndexTemplate {
        names: people
    }.render().unwrap();
    println!("Serving html...");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting...");
    HttpServer::new(||
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/js", "./js").show_files_listing())
            .service(actix_files::Files::new("/stylesheets", "./stylesheets").show_files_listing())
    )
        .bind("127.0.0.1:4200")?
        .run()
        .await
}
