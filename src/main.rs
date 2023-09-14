use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use chrono::{NaiveDateTime, TimeZone, Utc};
use dotenv::dotenv;
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};
use std::env;
use std::fs::File;
use std::io::Read;
#[macro_use]
extern crate tera;
use tera::{to_value, Result as TeraResult, Tera, Value};

use actix_identity::IdentityMiddleware;
use rustyroad::database::Database;
mod controllers;
mod models;
mod routes;

use std::collections::HashMap;

use crate::routes::markdown_filter;

fn date_time_format(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let date = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Invalid value format"))?;
    let format_arg = args
        .get("format")
        .ok_or_else(|| tera::Error::msg("Missing format argument"))?;
    let format = format_arg
        .as_str()
        .ok_or_else(|| tera::Error::msg("Format argument is not a string"))?;

    let date_time = Utc
        .datetime_from_str(date, "%Y-%m-%dT%H:%M:%S%.fZ")
        .map_err(|_| tera::Error::msg("Error parsing date time"))?;
    let formatted_date = date_time.format(format).to_string();

    to_value(&formatted_date)
        .map_err(|_| tera::Error::msg("Error converting formatted date to value"))
}

fn get_value_from_env_with_key(key: String) -> Result<String, Box<dyn std::error::Error>> {
    let key_value_from_env = env::var(key)?;
    if key_value_from_env == "SECRET_KEY" {
        if key_value_from_env.len() < 32 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Secret key must be at least 32 characters",
            )));
        }
        return Ok(key_value_from_env);
    }

    Ok(key_value_from_env)
}

fn load_encrypted_private_key() -> PKey<Private> {
    let path_production: String = "/etc/letsencrypt/live/rileyseaburg.com/privkey.pem".to_string();
    let path_development: String = "key.pem".to_string();

    let key: String = if env::var("ENV").unwrap().to_string().to_ascii_lowercase() == "production" {
        path_production.clone()
    } else {
        path_development
    };

    let mut file = File::open(key).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"").unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let enviornment = get_value_from_env_with_key("ENV".to_string()).unwrap();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();

    let uri = if enviornment == "production" {
        "0.0.0.0:80"
    } else {
        "localhost:8080"
    };

    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let database = web::Data::new(Database::get_database_from_rustyroad_toml().unwrap());

    log::info!("starting HTTPS server at https://rileyseaburg.com/");

    HttpServer::new(move || {
        let mut tera = Tera::new("templates/**/*").unwrap();
        tera.register_filter("date_time_format", date_time_format);
        tera.register_filter("markdown", markdown_filter);
        println!("Initializing Actix web application...");

        let secret_key = Key::from(
            get_value_from_env_with_key("SECRET_KEY".to_string())
                .unwrap()
                .as_bytes(),
        );

        let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
            .cookie_secure(false)
            .build();

        App::new()
            .wrap(
                actix_web::middleware::Logger::default()
                    .exclude("/static")
                    .exclude("/favicon.ico"),
            )
            .wrap(IdentityMiddleware::default())
            .app_data(database.clone())
            .wrap(session_mw)
            .app_data(web::Data::new(tera.clone()))
            .service(routes::index::index)
            .service(routes::dashboard::dashboard_route)
            .service(routes::login::login_route)
            .service(routes::login::login_function)
            .service(routes::login::user_logout)
            .service(routes::not_found::not_found)
            .service(routes::newsletter::newsletter)
            .service(routes::newsletter::newsletter_post)
            .service(routes::blog::blog)
            .service(routes::post::get_post)
            .service(routes::post::update_post)
            .service(routes::post::edit_post)
            .service(routes::post::get_post_return_post_as_json)
            .service(routes::post::create_post)
            .service(routes::post::new_post)
            .service(routes::post::delete_post)
            .service(routes::pages::pages)
            .service(routes::pages::create)
            .service(routes::pages::create_page)
            .service(Files::new("/", "./static"))
    })
    .bind(uri)
    .expect("yo")
    .workers(2)
    .run()
    .await
}
