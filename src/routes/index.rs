use actix_web::{get, web, HttpResponse};
use tera::{Context, Tera};

#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("route_name", "index");
    let rendered = tmpl.render("pages/index.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
