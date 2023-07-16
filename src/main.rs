use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{
    web::{self},
    App, HttpServer,
};
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

fn get_secret_key() -> Result<Key, Box<dyn std::error::Error>> {
    let secret_key_from_env = env::var("SECRET_KEY")?;
    if secret_key_from_env.len() < 32 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Secret key must be at least 32 characters",
        )));
    }
    let key = Key::from(secret_key_from_env.as_bytes());
    Ok(key)
}

fn load_encrypted_private_key() -> PKey<Private> {
    let mut file = File::open("/etc/letsencrypt/live/rileyseaburg.com/privkey.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"").unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();

    builder
        .set_certificate_chain_file("/etc/letsencrypt/live/rileyseaburg.com/fullchain.pem")
        .unwrap();

    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let database = web::Data::new(Database::get_database_from_rustyroad_toml().unwrap());

    log::info!("starting HTTPS server at http://localhost:8443");

    HttpServer::new(move || {
        // Load tera templates from the specified directory
        let tera = Tera::new("templates/**/*").unwrap();
        println!("Initializing Actix web application...");

        let secret_key = get_secret_key().unwrap();

        let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
            // disable secure cookie for local testing
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
            .app_data(web::Data::new(tera.clone())) // Updated line
            .service(routes::index::index)
            .service(routes::dashboard::dashboard_route)
            .service(routes::login::login_route)
            .service(routes::login::login_function)
            .service(routes::login::user_logout)
            .service(routes::not_found::not_found)
            .service(Files::new("/static", "./static")) // Add this line
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .unwrap()
    .workers(2)
    .run()
    .await
}
