use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};

use tera::{Context, Tera};

#[get("/dashboard")]
pub async fn dashboard_route(
    tmpl: web::Data<Tera>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("username", &user.id().unwrap());
        context.insert("title", "Dashboard");
        context.insert("route_name", "dashboard");
        let rendered = tmpl.render("layouts/authenticated.html.tera", &context).unwrap();
        Ok(HttpResponse::Ok().body(rendered))
    } else {
        let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view this page.");
        Ok(HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish())
    }
}
