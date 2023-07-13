use crate::models;
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
    tmpl: web::Data<Tera>,   // Updated line
    db: web::Data<Database>, // receive Database as web::Data<Database>
) -> Result<HttpResponse, actix_web::Error> {
    // get database data from rustyroad.toml
    form.user_login(tmpl, &db.get_ref().clone()).await
}

#[get("/logout")]
async fn user_logout(
    tmpl: web::Data<Tera>,
    req: HttpRequest, // Add the HttpRequest
) -> Result<HttpResponse, Error> {
    let database = rustyroad::database::Database::get_database_from_rustyroad_toml().unwrap();
    UserLogin::user_logout(tmpl, database, req).await
}
