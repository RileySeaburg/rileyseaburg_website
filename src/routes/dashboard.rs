use actix_web::{get, web, HttpResponse, HttpRequest, Error, http::StatusCode};
use rustyroad::database::Database;
use sqlx::PgPool;
use tera::{Context, Tera};
use crate::{models, controllers::get_user_from_session_token};
use models::user::UserLogin;

#[get("/dashboard")]
async fn dashboard_route(
    tmpl: web::Data<Tera>,
    database: web::Data<Database>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Create the database URL
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database.username, database.password, database.host, database.port, database.name
    );

    // Create the database connection pool
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres.");

    // Retrieve the session token from the cookies
    let session_token = req
        .cookie("session_token")
        .map(|cookie| cookie.value().to_string());

    if let Some(token) = session_token {
        // Get the user from the session token
        match get_user_from_session_token(&token, &db_pool).await {
            Ok(user) => {
                let mut context = Context::new();
                context.insert("route_name", "dashboard");
                context.insert("username", &user.username);
                match tmpl.render("pages/dashboard.html.tera", &context) {
                    Ok(rendered) => Ok(HttpResponse::Ok().body(rendered)),
                    Err(err) => {
                        // Log the error, so you know what went wrong.
                        eprintln!("Template rendering error: {}", err);
                        Ok(HttpResponse::InternalServerError().body(format!("Server error: {}", err)))
                    }
                }
            }
            Err(_) => {
                // If session token is not valid, redirect to 404 page.
                Ok(HttpResponse::SeeOther()
                    .header("Location", "/404")
                    .finish())
            }
        }
    } else {
        // If no session token found, redirect to 404 page.
        Ok(HttpResponse::SeeOther()
            .header("Location", "/404")
            .finish())
    }
}
