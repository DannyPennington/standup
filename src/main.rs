use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    grouped_names: Vec<Vec<&'a str>>
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let people = vec!["Danny", "Jason", "Jeremy", "Sarah", "Brian", "Baz", "Jen", "Kate", "Glorbo"];
    let mut grouped_people: Vec<Vec<&str>> = Vec::new();
    let mut buffer = Vec::new();
    for index in 0..people.len() {
        buffer.push(people[index]);
        if buffer.len() == 4 || index == people.len() - 1 {
            grouped_people.push(buffer.clone());
            buffer.clear();
        }
    }
    let html = IndexTemplate {
        grouped_names: grouped_people
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
            .service(actix_files::Files::new("/js", "./assets/js").show_files_listing())
            .service(actix_files::Files::new("/stylesheets", "./assets/stylesheets").show_files_listing())
    )
        .bind("0.0.0.0:4200")?
        .run()
        .await
}
