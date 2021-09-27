use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse};
use askama::Template;
use standup::helpers::{role_grouping};
use std::env;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    grouped_names: Vec<Vec<&'a str>>
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    // Add people to the relevant list here to add them to the board
    let devs = vec!["Danny", "Ewan", "Dan", "Sarthak", "Don", "Sean", "Abdi"];
    let design = vec!["Helen", "Sam", "Shail", "Jacob"];
    let admin = vec!["Alok", "Maria"];

    let people = vec![devs, design, admin];
    let grouped_people = role_grouping(people);
    let html = IndexTemplate {
        grouped_names: grouped_people
    }.render().unwrap();

    println!("Serving html...");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or("4200".to_owned());
    println!("Server starting on port {} ...", port);
    HttpServer::new(||
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/js", "./assets/js").show_files_listing())
            .service(actix_files::Files::new("/stylesheets", "./assets/stylesheets").show_files_listing())
    )
        .bind("0.0.0.0:".to_owned() + &*port)?
        .run()
        .await
}
