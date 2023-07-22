use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use dotenv::dotenv;
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};
use std::env;
use std::fs::File;
use std::io::Read;

use actix_identity::IdentityMiddleware;
use rustyroad::database::Database;
use tera::Tera;
mod controllers;
mod models;
mod routes;

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

    // if the environment is production, use the production path
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
    // Load environment variables from .env
    dotenv().ok();

    let enviornment = get_value_from_env_with_key("ENV".to_string()).unwrap();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();

    let uri = if enviornment == "production" {
        "0.0.0.0:443"
    } else {
        "localhost:8080"
    };

    if enviornment == "production" {
        builder
            .set_certificate_chain_file("/etc/letsencrypt/live/rileyseaburg.com/fullchain.pem")
            .unwrap();
    } else {
        builder.set_certificate_chain_file("cert.pem").unwrap();
    }
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let database = web::Data::new(Database::get_database_from_rustyroad_toml().unwrap());

    log::info!("starting HTTPS server at https://rileyseaburg.com/");

    HttpServer::new(move || {
        // Load tera templates from the specified directory
        let tera = Tera::new("templates/**/*").unwrap();
        println!("Initializing Actix web application...");

        let secret_key = Key::from(
            get_value_from_env_with_key("SECRET_KEY".to_string())
                .unwrap()
                .as_bytes(),
        );

        let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
            // disable secure cookie for local testing
            .cookie_secure(false)
            .build();

        App::new()
            // .wrap(
            //     actix_web::middleware::Logger::default()
            //         .exclude("/static")
            //         .exclude("/favicon.ico"),
            // )
            .wrap(IdentityMiddleware::default())
            .app_data(database.clone())
            .wrap(session_mw)
            .app_data(web::Data::new(tera.clone())) // Updated line
            .service(routes::index::index)
            .service(routes::dashboard::dashboard_route)
            .service(routes::login::login_route)
            .service(routes::login::login_function)
            .service(routes::login::user_logout)
            .service(routes::not_found::not_found)
            .service(routes::newsletter::newsletter)
            .service(routes::newsletter::newsletter_post)
            .service(routes::blog::blog)
            .service(Files::new("/static", "./static")) // Add this line
    })
    .bind_openssl(uri, builder)?
    .workers(2)
    .run()
    .await
}
