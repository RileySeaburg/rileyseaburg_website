use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};
use crate::models::Post;

#[get("/posts/{slug}")]
async fn get_post(user: Option<Identity>, tmpl: web::Data<Tera>, slug: web::Path<String>) -> impl Responder {
    let mut context = Context::new();
    let slug = slug.into_inner();
    let post = match Post::get_post_slug(slug.clone()).await {
        Ok(post) => post,
        Err(_e) => return HttpResponse::NotFound().body("Post not found"),
    };

    context.insert("post", &post);

    // Create URLs for the 'edit' and 'delete' actions.
    let edit_url = format!("/post/{}/edit", slug.clone());
    let delete_url = format!("/post/{}/delete", slug);
    context.insert("edit_url", &edit_url);
    context.insert("delete_url", &delete_url);

    if let Some(user) = user {
        context.insert("username", &user.id().unwrap());
        context.insert("title", "Blog");
        context.insert("route_name", "blog");
        let rendered = tmpl.render("layouts/authenticated/posts/post.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        context.insert("route_name", "blog");
        let rendered = tmpl.render("pages/post.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}


#[get("/post/{slug}/edit")]
async fn edit_post(user: Option<Identity>, tmpl: web::Data<Tera>, slug: web::Path<String>) -> impl Responder {
    let mut context = Context::new();
    let slug = slug.into_inner();
    let post = match Post::get_post_slug(slug.clone()).await {
        Ok(post) => post,
        Err(_e) => return HttpResponse::NotFound().body("Post not found"),
    };

    context.insert("post", &post);

 if let Some(user) = user {
     // Create the URL for the 'update' action.
     let update_url = format!("/posts/{}/update", slug);
     let delete_url = format!("/posts/{}/delete", slug);
     context.insert("update_url", &update_url);
     context.insert("delete_url", &delete_url);


     context.insert("username", &user.id().unwrap());
     context.insert("title", "Edit Post");
     context.insert("route_name", "edit_post");
     let rendered = tmpl.render("layouts/authenticated/posts/edit_post.html.tera", &context).unwrap();

     HttpResponse::Ok().body(rendered)
    } else {
        let title = format!("Not Logged in");
     let message = format!("You must be logged in to edit this post");
        context.insert("title", &title);
        context.insert("message", &message);
        let rendered = tmpl.render("pages/404.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}




#[get("/posts/{slug}/json")]
async fn get_post_return_post_as_json(slug: web::Path<String>) -> impl Responder {
    let slug = slug.into_inner();
    let post = match Post::get_post_slug(slug.clone()).await {
        Ok(post) => post,
        Err(_e) => return HttpResponse::NotFound().body("Post not found"),
    };

        HttpResponse::Ok().json(post)
    }



use actix_web::{post};

#[post("/posts/{slug}/update")]
async fn update_post(post: web::Json<Post>, slug: web::Path<String>) -> impl Responder {
    let slug = slug.into_inner();

    // Update the post.
    let updated_post = Post::update_post_slug(slug.clone(), post.0.clone()).await;

    // return JSON response with the sucess or failure of the update.
    let message = match updated_post {
        Ok(_post) => format!("Post {} updated successfully", slug),
        Err(_e) => format!("Unable to update post {}", slug),
    };

    HttpResponse::Ok().json(message)
}
