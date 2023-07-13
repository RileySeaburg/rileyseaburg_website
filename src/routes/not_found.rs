use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/404")]
async fn not_found(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "404 Page Not Found");
    context.insert(
        "description",
        "The page you are looking for does not exist.",
    );
    context.insert("route_name", "not_found");
    let rendered = tmpl.render("pages/404.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
