use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/pages")]
async fn pages(tmpl: web::Data<Tera>, user: Option<Identity>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Landing Pages");
    context.insert("username", &user.unwrap().id().unwrap());
    context.insert("route_name", "pages");
    let rendered = tmpl
        .render("layouts/authenticated/pages/pages.html.tera", &context)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/pages/create")]
async fn create(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "create");
    let rendered = tmpl.render("layouts/authenticated/pages/create.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}