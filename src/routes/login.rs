use crate::models;
use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use models::user::UserLogin;
use rustyroad::database::Database;
use tera::{Context, Tera};

#[get("/login")]
async fn login_route(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("route_name", "login");
    let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/login")]
async fn login_function(
    form: web::Form<UserLogin>,
    tmpl: web::Data<Tera>,
    db: web::Data<Database>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    form.user_login(tmpl, &db.get_ref().clone(), req).await
}

#[get("/logout")]
async fn user_logout(tmpl: web::Data<Tera>, user: Identity) -> Result<HttpResponse, Error> {
    UserLogin::user_logout(tmpl, user).await
}
