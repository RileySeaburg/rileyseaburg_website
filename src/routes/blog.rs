use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/blog")]
async fn blog(user: Option<Identity>, tmpl: web::Data<Tera>) -> impl Responder {
    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("username", &user.id().unwrap());
        context.insert("route_name", "authenticated blog");
        let rendered = tmpl
            .render("layouts/authenticated/blog.html.tera", &context)
            .unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        let mut context = Context::new();
        let posts = crate::models::Post::get_all_posts();
        context.insert("route_name", "blog");
        context.insert("posts", &posts);
        let rendered = tmpl.render("pages/blog.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}
