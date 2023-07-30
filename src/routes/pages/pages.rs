use actix_identity::Identity;
use actix_web::{dev::Response, get, post, web, HttpResponse, Responder};
use rustyroad::{
    database::{PostgresTypes, PostgresTypesIter},
    RustyRoad,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tera::{Context, Tera};

use crate::models::{Page, PageField, PageFieldIter};

#[get("/pages")]
async fn pages(tmpl: web::Data<Tera>, user: Option<Identity>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Landing Pages");
    context.insert("username", &user.unwrap().id().unwrap());
    context.insert("route_name", "pages");
    let rendered = tmpl
        .render("layouts/authenticated/pages/pages.html.tera", &context)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/pages/create")]
async fn create(tmpl: web::Data<Tera>, user: Option<Identity>) -> impl Responder {
    let mut context = Context::new();
    context.insert("username", &user.unwrap().id().unwrap());
    context.insert("title", "Create Landing Page");
    context.insert("route_name", "create");

    let mut types = vec![];

    for t in PostgresTypes::iter() {
        types.push(t);
    }

    context.insert("types", &types);
    let rendered = tmpl
        .render("layouts/authenticated/pages/create.html.tera", &context)
        .unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/pages/create")]
async fn create_page(page: web::Json<Page>) -> impl Responder {
    println!("Creating new page");
    println!("Page title: {:?}", page.title);
    println!("Page body: {:?}", page.content);
    println!("Page author: {:?}", page.author);
    // Create the page.

    // Create a dummy instance of the Page struct
    Page::create_page(page.clone()).unwrap();
    
    for field in PageField::iter() {
        println!("Page field: {:?}", field);
        println!("Page field value: {:?}", page.get_field(&field));
    }

    let response_message = ResponseMessage {
        status: "success".to_string(),
        message: format!("Page {:?} created successfully", page.title),
    };

    // Return the response message as JSON.
    HttpResponse::Ok().json(response_message)
}

#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    status: String,
    message: String,
}
