use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse};
use askama::Template;
use standup::helpers::{role_grouping, determine_config};
use std::env;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    grouped_names: Vec<Vec<&'a str>>
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {

    let config = determine_config();
    /*
    let mut devs: Vec<Value> = config.get_array("devs").unwrap();
    let mut design: Vec<Value> = config.get_array("design").unwrap();
    let mut admin: Vec<Value> = config.get_array("admin").unwrap();
    let mut new_devs: Vec<String> = vec![];
    let mut new_design: Vec<String> = vec![];
    let mut new_admin: Vec<String> = vec![];

    for item in &mut devs {
        new_devs.push(item.into_str().unwrap());
    }

    for item in &mut design {
        new_design.push(item.into_str().unwrap());
    }

    for item in &mut admin {
        new_admin.push(item.into_str().unwrap());
    }
     */

    let devs: Vec<&str> = config.get_array("devs").unwrap().iter().map(|x| x.into_str().unwrap().as_str()).collect::<Vec<&str>>();
    let design: Vec<&str> = config.get_array("design").unwrap().iter().map(|x| x.into_str().unwrap().as_str()).collect();
    let admin: Vec<&str> = config.get_array("admin").unwrap().iter().map(|x| x.into_str().unwrap().as_str()).collect();

    let grouped_people = role_grouping(vec![devs, design, admin]);
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
