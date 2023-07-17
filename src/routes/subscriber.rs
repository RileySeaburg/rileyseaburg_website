use actix_web::{get, web, HttpResponse};
use tera::{Context, Tera};

#[get("/subscriber")]
async fn subscriber(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "subscriber");
    let rendered = tmpl.render("pages/subscriber.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}