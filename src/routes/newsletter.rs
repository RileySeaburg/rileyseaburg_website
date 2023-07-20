use actix_web::{get, post, web, HttpResponse, Responder};
use rustyroad::database::Database;
use sqlx::PgPool;
use tera::{Context, Tera};
use crate::models::{NewsletterForm, Subscriber};

#[get("/newsletter")]
async fn newsletter(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "newsletter");
    let rendered = tmpl.render("pages/newsletter.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/newsletter")]
async fn newsletter_post(tmpl: web::Data<Tera>, form: web::Form<NewsletterForm>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "newsletter");

    // extract email from the form
    let email = &form.email.to_string();
    let newSubscriber = Subscriber::new(email);

    // insert the new subscriber into the database
    let database = web::Data::new(Database::get_database_from_rustyroad_toml().unwrap());

    // Create the database URL
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database.username,
        database.password,
        database.host,
        database.port,
        database.name
    );

    let mut conn = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres.");

    let result = newSubscriber.insert(&mut conn);

    match result.await {
        Ok(_) => {
            context.insert("message", "You have successfully subscribed to the newsletter.");
        }
        Err(_) => {
            context.insert("message", "There was an error subscribing to the newsletter.");
        }
    }


    let rendered = tmpl.render("pages/newsletter.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}