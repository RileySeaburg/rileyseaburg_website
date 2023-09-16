use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

use crate::models::user;

#[get("/webinar")]
async fn webinar(user: Option<Identity>, tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "webinar");

    if let Some(user) = user {
        context.insert("user", &user.id().unwrap());
        context.insert("title", "Webinar");
        context.insert("route_name", "webinar");
        let rendered = tmpl
            .render("layouts/authenticated/webinars/webinar.html.tera", &context)
            .unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        context.insert("title", "Webinar");
        context.insert("route_name", "webinar");
        let rendered = tmpl
            .render("pages/webinars/webinar.html.tera", &context)
            .unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/webinar/{id}")]
async fn get_webinar_by_id(
    user: Option<Identity>,
    tmpl: web::Data<Tera>,
    id: web::Path<String>,
) -> impl Responder {
    let mut context = Context::new();
    let route_name = format!("webinar {}", id);
    context.insert("route_name", &route_name);
    context.insert("title", "Webinar");
    context.insert("id", &id.to_string());

    if let Some(user) = user {
        context.insert("username", &user.id().unwrap());
        let rendered = tmpl
            .render(
                "layouts/authenticated/webinars/id.webinar.html.tera",
                &context,
            )
            .unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        let rendered = tmpl
            .render("pages/webinars/id.webinar.html.tera", &context)
            .unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/webinar/{id}/live")]
async fn webinar_live(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "webinar_live");
    let rendered = tmpl
        .render("pages/webinars/live.id.webinar.html.tera", &context)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}
