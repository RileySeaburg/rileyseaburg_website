use actix_web::{get, post, web, HttpResponse, Responder};
use rustyroad::database::Database;
use tera::{Context, Tera};
use crate::models::Subscriber;

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

    let mut conn = database.get_conn().unwrap();

    let result = newSubscriber.insert(&mut conn);

    match result {
        Ok(_) => {
            context.insert("message", "Successfully subscribed!");
        },
        Err(e) => {
            context.insert("message", "Failed to subscribe.");
            println!("Failed to subscribe: {}", e);
        }
    }


    let rendered = tmpl.render("pages/newsletter.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}