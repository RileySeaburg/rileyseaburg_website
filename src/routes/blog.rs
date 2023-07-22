use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};
use crate::models::Post;


#[get("/blog")]
async fn blog(user: Option<Identity>, tmpl: web::Data<Tera>) -> impl Responder {
    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("username", &user.id().unwrap());
        context.insert("route_name", "authenticated blog");
        let rendered = tmpl
            .render("layouts/authenticated/blog.html.tera", &context)
            .unwrap();
        return HttpResponse::Ok().body(rendered);
    }

    let mut context = Context::new();
    context.insert("route_name", "blog");
    let posts = Post::get_all_posts().await.unwrap();
    context.insert("posts", &posts);
    let rendered = tmpl.render("pages/blog.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)

}
