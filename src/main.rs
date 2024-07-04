mod utils;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::error::ErrorInternalServerError;
use tera::{Tera, Context};
use utils::get_slides;
use std::path::Path;
use actix_files;
use std::fs;

async fn index(template: web::Data<Tera>) -> impl Responder {
    let slide_names = get_slides("slides");

    let mut context = Context::new();
    context.insert("slides", &slide_names);

    let rendered = template.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn slide(template: web::Data<Tera>, path: web::Path<String>) -> Result<HttpResponse> {
    let slide_path = path.into_inner();
    let file_path = format!("slides/{}.html", slide_path);

    if Path::new(&file_path).exists() {        
        let slide_content = fs::read_to_string(&file_path)
            .map_err(|_| ErrorInternalServerError("Could not read file"))?;

        let mut context = Context::new();
        context.insert("slide_name", &slide_path);
        context.insert("slide_content", &slide_content);

        let rendered = template.render("slide.html", &context).unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    } else {
        let rendered = template.render("404.html", &Context::new()).unwrap();
        Ok(HttpResponse::NotFound().content_type("text/html").body(rendered))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(actix_files::Files::new("/assets", "assets").show_files_listing())
            .route("/", web::get().to(index))
            .route("/{slide_path:.*}", web::get().to(slide))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
