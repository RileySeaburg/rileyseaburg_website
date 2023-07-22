use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/post/{post_id}")]
async fn get_post(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("route_name", "post");
    let rendered = tmpl.render("pages/post.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
