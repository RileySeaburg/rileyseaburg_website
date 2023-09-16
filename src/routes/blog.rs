use crate::models::Post;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/blog")]
async fn blog(user: Option<Identity>, tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    let posts = Post::get_all_posts().await.unwrap();
    context.insert("posts", &posts);
    if let Some(user) = user {
        context.insert("username", &user.id().unwrap());
        context.insert("title", "Blog");
        context.insert("route_name", "blog");
        let rendered = tmpl
            .render("layouts/authenticated/blog.html.tera", &context)
            .unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        context.insert("route_name", "blog");

        let rendered = tmpl.render("pages/blog.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}
