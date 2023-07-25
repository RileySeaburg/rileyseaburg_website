use std::collections::HashMap;

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;

use tera::{Context, Tera, Result, Value, to_value};
use crate::models::Post;
extern crate markdown;
use markdown::to_html;


pub fn markdown_filter(value: &JsonValue, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("markdown", "value", String, value);
    Ok(to_value(to_html(s.as_str())).unwrap())
}


#[get("/post/{slug}")]
async fn get_post(user: Option<Identity>, tmpl: web::Data<Tera>, slug: web::Path<String>) -> impl Responder {
    
    let mut context = Context::new();
    let slug = slug.into_inner();
    let post = match Post::get_post_slug(slug.clone()).await {
        Ok(post) => post,
        Err(_e) => return HttpResponse::NotFound().body("Post not found"),
    };

    context.insert("post", &post);

    // Convert the post body from Markdown to HTML.
    let html: String = to_html(&post.content.unwrap_or("".to_string()));

    // Insert the HTML into the context.
    context.insert("html", &html);

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


#[derive(Serialize)]
struct ResponseMessage {
    status: String,
    message: String,
}

#[post("/posts/{slug}/update")]
async fn update_post(post: web::Json<Post>, slug: web::Path<String>) -> impl Responder {
    let slug = slug.into_inner();
    println!("Updating post {}", slug);
    println!("Post title: {:?}", post.title);
    println!("Post body: {:?}", post.content);
    println!("Post author: {:?}", post.author);
    // Update the post.
    let updated_post = Post::update_post_slug(slug.clone(), post.0.clone()).await;

    // Prepare a response message.
    let response_message = match updated_post {
        Ok(_post) => ResponseMessage {
            status: "success".to_string(),
            message: format!("Post {} updated successfully", slug),
        },
        Err(_e) => ResponseMessage {
            status: "error".to_string(),
            message: format!("Unable to update post {}", slug),
        },
    };

    // Return the response message as JSON.
    HttpResponse::Ok().json(response_message)
}


#[post("/post/create")]
async fn create_post(post: web::Json<Post>) -> impl Responder {
    println!("Creating new post");
    println!("Post title: {:?}", post.title);
    println!("Post body: {:?}", post.content);
    println!("Post author: {:?}", post.author);
    // Create the post.
    let created_post = Post::create_post(post.0.clone()).await;

    // Prepare a response message.
    let response_message = match created_post {
        Ok(_post) => ResponseMessage {
            status: "success".to_string(),
            message: format!("Post {:?} created successfully", post.title),
        },
        Err(_e) => ResponseMessage {
            status: "error".to_string(),
            message: format!("Unable to create post {:?}", post.title),
        },
    };

    // Return the response message as JSON.
    HttpResponse::Ok().json(response_message)
}

#[get("/post/new")]
async fn new_post(user: Option<Identity>, tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    if let Some(user) = user {
        context.insert("username", &user.id().unwrap());
        context.insert("title", "New Post");
        context.insert("route_name", "new_post");
        let rendered = tmpl.render("layouts/authenticated/posts/new_post.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        let title = format!("Not Logged in");
        let message = format!("You must be logged in to create a new post");
        context.insert("title", &title);
        context.insert("message", &message);
        let rendered = tmpl.render("pages/404.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}