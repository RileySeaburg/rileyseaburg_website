use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/blog")]
async fn blog(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "blog");
    let rendered = tmpl.render("pages/blog.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}