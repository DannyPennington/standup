use actix_web::{App, HttpServer, HttpRequest, Result, web, HttpResponse, error};
use askama::Template;
use standup::helpers::{role_grouping, determine_config};
use std::env;
use config::ConfigError;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    team_name: String,
    image_path: String,
    grouped_names: Vec<Vec<String>>
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {

    let error_handler = |e: ConfigError| {
        error::ErrorInternalServerError(e.to_string())
    };

    let config = determine_config().map_err(error_handler)?;

    let team_name = config.get_str("team_name").map_err(error_handler)?;

    let image_path = match team_name.as_str() {
        "Neighbourhood Watch" => "/images/NW-2021.png",
        "Team Alpha" => "/images/TA-2021.png",
        _ => "/"
    }.to_owned();

    let mut groups = vec![];
    for &group in &["devs", "design", "admin"] {
        let group_vec: Vec<String> = config.get_array(group).map_err(error_handler)?
            .into_iter().map(|x| x.into_str().unwrap()).collect();
        groups.push(group_vec)
    }

    let grouped_names = role_grouping(groups);
    let html = IndexTemplate {
        team_name,
        image_path,
        grouped_names
    }.render().map_err(|e| {
        error::ErrorInternalServerError(e.to_string())
    })?;

    println!("Serving html...");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "4200".to_owned());
    println!("Server starting on port {} ...", port);
    HttpServer::new(||
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/js", "./assets/js").show_files_listing())
            .service(actix_files::Files::new("/stylesheets", "./assets/stylesheets").show_files_listing())
            .service(actix_files::Files::new("/images", "./assets/images").show_files_listing())
    )
        .bind("0.0.0.0:".to_owned() + &*port)?
        .run()
        .await
}
